use crate::alligator::{
    constants::DEFAULT_DRONE_CHANNEL,
    swarm::{uavs::Drone, users::Pilot},
    utils::hash_string,
};
use multi_map::MultiMap;
use std::collections::{hash_map::Entry, HashMap};

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

impl RootNode {
    pub fn drones_node(&self) -> &HashMap<String, DroneNode> {
        &self.drones
    }

    pub fn pilots_node(&self) -> &PilotNode {
        &self.pilots
    }

    pub fn division_names(&self) -> Vec<&String> {
        self.drones.keys().collect()
    }

    pub fn insert_drone<'a>(&mut self, division_name: &'a str, drone: Drone) -> usize {
        match self
            .drones
            .entry(hash_string(&division_name.to_lowercase()))
        {
            Entry::Vacant(node) => {
                let mut drone_node = DroneNode::new();
                let session_id = drone_node.insert(drone, None);

                // Inserts a new drone node with the division
                // name that was checked for.
                node.insert(drone_node);
                session_id
            }

            Entry::Occupied(mut value) => value.get_mut().insert(drone, None),
        }
    }

    pub fn remove_drone(&mut self, division_name: &str, session_id: Session) -> Option<Drone> {
        self.drones.remove(division_name)?.remove(session_id)
    }

    pub fn insert_pilot(&mut self, pilot: Pilot) -> usize {
        self.pilots.insert(pilot)
    }

    pub fn remove_pilot(&mut self, session_id: Session) -> Option<Pilot> {
        self.pilots.remove(session_id)
    }

    #[allow(dead_code)]
    pub fn pilots_data(&self) -> Vec<&Pilot> {
        self.pilots.pilots_data()
    }

    pub fn _division(&self, division_name: &str) -> Option<&DroneNode> {
        self.drones.get(&hash_string(&division_name.to_lowercase()))
    }

    pub fn division_as_mut(&mut self, division_name: &str) -> Option<&mut DroneNode> {
        self.drones
            .get_mut(&hash_string(&division_name.to_lowercase()))
    }

    pub fn get_or_create_division(&mut self, name: &str) -> &mut DroneNode {
        self.drones
            .entry(name.to_string())
            .or_insert_with(DroneNode::default)
    }

    pub fn delete_division(&mut self, name: &str) {
        let mut drones = vec![];
        if let Some(node) = self.drones.get(name) {
            for drone in node.drones().iter() {
                drones.push((*drone.0, (drone.1).0.to_string(), (drone.1).1.clone())); // (session, hash, drone)
            }
        }

        for drone in drones {
            // Insert into the default division.
            self.get_default_division().insert(drone.2, Some(drone.0));

            // Delete all moved drones by session id from previous division.
            if let Some(a) = self.drones.get_mut(name) {
                a.remove(drone.0);
            }
        }

        if let Some(node) = self.drones.get(name) {
            // Only delete iff the division is empty (this is essential for cases
            // were another pilot would have added a drone to this same division
            // in few seconds of last check).
            if node.len() == 0 {
                self.drones.remove(name);
            }
        }
    }

    pub fn get_default_division(&mut self) -> &mut DroneNode {
        self.get_or_create_division(DEFAULT_DRONE_CHANNEL)
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
