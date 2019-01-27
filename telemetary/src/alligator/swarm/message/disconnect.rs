use crate::alligator::{self, server::ClientType, swarm::Swarm};
use actix::prelude::{Context, Handler, Message as ActixMessage};

#[derive(ActixMessage, Debug)]
pub(crate) struct Disconnect {
    pub session_id: usize,
    pub client: ClientType,
}

// Handles disconnect message for the swarm.
impl Handler<Disconnect> for Swarm {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        alligator::log(&format!("{:?} just disconnected", msg));

        // Remove the client recipient address from the respective swarm node.
        match msg.client {
            ClientType::Drone { division_name, .. } => {
                self.network.remove_drone(&division_name, msg.session_id);
            }

            ClientType::Pilot { .. } => {
                self.network.remove_pilot(msg.session_id);
            }
        };
    }
}
