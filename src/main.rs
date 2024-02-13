mod archipelago_rs;
mod cli;
mod data;
mod idmap;
mod logic;
mod state;

use archipelago_rs::{client::ArchipelagoClient, protocol::Connected};
use clap::Parser;
use cli::print_state;
use console::{Key, Term};
use state::State;
use std::{
    io::{self, BufRead, Write},
    process::exit,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server: Option<Option<String>>,
    #[arg(short, long)]
    port: Option<Option<String>>,
    #[arg(short = 'S', long)]
    slot: Option<Option<String>>,
    #[arg(short = 'P', long)]
    password: Option<Option<String>>,
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let (mut client, mut state) = if let Ok((mut client, connected)) = connect().await {
        let mut state = State::new(client.data_package().expect("No datapackage was received"));
        state.sync(connected, client.sync().await.expect("Failed to sync items"));
        (client, state)
    } else {
        println!("Failed to connect to Archipelago server");
        exit(1);
    };

    println!("Connected!");
    let term = Term::stdout();

    loop {
        let _ = term.clear_screen();
        print_state(&state);
        let key = term.read_key_raw().unwrap();
        if key == Key::Char('\u{4}') {
            exit(0);
        }
    }
}

pub async fn connect() -> anyhow::Result<(ArchipelagoClient, Connected)> {
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

    let mut url = String::new();
    url.push_str("ws://");
    url.push_str(&server);
    url.push(':');
    url.push_str(&port);

    let mut client = ArchipelagoClient::with_data_package(&url, None).await?;

    let connected = if let Some(pass) = password {
        client.connect("Manual_manualsotm_toto00", &slot, Some(&*pass), Some(7), vec!["AP".to_string()]).await?
    } else {
        client.connect("Manual_manualsotm_toto00", &slot, None, Some(7), vec!["AP".to_string()]).await?
    };

    Ok((client, connected))
}

fn prompt(text: &str) -> Option<String> {
    print!("{} > ", text);
    let _ = io::stdout().flush();

    Some(io::stdin().lock().lines().next().unwrap().unwrap().as_str().into())
}
