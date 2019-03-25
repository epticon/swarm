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

    pub fn insert(&mut self, drone: Drone, session_id: Option<usize>) -> Session {
        let session_id = match session_id {
            None => self.range.gen::<Session>(),
            Some(id) if !self.inner.contains_key(&id) => self.range.gen::<Session>(),
            Some(id) => id,
        };

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

// impl Serialize for DroneNode {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut map = serializer.serialize_map(Some(self.inner.iter().len()))?;
//         for (session, (hash, drone)) in self.inner.iter() {
//             map.serialize_entry(session, &(hash, drone))?;
//         }
//         map.end()
//     }
// }

// impl Clone for DroneNode {
//     fn clone(&self) -> Self {
//         let mut map = MultiMap::new();
//         for m in self.inner.iter() {
//             map.insert(*m.0, (m.1).0.to_string(), (m.1).1.clone());
//         }

//         Self {
//             inner: map,
//             range: self.range.clone(),
//         }
//     }
// }
