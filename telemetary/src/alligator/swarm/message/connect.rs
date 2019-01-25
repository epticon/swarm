use crate::alligator::server::ClientType;
use crate::alligator::swarm::{Drone, DroneConfig, Message, Swarm};
use actix::prelude::{Context, Handler, Message as ActixMessage, Recipient};
use rand::Rng;

#[derive(ActixMessage)]
#[rtype(usize)]
pub(crate) struct Connect {
    // pub client_id: String,
    pub client: ClientType,
    pub address: Recipient<Message>,
}

// Handles connect message for the swarm.
impl Handler<Connect> for Swarm {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // Improve error message by specifying the type of client that disconnected.
        println!("Someone just connected");

        // Add the client recient address from the respective swarm node.
        // let session_id = self.range.gen::<usize>();

        match msg.client {
            ClientType::Drone {
                hash,
                owner_hash,
                division_name,
            } => {
                let config = DroneConfig {
                    address: msg.address,
                    owner_hash: owner_hash,
                    hash: hash,
                };

                self.network
                    .insert_drone(&division_name, Drone::new(&config));
            }

            ClientType::Pilot { hash } => {}
        };
        // session_id
        3
    }
}
