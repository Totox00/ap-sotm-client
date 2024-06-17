use console::{Key, Term};
use tokio::{sync::mpsc::UnboundedSender, task::yield_now};

pub enum Input {
    Filter(String),
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
    CursorHome,
    Select,
    Send,
    Exit,
}

pub async fn input_thread(sender: UnboundedSender<Input>) {
    let term = Term::stdout();
    let mut filter = String::new();
    loop {
        let key = term.read_key_raw().unwrap();
        match key {
            Key::ArrowLeft => {
                let _ = sender.send(Input::CursorLeft);
            }
            Key::ArrowRight => {
                let _ = sender.send(Input::CursorRight);
            }
            Key::ArrowUp => {
                let _ = sender.send(Input::CursorUp);
            }
            Key::ArrowDown => {
                let _ = sender.send(Input::CursorDown);
            }
            Key::Enter => {
                let _ = sender.send(Input::Send);
            }
            Key::Tab => {
                let _ = sender.send(Input::Select);
            }
            Key::Backspace => {
                filter.pop();
                let _ = sender.send(Input::Filter(filter.clone()));
            }
            Key::Char(char) => {
                if char == '\u{4}' {
                    let _ = sender.send(Input::Exit);
                }
                filter.push(char);
                let _ = sender.send(Input::Filter(filter.clone()));
            }
            Key::Home => {
                let _ = sender.send(Input::CursorHome);
            }
            Key::CtrlC => {
                filter.clear();
                let _ = sender.send(Input::Filter(filter.clone()));
            }
            _ => (),
        }
        yield_now().await;
    }
}
