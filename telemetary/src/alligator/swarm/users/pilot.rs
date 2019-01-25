use crate::alligator::swarm::{ClientTrait, Message};
use actix::Recipient;

#[derive(Clone, Hash)]
pub(crate) struct Pilot {
    address: Recipient<Message>,
    hash: String,
}

pub(crate) struct PilotConfig {
    pub hash: String,
    pub address: Recipient<Message>,
}

impl<'a> Pilot {
    pub fn new(config: &'a PilotConfig) -> Self {
        Pilot {
            address: config.address.clone(),
            hash: config.hash.to_string(),
        }
    }
}

impl<'a> ClientTrait<'a> for Pilot {
    fn address(&'a self) -> &'a Recipient<Message> {
        &self.address
    }

    fn hash(&'a self) -> &'a str {
        &self.hash
    }
}
