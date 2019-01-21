use actix::{Actor,Context};

pub(crate) struct Swarm;

impl Actor for Swarm {
    type Context = Context<Self>;
}

impl Default for Swarm {
    fn default() -> Self {
        Swarm
    }
}
