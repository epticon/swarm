pub(crate) use self::clients::*;
pub(crate) use self::message::*;
use self::nodes::RootNode;
use crate::alligator::utils::notifications::{notify_message, NotificationTypes};
use actix::prelude::SendError;
use actix::prelude::{Actor, Context, Message as ActixMessage};
use serde::Serialize;
use serde_json::{json, to_string};
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

        let mut closed_drones = vec![];
        if let Some(node) = self.network.division_as_mut(&division_name) {
            for drone in node.drones().iter() {
                if *drone.0 != skip_id {
                    let response = (drone.1).1.address().do_send(Message(json.to_string()));

                    if response.is_err() {
                        closed_drones.push(*drone.0);
                    }
                }
            }

            // Remove every disconnected drone. This helps remove redundant clients.
            for session_id in closed_drones.iter() {
                node.remove(*session_id);
                println!(
                    "Removing drone with Session: {}, as client connection is closed.",
                    session_id
                );
            }
        }

        if !closed_drones.is_empty() {
            let _ = self.send_message_to_pilots(&notify_message(
                json!({
                    "division_name": division_name,
                    "drones_session": &closed_drones
                }),
                NotificationTypes::DronesDown,
            ));
        }

        Ok(())
    }

    fn send_message_to_pilots<T: Serialize>(
        &mut self,
        message: &T,
    ) -> Result<(), SendError<Message>> {
        let json = Rc::new(to_string(&message).unwrap()); // this is safe

        let mut closed_pilots = vec![];

        for pilot in self.network.pilots_node().pilots().iter() {
            let response = (pilot.1).1.address().do_send(Message(json.to_string()));

            if response.is_err() {
                closed_pilots.push(*pilot.0);
            }
        }

        // Remove every disconnected pilot. This helps remove redundant clients.
        for session_id in closed_pilots.iter() {
            self.network.remove_pilot(*session_id);

            println!(
                "Removing pilot with Session: {}, as client connection is closed.",
                session_id
            );
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
