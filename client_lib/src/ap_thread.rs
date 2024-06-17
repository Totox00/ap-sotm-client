use tokio::{sync::mpsc::UnboundedSender, task::yield_now};

use archipelago_rs::{self, client::ClientReceiver, protocol::ServerMessage};

use crate::Update;

pub async fn ap_thread(client_sender: UnboundedSender<Update>, mut ap_receiver: ClientReceiver) {
    loop {
        let res = ap_receiver.recv().await;
        if let Ok(Some(res)) = res {
            match res {
                ServerMessage::ReceivedItems(packet) => {
                    let _ = client_sender.send(Update::Items(packet.items.iter().map(|item| item.item).collect()));
                }
                ServerMessage::PrintJSON(msg) => {
                    let _ = client_sender.send(Update::Msg(msg));
                }
                _ => (),
            }
        }
        yield_now().await;
    }
}
