use crate::alligator::swarm::{
    devices::drone::Drone,
    nodes::{HashString, Session},
    ClientTrait,
};
use multi_map::MultiMap;
use rand::Rng;

pub(crate) struct DroneNode {
    inner: MultiMap<Session, HashString, Drone>, // <session_id, hash, drone>
    range: rand::rngs::ThreadRng,
}

impl DroneNode {
    pub fn new() -> Self {
        DroneNode {
            range: rand::thread_rng(),
            inner: MultiMap::new(),
        }
    }

    pub fn insert(&mut self, drone: Drone) -> Session {
        let session_id = self.range.gen::<Session>();
        self.inner
            .insert(session_id, drone.hash().to_string(), drone);

        session_id
    }
}

impl Default for DroneNode {
    fn default() -> Self {
        DroneNode {
            inner: MultiMap::new(),
            range: rand::thread_rng(),
        }
    }
}
