mod cli;
mod format_json;
mod input_thread;

use clap::Parser;
use cli::{find_location, print};
use client_lib::{data::Location, datapackage::DefaultDatapackageStore, persistent::DefaultPersistentStore, DisplayUpdate, Session, Update};
use console::Term;
use format_json::format;
use input_thread::{input_thread, Input};
use std::{
    collections::VecDeque,
    io::{self, BufRead, Write},
    process::exit,
};
use tokio::{runtime::Builder, select, sync::mpsc::unbounded_channel};

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

fn main() {
    let (ap_sender, ap_receiver) = unbounded_channel();
    let (input_sender, mut input_receiver) = unbounded_channel();
    let (server_sender, mut server_receiver) = unbounded_channel();

    let runtime = Builder::new_multi_thread().enable_io().build().unwrap();
    let (server, port, slot, pass) = get_server_info();

    let session: Session<DefaultDatapackageStore, DefaultPersistentStore> = runtime.block_on(Session::connect(&server, port, &slot, pass.as_deref(), true)).unwrap_or_else(|_| {
        runtime.block_on(Session::connect(&server, port, &slot, pass.as_deref(), false)).unwrap_or_else(|err| {
            println!("Failed to connect to archipelago server");
            dbg!(err);
            exit(1);
        })
    });

    println!("Connected!");

    let datapackage_store = session.datapackage_store.clone();
    let slot = session.slot.clone();
    let players = session.players.clone();
    let mut state = session.state;
    let mut filter = String::new();
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut msg_buffer = VecDeque::with_capacity(10);
    let mut multi_send = false;

    let term = Term::stdout();
    let _ = term.hide_cursor();

    runtime.spawn(session.run(ap_receiver, server_sender));
    runtime.spawn(input_thread(input_sender));
    runtime.block_on(async move {
        loop {
            let send_victory = print(&term, &state, &filter, cursor_x, cursor_y, &msg_buffer, multi_send);
            if send_victory {
                let _ = ap_sender.send(Update::Send(vec![Location::Victory]));
            }

            select! {
                Some(update) = server_receiver.recv() => match update {
                    DisplayUpdate::Msg(msg) => {
                        if msg_buffer.len() > 9 {
                            msg_buffer.pop_front();
                        }
                        msg_buffer.push_back(format(&datapackage_store, msg, &players, &slot));
                    }
                    DisplayUpdate::State(new_state) => {
                        state = new_state;
                    }
                    DisplayUpdate::Exit => {
                        let _ = term.show_cursor();
                        let _ = term.clear_screen();
                        exit(0);
                    }
                },
                Some(input) = input_receiver.recv() => match input {
                    Input::Filter(new_filter) => filter = new_filter,
                    Input::CursorLeft => cursor_x = cursor_x.saturating_sub(1),
                    Input::CursorRight => {
                        if cursor_x < 5 {
                            cursor_x += 1;
                        }
                    }
                    Input::CursorUp => cursor_y = cursor_y.saturating_sub(1),
                    Input::CursorDown => cursor_y += 1,
                    Input::CursorHome => (cursor_x, cursor_y) = (0, 0),
                    Input::Select => multi_send = !multi_send,
                    Input::Send => {
                        if let Some(location) = find_location(&state, &filter, cursor_x, cursor_y) {
                            let locations = if multi_send { resolve_multi_send(location) } else { vec![location] };
                            let len = locations.len();

                            let _ = ap_sender.send(Update::Send(locations));

                            if cursor_y > 0 {
                                if cursor_y < len {
                                    cursor_y += 1;
                                    cursor_x -= 1;
                                }
                                cursor_y -= len;
                            } else {
                                cursor_x -= 1;
                            }
                        }
                    }
                    Input::Exit => {
                        if ap_sender.send(Update::Exit).is_err() {
                            println!("Failed to exit cleanly. Checked locations may have failed to save");
                            exit(1);
                        }
                    }
                },
            }
        }
    });
}

fn get_server_info() -> (String, u16, String, Option<String>) {
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

    (server, port.parse().unwrap_or(38281), slot, password)
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

fn resolve_multi_send(location: Location) -> Vec<Location> {
    match location {
        Location::Variant(_) => vec![location],
        Location::Villain((v, d)) => (0..=3).filter(|b| (d & *b) == *b).map(|d| Location::Villain((v, d))).collect(),
        Location::TeamVillain((v, d)) => (0..=3).filter(|b| (d & *b) == *b).map(|d| Location::TeamVillain((v, d))).collect(),
        Location::Environment(_) => vec![location],
        Location::Victory => vec![Location::Victory],
    }
}
