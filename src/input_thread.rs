use std::process::exit;

use console::{Key, Term};
use tokio::sync::mpsc::Sender;

use crate::cli::DisplayUpdate;

pub async fn input_thread(sender: Sender<DisplayUpdate>) {
    let term = Term::stdout();
    let mut filter = String::new();
    loop {
        let key = term.read_key_raw().unwrap();
        match key {
            Key::ArrowLeft => {
                let _ = sender.send(DisplayUpdate::CursorLeft);
            }
            Key::ArrowRight => {
                let _ = sender.send(DisplayUpdate::CursorRight);
            }
            Key::ArrowUp => {
                let _ = sender.send(DisplayUpdate::CursorUp);
            }
            Key::ArrowDown => {
                let _ = sender.send(DisplayUpdate::CursorDown);
            }
            Key::Enter => {
                let _ = sender.send(DisplayUpdate::Send);
            }
            Key::Shift => {
                let _ = sender.send(DisplayUpdate::Select);
            }
            Key::Backspace => {
                filter.pop();
                let _ = sender.send(DisplayUpdate::Filter(filter.clone()));
            }
            Key::Char(char) => {
                if char == '\u{4}' {
                    exit(0);
                }
                filter.push(char);
                let _ = sender.send(DisplayUpdate::Filter(filter.clone()));
            }
            Key::Home => {
                let _ = sender.send(DisplayUpdate::CursorHome);
            }
            Key::CtrlC => {
                filter.clear();
                let _ = sender.send(DisplayUpdate::Filter(filter.clone()));
            }
            _ => (),
        }
    }
}
