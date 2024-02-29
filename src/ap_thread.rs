use tokio::sync::mpsc::Sender;

use crate::{
    archipelago_rs::{self, client::ArchipelagoClientReceiver},
    cli::DisplayUpdate,
    data::Item,
    state::State,
};

pub async fn ap_thread(mut state: State, client_sender: Sender<DisplayUpdate>, mut ap_receiver: ArchipelagoClientReceiver) {
    let id_map = state.idmap.clone();
    let _ = client_sender.send(DisplayUpdate::State(state.clone())).await;

    loop {
        if let Some(res) = ap_receiver.recv().await.unwrap() {
            match res {
                archipelago_rs::protocol::ServerMessage::RoomInfo(_) => (),
                archipelago_rs::protocol::ServerMessage::ConnectionRefused(_) => (),
                archipelago_rs::protocol::ServerMessage::Connected(_) => (),
                archipelago_rs::protocol::ServerMessage::ReceivedItems(packet) => {
                    for item in packet.items {
                        if let Some(item) = id_map.items_from_id.get(&item.item) {
                            match item {
                                Item::Hero(hero) => state.items.set_hero(*hero),
                                Item::Variant(variant) => state.items.set_hero_variant(*variant),
                                Item::Villain(villain) => state.items.set_villain(*villain),
                                Item::TeamVillain(teamvillain) => state.items.set_team_villain(*teamvillain),
                                Item::Environment(environment) => state.items.set_environment(*environment),
                                Item::Scion => state.items.scions += 1,
                                Item::Filler => (),
                            }
                        }
                    }
                    let _ = client_sender.send(DisplayUpdate::State(state.clone())).await;
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
    }
}
