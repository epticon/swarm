use crate::alligator::swarm::devices::{ClientTrait, DeviceTrait, Message};
use actix::Recipient;

#[derive(Clone)]
pub(crate) struct Drone {
    hash: String,
    owner_hash: String,
    address: Recipient<Message>,
}

pub(crate) struct DroneConfig {
    pub hash: String,
    pub owner_hash: String,
    pub address: Recipient<Message>,
}

impl<'a> Drone {
    pub fn new(config: &'a DroneConfig) -> Self {
        Drone {
            address: config.address.clone(),
            owner_hash: config.owner_hash.to_string(),
            hash: config.hash.to_string(),
        }
    }
}

impl<'a> DeviceTrait<'a> for Drone {
    fn owner_hash(&'a self) -> &'a str {
        &self.owner_hash
    }
}

impl<'a> ClientTrait<'a> for Drone {
    fn address(&'a self) -> &'a Recipient<Message> {
        &self.address
    }

    fn hash(&'a self) -> &'a str {
        &self.hash
    }
}
