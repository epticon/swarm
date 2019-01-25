pub(crate) use self::devices::drone::{Drone, DroneConfig};
pub(crate) use self::message::{Connect, Disconnect};
use self::nodes::RootNode;
pub(crate) use self::users::{Pilot, PilotConfig};
use actix::prelude::{Actor, Context, Message as ActixMessage};
use actix::Recipient;

mod devices;
mod message;
mod nodes;
mod users;

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

pub(crate) trait ClientTrait<'a, Address = &'a Recipient<Message>> {
    fn address(&'a self) -> Address {
        unimplemented!()
    }

    fn hash(&'a self) -> &'a str {
        unimplemented!()
    }
}
