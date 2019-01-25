use crate::alligator::swarm::devices::DeviceTrait;
use crate::alligator::swarm::Message;
use actix::Recipient;

#[derive(Clone)]
pub(crate) struct Drone {
    hash: String,
    owner_hash: String,
    address: Recipient<Message>,
}

impl<'a> DeviceTrait<'a> for Drone {
    fn address(&'a self) -> &'a Recipient<Message> {
        &self.address
    }

    fn owner_hash(&'a self) -> &'a str {
        &self.owner_hash
    }

    fn hash(&'a self) -> &'a str {
        &self.hash
    }
}

pub(crate) struct DroneConfig {
    pub hash: String,
    pub owner_hash: String,
    pub address: Recipient<Message>,
}

// address: Recipient<Message>, owner_hash: &'a str, hash: &'a str

impl<'a> Drone {
    pub fn new(config: &'a DroneConfig) -> Self {
        Drone {
            address: config.address.clone(),
            owner_hash: config.owner_hash.to_string(),
            hash: config.hash.to_string(),
        }
    }
}
