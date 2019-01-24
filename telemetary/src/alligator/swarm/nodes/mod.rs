mod drone;
mod pilot;

pub(crate) use self::drone::DroneNode;
pub(crate) use self::pilot::PilotNode;
use multi_map::MultiMap;

// Pilots in a RootNode are priviledged to send command to all
// drones in the DroneNode contained in the same RootNode.
pub(crate) struct RootNode {
    pilots: PilotNode,
    drones: DroneNode,
}

impl Default for RootNode {
    fn default() -> Self {
        Self {
            drones: DroneNode::default(),
            pilots: PilotNode::default(),
        }
    }
}

pub(crate) trait ClientNodeTrait {
    type Item;

    fn get<'a>(
        &self,
        session_id: usize,
        inner: &'a MultiMap<usize, String, Self::Item>,
    ) -> Option<&'a Self::Item> {
        inner.get(&session_id)
    }

    fn get_all<'a>(&self, inner: &'a MultiMap<usize, String, Self::Item>) -> Vec<&'a Self::Item> {
        inner.iter().map(|s| &(s.1).1).collect()
    }
}
