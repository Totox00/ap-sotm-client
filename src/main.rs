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
use cli::{print, send_location, DisplayUpdate};
use console::Term;
use data::Item;
use input_thread::input_thread;
use serde_json::{from_reader, to_writer};
use state::State;
use std::{
    collections::{HashMap, VecDeque},
    fs::{create_dir, create_dir_all, remove_file, File},
    io::{self, stdout, BufRead, Write},
    path::Path,
    process::exit,
};
use tokio::{spawn, sync::mpsc::channel};

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

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let (client_sender, mut receiver) = channel::<DisplayUpdate>(16);

    let (client, mut state, connected, slot) = match connect().await {
        Ok((mut client, connected, slot)) => {
            let mut state = State::new(&client.data_package);
            state.sync(&connected, client.sync().await.expect("Failed to sync items"));
            (client, state, connected, slot)
        }
        Err(err) => {
            println!("Failed to connect to Archipelago server with reason: {err}");
            exit(1);
        }
    };

    println!("Connected!");
    let (mut ap_sender, ap_receiver) = client.split();

    drop(spawn(ap_thread(state.idmap.clone(), client_sender.clone(), ap_receiver, connected, slot)));
    drop(spawn(input_thread(client_sender)));

    let term = Term::stdout();
    let _ = term.hide_cursor();
    let mut filter = String::new();
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut msg_buffer = VecDeque::with_capacity(10);

    loop {
        print(&term, &state, &filter, cursor_x, cursor_y, &msg_buffer, &mut ap_sender).await;
        if let Some(update) = receiver.recv().await {
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
                DisplayUpdate::Select => (),
                DisplayUpdate::Send => {
                    send_location(&mut ap_sender, &mut state, &filter, cursor_x, cursor_y).await;
                    if cursor_y > 0 {
                        cursor_y -= 1;
                    } else {
                        cursor_x -= 1;
                    }
                }
            }
        } else {
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
pub async fn connect() -> anyhow::Result<(ArchipelagoClient, Connected, String)> {
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

    let url = format!("wss://{server}:{port}");

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
        client.connect("Manual_sotm_toto00", &slot, Some(&*pass), Some(7), vec!["AP".to_string()]).await?
    } else {
        client.connect("Manual_sotm_toto00", &slot, None, Some(7), vec!["AP".to_string()]).await?
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
