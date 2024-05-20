mod ap_thread;
mod archipelago_rs;
mod cli;
mod data;
mod idmap;
mod input_thread;
mod logic;
mod state;

use ap_thread::ap_thread;
use archipelago_rs::{client::ArchipelagoClient, protocol::Connected};
use clap::Parser;
use cli::{find_location, print, DisplayUpdate};
use console::Term;
use data::Item;
use input_thread::input_thread;
use serde::Deserialize;
use serde_json::{from_reader, to_writer};
use state::{LocationsSerializable, State};
use std::{
    collections::{HashMap, VecDeque},
    fs::{create_dir, create_dir_all, remove_file, File},
    io::{self, stdout, BufRead, Write},
    path::Path,
    process::exit,
    sync::mpsc::channel,
    thread::spawn,
};
use tokio::runtime::Builder;

use crate::{archipelago_rs::protocol::ClientStatus, data::Location};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[allow(clippy::option_option)]
    #[arg(short, long)]
    server: Option<Option<String>>,
    #[allow(clippy::option_option)]
    #[arg(short, long)]
    port: Option<Option<String>>,
    #[allow(clippy::option_option)]
    #[arg(short = 'S', long)]
    slot: Option<Option<String>>,
    #[allow(clippy::option_option)]
    #[arg(short = 'P', long)]
    password: Option<Option<String>>,
}

#[derive(Debug, Clone, Deserialize)]
struct SlotData {
    pub required_scions: u32,
    pub required_villains: u32,
    pub required_variants: u32,
    pub villain_difficulty_points: [u32; 4],
    pub locations_per: [u8; 6],
}

fn main() {
    let (client_sender, receiver) = channel::<DisplayUpdate>();

    let runtime = Builder::new_multi_thread().enable_io().build().unwrap();
    let seed_name: String;

    let (client, mut state, connected, slot) = match runtime.block_on(async { connect(true).await }) {
        Ok((mut client, connected, slot)) => {
            seed_name = client.room_info.seed_name.clone();
            let persistent = get_persistent(&client.room_info.seed_name);
            let mut state = State::new(&client.data_package);
            runtime.block_on(async { state.sync(&connected, client.sync().await.expect("Failed to sync items"), persistent) });
            (client, state, connected, slot)
        }
        Err(_) => match runtime.block_on(async { connect(false).await }) {
            Ok((mut client, connected, slot)) => {
                seed_name = client.room_info.seed_name.clone();
                let persistent = get_persistent(&client.room_info.seed_name);
                let mut state = State::new(&client.data_package);
                runtime.block_on(async { state.sync(&connected, client.sync().await.expect("Failed to sync items"), persistent) });
                (client, state, connected, slot)
            }
            Err(err) => {
                println!("Failed to connect to Archipelago server with reason: {err}");
                exit(1);
            }
        },
    };

    println!("Connected!");
    let (mut ap_sender, ap_receiver) = client.split();

    let cloned_sender = client_sender.clone();
    drop(spawn(|| input_thread(cloned_sender)));
    runtime.spawn(ap_thread(state.idmap.clone(), client_sender.clone(), ap_receiver, connected, slot));

    let term = Term::stdout();
    let _ = term.hide_cursor();
    let mut filter = String::new();
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut msg_buffer = VecDeque::with_capacity(10);
    let mut multi_send = false;

    loop {
        let send_victory = print(&term, &state, &filter, cursor_x, cursor_y, &msg_buffer, multi_send);
        if send_victory {
            runtime.block_on(async {
                let res = ap_sender.status_update(ClientStatus::Goal).await;
                if res.is_err() {
                    println!("Failed to send goal");
                } else {
                    state.checked_locations.victory = true
                }
            })
        }
        if let Ok(update) = receiver.recv() {
            match update {
                DisplayUpdate::Msg(msg) => {
                    if msg_buffer.len() > 9 {
                        msg_buffer.pop_front();
                    }
                    msg_buffer.push_back(msg);
                }
                DisplayUpdate::Items(items) => {
                    for item in items {
                        match item {
                            Item::Hero(hero) => state.items.set_hero(hero),
                            Item::Variant(variant) => state.items.set_hero_variant(variant),
                            Item::Villain(villain) => state.items.set_villain(villain),
                            Item::TeamVillain(teamvillain) => state.items.set_team_villain(teamvillain),
                            Item::Environment(environment) => state.items.set_environment(environment),
                            Item::Scion => state.items.scions += 1,
                            Item::Filler => (),
                        }
                    }
                }
                DisplayUpdate::Filter(new_filter) => filter = new_filter,
                DisplayUpdate::CursorLeft => cursor_x = cursor_x.saturating_sub(1),
                DisplayUpdate::CursorRight => {
                    if cursor_x < 5 {
                        cursor_x += 1;
                    }
                }
                DisplayUpdate::CursorUp => cursor_y = cursor_y.saturating_sub(1),
                DisplayUpdate::CursorDown => cursor_y += 1,
                DisplayUpdate::CursorHome => (cursor_x, cursor_y) = (0, 0),
                DisplayUpdate::Select => multi_send = !multi_send,
                DisplayUpdate::Send => {
                    let mut sent = 0;

                    if let Some(location) = find_location(&state, &filter, cursor_x, cursor_y) {
                        runtime.block_on(async {
                            let mut location_ids = vec![];

                            let locations = if multi_send { resolve_multi_send(location) } else { Box::new([location]) };
                            for location in locations.iter() {
                                for n in 0..=state.slot_data.locations_per[match location {
                                    Location::Variant(_) => 5,
                                    Location::Villain((_, d)) | Location::TeamVillain((_, d)) => *d as usize,
                                    Location::Environment(_) => 4,
                                }] {
                                    if n > 0 {
                                        if let Some(id) = state.idmap.locations_to_id.get(&(*location, n)) {
                                            location_ids.push(*id)
                                        }
                                    }
                                }
                            }

                            let res = ap_sender.location_checks(location_ids).await;
                            if res.is_ok() {
                                for location in locations.iter() {
                                    match *location {
                                        Location::Variant(v) => state.checked_locations.mark_variant(v),
                                        Location::Villain((v, d)) => state.checked_locations.mark_villain(v, d),
                                        Location::TeamVillain((v, d)) => state.checked_locations.mark_team_villain(v, d),
                                        Location::Environment(e) => state.checked_locations.mark_environment(e),
                                    }
                                }
                                sent = locations.len();
                            }
                        })
                    }
                    if cursor_y > 0 {
                        if cursor_y < sent {
                            cursor_y += 1;
                            cursor_x -= 1;
                        }
                        cursor_y -= sent;
                    } else {
                        cursor_x -= 1;
                    }
                }
                DisplayUpdate::Exit => {
                    let _ = term.show_cursor();
                    let _ = term.clear_screen();
                    set_persistent(&seed_name, state.checked_locations.into());
                    exit(0);
                }
            }
        } else {
            let _ = term.show_cursor();
            let _ = term.clear_screen();
            set_persistent(&seed_name, state.checked_locations.into());
            exit(1);
        }
    }
}

/// # Errors
///
/// Will return `Err` if a connection to the server with data package could not be established
///
///
/// # Panics
///
/// Will panic on data package cache errors
pub async fn connect(secure: bool) -> anyhow::Result<(ArchipelagoClient, Connected, String)> {
    let args = Args::parse();

    let mut server = if let Some(server) = args.server {
        server.unwrap_or("archipelago.gg".into())
    } else {
        prompt("Server? (default: archipelago.gg)").unwrap_or("archipelago.gg".into())
    };
    let mut port = if let Some(port) = args.port {
        port.unwrap_or("38281".into())
    } else {
        prompt("Port? (default: 38281)").unwrap_or("38281".into())
    };
    let mut slot = if let Some(slot) = args.slot {
        slot.unwrap_or("Player".into())
    } else {
        prompt("Slot? (default: Player)").unwrap_or("Player".into())
    };
    let password = if let Some(password) = args.password { password } else { prompt("Password?") };

    if server.is_empty() {
        server = String::from("archipelago.gg");
    }

    if port.is_empty() {
        port = String::from("38281");
    }

    if slot.is_empty() {
        slot = String::from("Player");
    }

    let url = if secure { format!("wss://{server}:{port}") } else { format!("ws://{server}:{port}") };

    let mut client = ArchipelagoClient::new(&url).await?;

    create_dir_all("./datapackage").expect("Failed to create datapackage cache");

    let mut missing_games = vec![];
    let mut cached = HashMap::new();
    for (game, checksum) in &client.room_info.datapackage_checksums {
        if let Ok(reader) = File::open(Path::new("./datapackage").join(game).join(checksum)) {
            if let Ok(data) = from_reader(reader) {
                cached.insert(game.to_owned(), data);
            } else {
                missing_games.push(game.to_owned());
            }
        } else {
            missing_games.push(game.to_owned());
        }
    }

    let mut lock = stdout().lock();
    for game in &missing_games {
        let _ = writeln!(lock, "Missing datapackage for {game}...");
    }

    client.get_data_package(Some(missing_games)).await?;

    for (game, data) in &client.data_package.games {
        if let Some(checksum) = client.room_info.datapackage_checksums.get(game) {
            let _ = create_dir(Path::new("./datapackage").join(game));
            if let Ok(writer) = File::create(Path::new("./datapackage").join(game).join(checksum)) {
                let res = to_writer(writer, data);
                if res.is_err() {
                    remove_file(Path::new("./datapackage").join(game).join(checksum)).unwrap_or_else(|_| panic!("Failed to clean failed write for {game}-{checksum}. Data may be corrupt"));
                }
            }
        }
    }

    client.data_package.games.extend(cached);

    let connected = if let Some(pass) = password {
        client.connect("Sentinels of the Multiverse", &slot, Some(&*pass), Some(7), vec!["AP".to_string()]).await?
    } else {
        client.connect("Sentinels of the Multiverse", &slot, None, Some(7), vec!["AP".to_string()]).await?
    };

    Ok((client, connected, slot))
}

fn prompt(text: &str) -> Option<String> {
    print!("{text} > ");
    let _ = io::stdout().flush();

    let input = String::from(io::stdin().lock().lines().next().unwrap().unwrap().as_str());

    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

pub fn get_persistent(seed_name: &str) -> LocationsSerializable {
    create_dir_all("./persistent").expect("Failed to create persistent storage");
    if let Ok(reader) = File::open(Path::new("./persistent").join(seed_name)) {
        if let Ok(data) = from_reader(reader) {
            data
        } else {
            LocationsSerializable::new()
        }
    } else {
        LocationsSerializable::new()
    }
}

pub fn set_persistent(seed_name: &str, locations: LocationsSerializable) {
    create_dir_all("./persistent").expect("Failed to create persistent storage");
    if let Ok(writer) = File::create(Path::new("./persistent").join(seed_name)) {
        let res = to_writer(writer, &locations);
        if res.is_err() {
            println!("Failed to save locations to persistent storage")
        }
    } else {
        println!("Failed to save locations to persistent storage")
    }
}

fn resolve_multi_send(location: Location) -> Box<[Location]> {
    match location {
        Location::Variant(_) => Box::new([location]),
        Location::Villain((v, d)) => (0..=3).filter(|b| (d & *b) == *b).map(|d| Location::Villain((v, d))).collect(),
        Location::TeamVillain((v, d)) => (0..=3).filter(|b| (d & *b) == *b).map(|d| Location::TeamVillain((v, d))).collect(),
        Location::Environment(_) => Box::new([location]),
    }
}
