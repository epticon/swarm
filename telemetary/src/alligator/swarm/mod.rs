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
        &mut self,
        division_name: &str,
        message: &T,
        skip_id: usize,
    ) -> Result<(), SendError<Message>>
    where
        T: Serialize,
    {
        let json = Rc::new(to_string(&message).unwrap()); // this is safe

        if let Some(node) = self.network.get_division_as_mut(&division_name) {
            let mut closed_drones = vec![];

            for drone in node.drones().iter() {
                if *drone.0 != skip_id {
                    let response = (drone.1).1.address().try_send(Message(json.to_string()));

                    if response.is_err() {
                        closed_drones.push(*drone.0);
                    }
                }
            }

            // Remove every disconnected drones (failure to do so, will result
            //  in delayed sending of messaged to drones in future).
            for session_id in closed_drones.iter() {
                node.remove(*session_id);
                println!(
                    "Removing drone with ID: {}, as client connection is closed.",
                    session_id
                );
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
