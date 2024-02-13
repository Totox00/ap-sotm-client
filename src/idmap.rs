use std::collections::HashMap;

use crate::{
    archipelago_rs::protocol::DataPackageObject,
    data::{Item, Location},
};

#[derive(Debug)]
pub struct IdMap {
    pub items_from_id: HashMap<i64, Item>,
    pub items_to_id: HashMap<Item, i64>,
    pub locations_from_id: HashMap<i64, Location>,
    pub locations_to_id: HashMap<Location, i64>,
}

impl IdMap {
    pub fn new(datapackage: &DataPackageObject) -> Self {
        let mut map = IdMap {
            items_from_id: HashMap::new(),
            items_to_id: HashMap::new(),
            locations_from_id: HashMap::new(),
            locations_to_id: HashMap::new(),
        };

        let data = datapackage.games.get("Manual_manualsotm_toto00").expect("Datapackage for Sentinels of the Multiverse was not found");

        for (item, id) in &data.item_name_to_id {
            if let Some(item) = Item::from_str(item.as_str()) {
                map.items_from_id.insert(*id, item);
                map.items_to_id.insert(item, *id);
            }
        }

        for (location, id) in &data.location_name_to_id {
            if let Some(location) = Location::from_str(location.as_str()) {
                map.locations_from_id.insert(*id, location);
                map.locations_to_id.insert(location, *id);
            }
        }

        map
    }
}
