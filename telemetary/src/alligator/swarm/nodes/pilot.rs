use crate::alligator::swarm::{
    nodes::{ClientNodeTrait, HashString, Session},
    users::Pilot,
    ClientTrait,
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

    pub fn remove(&mut self, session_id: Session) -> Option<Pilot> {
        self.inner.remove(&session_id)
    }

    pub fn _remove_by_hash(&mut self, hash: &str) -> Option<Pilot> {
        self.inner.remove_alt(hash)
    }

    pub fn _get_pilot(&self, session_id: usize) -> Option<&Pilot> {
        self.get(session_id, &self.inner)
    }

    pub fn _get_pilot_by_hash(&self, hash: HashString) -> Option<&Pilot> {
        self.inner.get_alt(&hash)
    }

    pub fn pilots(&self) -> Vec<&Pilot> {
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
