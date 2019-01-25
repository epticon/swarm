use crate::alligator::swarm::{
    nodes::{ClientNodeTrait, HashString, Session},
    ClientTrait, Pilot,
};
use multi_map::MultiMap;
use rand::Rng;

pub(crate) struct PilotNode {
    inner: MultiMap<Session, HashString, Pilot>,
    range: rand::rngs::ThreadRng,
}

impl ClientNodeTrait for PilotNode {
    type Item = Pilot;
}

impl PilotNode {
    pub fn insert(&mut self, pilot: Pilot) -> Session {
        let session_id = self.range.gen::<Session>();

        self.inner
            .insert(session_id, pilot.hash().to_owned(), pilot);

        session_id
    }

    pub fn _get_pilot(&self, session_id: usize) -> Option<&Pilot> {
        self.get(session_id, &self.inner)
    }

    pub fn _pilots(&self) -> Vec<&Pilot> {
        self.get_all(&self.inner)
    }
}

impl Default for PilotNode {
    fn default() -> Self {
        PilotNode {
            inner: MultiMap::new(),
            range: rand::thread_rng(),
        }
    }
}
