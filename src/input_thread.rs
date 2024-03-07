use std::process::exit;

use console::{Key, Term};
use tokio::{sync::mpsc::Sender, task::yield_now};

use crate::cli::DisplayUpdate;

pub async fn input_thread(sender: Sender<DisplayUpdate>) {
    let term = Term::stdout();
    let mut filter = String::new();
    loop {
        let key = term.read_key_raw().unwrap();
        match key {
            Key::ArrowLeft => {
                let _ = sender.send(DisplayUpdate::CursorLeft).await;
            }
            Key::ArrowRight => {
                let _ = sender.send(DisplayUpdate::CursorRight).await;
            }
            Key::ArrowUp => {
                let _ = sender.send(DisplayUpdate::CursorUp).await;
            }
            Key::ArrowDown => {
                let _ = sender.send(DisplayUpdate::CursorDown).await;
            }
            Key::Enter => {
                let _ = sender.send(DisplayUpdate::Send).await;
            }
            Key::Shift => {
                let _ = sender.send(DisplayUpdate::Select).await;
            }
            Key::Backspace => {
                filter.pop();
                let _ = sender.send(DisplayUpdate::Filter(filter.clone())).await;
            }
            Key::Char(char) => {
                if char == '\u{4}' {
                    let _ = term.show_cursor();
                    exit(0);
                }
                filter.push(char);
                let _ = sender.send(DisplayUpdate::Filter(filter.clone())).await;
            }
            Key::Home => {
                let _ = sender.send(DisplayUpdate::CursorHome).await;
            }
            Key::CtrlC => {
                filter.clear();
                let _ = sender.send(DisplayUpdate::Filter(filter.clone())).await;
            }
            _ => (),
        }
        yield_now().await;
    }
}
