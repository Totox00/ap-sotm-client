use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc},
};

use console::{style, StyledObject};
use tokio::task::yield_now;

use crate::{
    archipelago_rs::{self, client::ArchipelagoClientReceiver, protocol::Connected},
    cli::DisplayUpdate,
    idmap::IdMap,
};

struct GameData {
    item_id_to_name: HashMap<i64, String>,
    location_id_to_name: HashMap<i64, String>,
}

pub async fn ap_thread(id_map: Arc<IdMap>, client_sender: Sender<DisplayUpdate>, mut ap_receiver: ArchipelagoClientReceiver, connected: Connected, slot: String) {
    let mut player_to_game: HashMap<i32, Arc<GameData>> = HashMap::new();

    for (name, game) in ap_receiver.data_package.games.iter() {
        let mut item_id_to_name = HashMap::new();
        let mut location_id_to_name = HashMap::new();

        for (item, id) in &game.item_name_to_id {
            item_id_to_name.insert(*id, item.to_owned());
        }
        for (location, id) in &game.location_name_to_id {
            location_id_to_name.insert(*id, location.to_owned());
        }

        let rc = Arc::new(GameData { item_id_to_name, location_id_to_name });
        for player in connected
            .players
            .iter()
            .filter(|p| connected.slot_info.get(&p.slot.to_string()).is_some_and(|s| s.game == *name))
            .map(|p| p.slot)
        {
            player_to_game.insert(player, rc.clone());
        }
    }

    loop {
        let res = ap_receiver.recv().await;
        if let Ok(Some(res)) = res {
            match res {
                archipelago_rs::protocol::ServerMessage::ReceivedItems(packet) => {
                    let _ = client_sender.send(DisplayUpdate::Items(packet.items.iter().filter_map(|item| id_map.items_from_id.get(&item.item)).copied().collect()));
                }
                archipelago_rs::protocol::ServerMessage::PrintJSON(msg) => {
                    let _ = client_sender.send(DisplayUpdate::Msg(
                        msg.data
                            .iter()
                            .map(|part| {
                                let mut t_str = "text";
                                if let Some(str) = &part.r#type {
                                    t_str = str.as_str();
                                }
                                let text = part.text.clone().unwrap_or_default();
                                match t_str {
                                    "player_id" => {
                                        let player = connected
                                            .players
                                            .iter()
                                            .find(|player| player.slot == text.parse().unwrap_or(0))
                                            .map(|player| player.alias.clone())
                                            .unwrap_or(format!("Unknown player {text}"));
                                        if slot == player {
                                            style(player).magenta()
                                        } else {
                                            style(player).yellow()
                                        }
                                    }
                                    "player_name" => {
                                        if slot == text {
                                            style(text).magenta()
                                        } else {
                                            style(text).yellow()
                                        }
                                    }
                                    "item_id" => style_item(
                                        if let Some(data) = player_to_game.get(&part.player.unwrap_or(0)) {
                                            data.item_id_to_name.get(&text.parse().unwrap_or(0)).cloned().unwrap_or(String::from("Unknown item"))
                                        } else {
                                            String::from("Unknown item")
                                        },
                                        part.flags.unwrap_or(0),
                                    ),
                                    "item_name" => style(text).cyan(),
                                    "location_id" => style(if let Some(data) = player_to_game.get(&part.player.unwrap_or(0)) {
                                        data.location_id_to_name.get(&text.parse().unwrap_or(0)).cloned().unwrap_or(String::from("Unknown location"))
                                    } else {
                                        String::from("Unknown location")
                                    })
                                    .green(),
                                    "location_name" => style(text).green(),
                                    "entrance_name" => style(text).italic(),
                                    "color" => style_color(text, &part.color.clone().unwrap_or(String::from("bold"))),
                                    _ => style(text),
                                }
                            })
                            .map(|style| style.to_string())
                            .collect::<String>(),
                    ));
                }
                _ => (),
            }
        }
        yield_now().await;
    }
}

fn style_item(str: String, flags: i32) -> StyledObject<String> {
    match flags {
        0b001 | 0b111 => style(str).magenta(),
        0b010 => style(str).blue(),
        0b011 => style(str).magenta().on_blue(),
        0b100 => style(str).red(),
        0b101 => style(str).magenta().on_red(),
        0b110 => style(str).blue().on_red(),
        _ => style(str).cyan(),
    }
}

fn style_color(str: String, color: &str) -> StyledObject<String> {
    match color {
        "bold" => style(str).bold(),
        "underline" => style(str).underlined(),
        "black" => style(str).black(),
        "red" => style(str).red(),
        "green" => style(str).green(),
        "yellow" => style(str).yellow(),
        "blue" => style(str).blue(),
        "magenta" => style(str).magenta(),
        "cyan" => style(str).cyan(),
        "white" => style(str).white(),
        "black_bg" => style(str).on_black(),
        "red_bg" => style(str).on_red(),
        "green_bg" => style(str).on_green(),
        "yellow_bg" => style(str).on_yellow(),
        "blue_bg" => style(str).on_blue(),
        "magenta_bg" => style(str).on_magenta(),
        "cyan_bg" => style(str).on_cyan(),
        "white_bg" => style(str).on_white(),
        _ => style(str),
    }
}
