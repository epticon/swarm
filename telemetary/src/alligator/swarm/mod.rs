pub(crate) use self::clients::*;
pub(crate) use self::message::*;
use self::nodes::RootNode;
use actix::prelude::SendError;
use actix::prelude::{Actor, Context, Message as ActixMessage};
use serde::Serialize;
use serde_json::to_string;
use std::rc::Rc;

mod clients;
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

impl Swarm {
    fn send_message_to_drones<T: Serialize>(
        &self,
        division_name: &str,
        message: &T,
        skip_id: usize,
    ) -> Result<(), SendError<Message>>
    where
        T: Serialize,
    {
        let json = Rc::new(to_string(&message).unwrap()); // this is safe

        if let Some(node) = self.network.drones().get(division_name) {
            for drone in node.drones().iter() {
                if *drone.0 != skip_id {
                    (drone.1).1.address().do_send(Message(json.to_string()))?;
                }
            }
        }

        Ok(())
    }

    fn send_message_to_pilots<T: Serialize>(&self, message: &T) -> Result<(), SendError<Message>> {
        let json = Rc::new(to_string(&message).unwrap()); // this is safe

        for pilot in self.network.pilots().iter() {
            pilot.address().do_send(Message(json.to_string()))?;
        }

        Ok(())
    }
}

impl Default for Swarm {
    fn default() -> Self {
        Swarm {
            network: RootNode::default(),
        }
    }
}
