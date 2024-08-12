pub mod data;
pub mod datapackage;
mod logic;
pub mod persistent;
pub mod state;

use archipelago_protocol::{Connected, PrintJSON};
use data::{Item, Location};
use datapackage::DatapackageStore;
use persistent::PersistentStore;
use state::{CleanedSlotData, Locations, State};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Update {
    Msg(PrintJSON),
    Items(Vec<i64>),
    Send(Vec<Location>),
    Exit(Locations),
}

#[allow(clippy::large_enum_variant)] // Boxing State wouldn't do much since most updates are state updates anyways
#[derive(Debug)]
pub enum DisplayUpdate {
    Msg(PrintJSON),
    ReceivedItems(Vec<Item>),
    CheckedLocations(Vec<Location>),
    Exit,
}

pub struct Session<D, P>
where
    D: DatapackageStore,
    P: PersistentStore,
{
    pub datapackage_store: D,
    pub persistent_store: P,
    pub players: HashMap<i32, String>,
    pub slot: String,
    pub slot_data: CleanedSlotData,
    pub state: State,
}

impl<D, P> Session<D, P>
where
    D: DatapackageStore,
    P: PersistentStore,
{
    pub fn new(seed_name: &str, datapackage_store: D, connected: Connected, slot: &str) -> Session<D, P> {
        let persistent_store = P::new(seed_name);

        let mut state = State::new(connected.slot_data);

        state.checked_locations = persistent_store.load();

        let mut players = HashMap::new();
        for player in connected.players {
            players.insert(player.slot, player.alias);
        }

        Session {
            datapackage_store,
            persistent_store,
            players,
            slot: slot.to_string(),
            slot_data: state.slot_data,
            state,
        }
    }
}
