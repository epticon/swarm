pub(crate) use self::message::{Connect, Disconnect};
use self::nodes::RootNode;
use actix::prelude::{Actor, Context, Message as ActixMessage};

mod message;
mod nodes;

#[derive(ActixMessage)]
pub struct Message(pub String);

pub(crate) struct Swarm {
    network: RootNode,
}

impl Actor for Swarm {
    type Context = Context<Self>;
}

impl Default for Swarm {
    fn default() -> Self {
        Swarm {
            network: RootNode::default(),
        }
    }
}
