use crate::alligator::swarm::{uavs::Drone, users::Pilot};
use multi_map::MultiMap;
use std::{
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    hash::{Hash, Hasher},
};

mod drone;
mod pilot;

pub(crate) use self::drone::DroneNode;
pub(crate) use self::pilot::PilotNode;

pub(crate) type HashString = String;
pub(crate) type Session = usize;

// Note:
// Currently, Pilots in a RootNode are priviledged to send command to all
// divisions/DroneNode contained in the same RootNode.
//
// Todo: A lot of improvement of the swarm structure has to be made,
// Todo: Concepts such as organisations and role-based pilot has to be introduced.
pub(crate) struct RootNode {
    pilots: PilotNode,
    drones: HashMap<String, DroneNode>, // <division_name, drone_node>
}

impl Default for RootNode {
    fn default() -> Self {
        Self {
            drones: HashMap::default(),
            pilots: PilotNode::default(),
        }
    }
}

// Hash a given string.
fn hash_string(string: &str) -> String {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish().to_string()
}

impl RootNode {
    pub fn insert_drone<'a>(&mut self, division_name: &'a str, drone: Drone) -> usize {
        match self
            .drones
            .entry(hash_string(&division_name.to_lowercase()))
        {
            Entry::Vacant(node) => {
                let mut drone_node = DroneNode::new();
                let session_id = drone_node.insert(drone);

                // Inserts a new drone node with the division
                // name that was checked for.
                node.insert(drone_node);
                session_id
            }

            Entry::Occupied(mut value) => value.get_mut().insert(drone),
        }
    }

    pub fn insert_pilot(&mut self, pilot: Pilot) -> usize {
        self.pilots.insert(pilot)
    }
}

pub(crate) trait ClientNodeTrait {
    type Item;

    fn get<'a>(
        &self,
        session_id: usize,
        inner: &'a MultiMap<usize, String, Self::Item>,
    ) -> Option<&'a Self::Item> {
        inner.get(&session_id)
    }

    fn get_all<'a>(&self, inner: &'a MultiMap<usize, String, Self::Item>) -> Vec<&'a Self::Item> {
        inner.iter().map(|s| &(s.1).1).collect()
    }
}
