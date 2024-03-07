use std::sync::Arc;

use tokio::{sync::mpsc::Sender, task::yield_now};

use crate::{
    archipelago_rs::{self, client::ArchipelagoClientReceiver},
    cli::DisplayUpdate,
    idmap::IdMap,
};

pub async fn ap_thread(id_map: Arc<IdMap>, client_sender: Sender<DisplayUpdate>, mut ap_receiver: ArchipelagoClientReceiver) {
    loop {
        let res = ap_receiver.recv().await;
        if let Ok(Some(res)) = res {
            match res {
                archipelago_rs::protocol::ServerMessage::RoomInfo(_) => (),
                archipelago_rs::protocol::ServerMessage::ConnectionRefused(_) => (),
                archipelago_rs::protocol::ServerMessage::Connected(_) => (),
                archipelago_rs::protocol::ServerMessage::ReceivedItems(packet) => {
                    let _ = client_sender
                        .send(DisplayUpdate::Items(packet.items.iter().filter_map(|item| id_map.items_from_id.get(&item.item)).copied().collect()))
                        .await;
                }
                archipelago_rs::protocol::ServerMessage::LocationInfo(_) => (),
                archipelago_rs::protocol::ServerMessage::RoomUpdate(_) => (),
                archipelago_rs::protocol::ServerMessage::Print(_) => (),
                archipelago_rs::protocol::ServerMessage::PrintJSON(_) => (),
                archipelago_rs::protocol::ServerMessage::DataPackage(_) => (),
                archipelago_rs::protocol::ServerMessage::Bounced(_) => (),
                archipelago_rs::protocol::ServerMessage::InvalidPacket(_) => (),
                archipelago_rs::protocol::ServerMessage::Retrieved(_) => (),
                archipelago_rs::protocol::ServerMessage::SetReply(_) => (),
            }
        }
        yield_now().await;
    }
}
