use crate::alligator::swarm::devices::{drone::Drone, DeviceTrait};
use multi_map::MultiMap;
use rand::Rng;

pub(crate) struct DroneNode {
    inner: MultiMap<usize, String, Drone>, // <session_id, hash, drone>
    range: rand::rngs::ThreadRng,
}

impl DroneNode {
    pub fn new() -> Self {
        DroneNode {
            range: rand::thread_rng(),
            inner: MultiMap::new(),
        }
    }

    pub fn insert(&mut self, drone: Drone) {
        let session_id = self.range.gen::<usize>();
        self.inner
            .insert(session_id, drone.hash().to_string(), drone);
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
