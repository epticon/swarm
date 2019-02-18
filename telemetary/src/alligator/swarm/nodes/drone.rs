use crate::alligator::swarm::nodes::ClientNodeTrait;
use crate::alligator::swarm::{
    nodes::{HashString, Session},
    uavs::Drone,
    ClientTrait,
};
use multi_map::MultiMap;
use rand::Rng;

pub(crate) struct DroneNode {
    inner: MultiMap<Session, HashString, Drone>, // <session_id, hash, drone>
    range: rand::rngs::ThreadRng,
}

impl ClientNodeTrait for DroneNode {
    type Item = Drone;
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

    pub fn remove(&mut self, session_id: Session) -> Option<Drone> {
        self.inner.remove(&session_id)
    }

    pub fn _remove_by_hash(&mut self, hash: &str) -> Option<Drone> {
        self.inner.remove_alt(hash)
    }

    pub fn drones(&self) -> &MultiMap<Session, HashString, Drone> {
        &self.inner
    }

    pub fn drones_as_mut(&mut self) -> &mut MultiMap<Session, HashString, Drone> {
        &mut self.inner
    }

    pub fn _get_values(&self) -> Vec<&Drone> {
        self.get_all(&self.inner)
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
