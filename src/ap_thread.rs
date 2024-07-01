use crate::Update;
use archipelago_client::{self, ClientReceiver};
use archipelago_protocol::ServerMessage;
use tokio::{sync::mpsc::UnboundedSender, task::yield_now};

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
