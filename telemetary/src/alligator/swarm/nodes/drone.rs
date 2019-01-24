use actix::Recipient;
use multi_map::MultiMap;

use crate::alligator::swarm::Message;

pub(crate) struct DroneNode {
    inner: MultiMap<String, String, Drone>, // <division_name, hash, drone>
}

pub(crate) struct Drone {
    address: Recipient<Message>,
}

impl Default for DroneNode {
    fn default() -> Self {
        DroneNode {
            inner: MultiMap::new(),
        }
    }
}
