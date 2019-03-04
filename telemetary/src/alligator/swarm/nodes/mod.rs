use crate::alligator::{
    constants::DEFAULT_DRONE_CHANNEL,
    swarm::{uavs::Drone, users::Pilot},
    utils::hash_string,
};
use multi_map::MultiMap;
use std::cell::RefCell;
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
    #[allow(dead_code)]
    pub fn pilots_data(&self) -> Vec<&Pilot> {
        self.pilots.pilots_data()
    }

    pub fn get_default_division(&mut self) -> &mut DroneNode {
        self.get_or_create_division(DEFAULT_DRONE_CHANNEL)
    }

    pub fn get_or_create_division(&mut self, name: &str) -> &mut DroneNode {
        self.drones
            .entry(name.to_string())
            .or_insert_with(DroneNode::default)
    }

    pub fn delete_division(&mut self, name: &str) {
        let rc_self = RefCell::new(self);
        let sf1 = rc_self.borrow();
        let mut sf2 = rc_self.borrow_mut();

        if let Some(node) = sf1.drones.get(name) {
            for drone in node.drones().iter() {
                // Move all drones in division that is about to be deleted division into
                // the default divison (General) before deleting.
                sf2.get_default_division()
                    .insert((drone.1).1.to_owned(), Some(*drone.0));

                sf2.drones.get_mut(name).and_then(|e: &mut DroneNode| {
                    e.remove(*drone.0);
                    Some(())
                });
            }
        }
    }

    pub fn pilots_node(&self) -> &PilotNode {
        &self.pilots
    }

    pub fn _drones(&self) -> &HashMap<String, DroneNode> {
        &self.drones
    }

    pub fn _division(&self, division_name: &str) -> Option<&DroneNode> {
        self.drones.get(&hash_string(&division_name.to_lowercase()))
    }

    pub fn division_as_mut(&mut self, division_name: &str) -> Option<&mut DroneNode> {
        self.drones
            .get_mut(&hash_string(&division_name.to_lowercase()))
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

    pub fn insert_pilot(&mut self, pilot: Pilot) -> usize {
        self.pilots.insert(pilot)
    }

    pub fn remove_drone(&mut self, division_name: &str, session_id: Session) -> Option<Drone> {
        self.drones.remove(division_name)?.remove(session_id)
    }

    pub fn remove_pilot(&mut self, session_id: Session) -> Option<Pilot> {
        self.pilots.remove(session_id)
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
