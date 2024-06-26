mod ap_thread;
pub mod data;
pub mod datapackage;
mod logic;
pub mod persistent;
pub mod state;

use anyhow::Result;
use ap_thread::ap_thread;
use archipelago_rs::{
    client::Client,
    protocol::{ClientStatus, PrintJSON},
};
use data::Location;
use datapackage::DatapackageStore;
use persistent::PersistentStore;
use state::State;
use std::collections::HashMap;
use tokio::{
    select, spawn,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

#[derive(Debug)]
pub enum Update {
    Msg(PrintJSON),
    Items(Vec<i64>),
    Send(Vec<Location>),
    Exit,
}

#[allow(clippy::large_enum_variant)] // Boxing State wouldn't do much since most updates are state updates anyways
#[derive(Debug)]
pub enum DisplayUpdate {
    Msg(PrintJSON),
    State(State),
    Exit,
}

pub struct Session<D, P>
where
    D: DatapackageStore,
    P: PersistentStore,
{
    pub datapackage_store: D,
    pub persistent_store: P,
    pub state: State,
    pub client: Client,
    pub players: HashMap<i32, String>,
    pub slot: String,
}

impl<D, P> Session<D, P>
where
    D: DatapackageStore,
    P: PersistentStore,
{
    pub async fn connect(server: &str, port: u16, slot: &str, pass: Option<&str>, secure: bool) -> Result<Session<D, P>> {
        let (mut client, room_info) = Client::new(&format!("{}://{server}:{port}", if secure { "wss" } else { "ws" })).await?;

        let mut datapackage_store = D::new(room_info.datapackage_checksums);
        let missing_games = datapackage_store.missing_games();
        let data_package = if !missing_games.is_empty() {
            Some(client.get_data_package(Some(missing_games)).await?.data)
        } else {
            None
        };

        let connected = client.connect("Sentinels of the Multiverse", slot, "", pass, Some(7), &["AP".into()]).await?;

        if let Some(data) = data_package {
            datapackage_store.cache(data);
        }

        datapackage_store.build_player_map(&connected);

        let persistent_store = P::new(&room_info.seed_name);

        let mut state = State::new(connected.slot_data);

        state.checked_locations = persistent_store.load();

        client.sync().await?;

        let mut players = HashMap::new();
        for player in connected.players {
            players.insert(player.slot, player.alias);
        }

        Ok(Session {
            datapackage_store,
            persistent_store,
            state,
            client,
            players,
            slot: slot.to_string(),
        })
    }

    pub async fn run(mut self, mut locations_to_send: UnboundedReceiver<Update>, display_sender: UnboundedSender<DisplayUpdate>) -> ! {
        let (mut ap_sender, ap_receiver) = self.client.split();
        let (sender, mut receiver) = unbounded_channel::<Update>();

        spawn(ap_thread(sender, ap_receiver));

        loop {
            if let Some(update) = select! {
                v = locations_to_send.recv() => v,
                v = receiver.recv() => v
            } {
                let _ = match update {
                    Update::Msg(msg) => display_sender.send(DisplayUpdate::Msg(msg)),
                    Update::Items(item_ids) => {
                        for id in item_ids {
                            if let Some(item) = self.datapackage_store.id_to_own_item(id) {
                                self.state.items.set_item(item);
                            }
                        }
                        display_sender.send(DisplayUpdate::State(self.state))
                    }
                    Update::Send(locations) => {
                        let mut location_ids = vec![];

                        for location in locations.iter() {
                            if *location == Location::Victory {
                                let res = ap_sender.status_update(ClientStatus::Goal).await;
                                if res.is_ok() {
                                    self.state.checked_locations.victory = true;
                                }
                            } else {
                                for n in 0..=self.state.slot_data.locations_per[match location {
                                    Location::Variant(_) => 5,
                                    Location::Villain((_, d)) | Location::TeamVillain((_, d)) => *d as usize,
                                    Location::Environment(_) => 4,
                                    Location::Victory => unreachable!(),
                                }] {
                                    if n > 0 {
                                        if let Some(id) = self.datapackage_store.id_from_own_location((*location, n)) {
                                            location_ids.push(id)
                                        }
                                    }
                                }
                            }
                        }

                        let res = ap_sender.location_checks(location_ids).await;
                        if res.is_ok() {
                            for location in locations.iter() {
                                match *location {
                                    Location::Variant(v) => self.state.checked_locations.mark_variant(v),
                                    Location::Villain((v, d)) => self.state.checked_locations.mark_villain(v, d),
                                    Location::TeamVillain((v, d)) => self.state.checked_locations.mark_team_villain(v, d),
                                    Location::Environment(e) => self.state.checked_locations.mark_environment(e),
                                    Location::Victory => (),
                                }
                            }
                        }

                        display_sender.send(DisplayUpdate::State(self.state))
                    }
                    Update::Exit => {
                        self.persistent_store.save(&self.state.checked_locations);
                        display_sender.send(DisplayUpdate::Exit)
                    }
                };
            }
        }
    }
}
