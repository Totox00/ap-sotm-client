use std::collections::HashMap;

use archipelago_rs::protocol::PrintJSON;
use client_lib::datapackage::DatapackageStore;
use console::{style, StyledObject};

pub fn format<D>(datapackage_store: &D, msg: PrintJSON, players: &HashMap<i32, String>, slot: &str) -> String
where
    D: DatapackageStore,
{
    msg.data
        .iter()
        .map(|part| {
            let mut t_str = "text";
            if let Some(str) = &part.r#type {
                t_str = str;
            }
            let text = part.text.clone().unwrap_or_default();
            match t_str {
                "player_id" => {
                    if let Some(player) = players.get(&text.parse::<i32>().unwrap_or(0)) {
                        if slot == *player {
                            style(player.to_string()).magenta()
                        } else {
                            style(player.to_string()).yellow()
                        }
                    } else {
                        style(format!("Unknown player {text}")).yellow()
                    }
                }
                "player_name" => {
                    if slot == text {
                        style(text.to_string()).magenta()
                    } else {
                        style(text.to_string()).yellow()
                    }
                }
                "item_id" => style_item(datapackage_store.get_item(part.player.unwrap_or(0), text.parse().unwrap_or(0)).to_string(), part.flags.unwrap_or(0)),
                "item_name" => style(text.to_string()).cyan(),
                "location_id" => style(datapackage_store.get_location(part.player.unwrap_or(0), text.parse().unwrap_or(0)).to_string()).green(),
                "location_name" => style(text.to_string()).green(),
                "entrance_name" => style(text.to_string()).italic(),
                "color" => {
                    if let Some(color) = &part.color {
                        style_color(text.to_string(), color)
                    } else {
                        style_color(text.to_string(), "bold")
                    }
                }
                _ => style(text.to_string()),
            }
        })
        .map(|style| style.to_string())
        .collect()
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
