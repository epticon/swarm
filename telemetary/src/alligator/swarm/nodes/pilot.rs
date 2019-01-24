use crate::alligator::swarm::nodes::ClientNodeTrait;
use actix::Recipient;
use multi_map::MultiMap;

use crate::alligator::swarm::Message;

#[derive(Clone)]
pub(crate) struct Pilot {
    address: Recipient<Message>,
}

pub(crate) struct PilotNode {
    inner: MultiMap<usize, String, Pilot>, // <session_id, hash, pilot>
}

impl ClientNodeTrait for PilotNode {
    type Item = Pilot;
}

impl PilotNode {
    pub fn get_pilot(&self, session_id: usize) -> Option<&Pilot> {
        self.get(session_id, &self.inner)
    }

    pub fn pilots(&self) -> Vec<&Pilot> {
        self.get_all(&self.inner)
    }
}

impl Default for PilotNode {
    fn default() -> Self {
        PilotNode {
            inner: MultiMap::new(),
        }
    }
}
