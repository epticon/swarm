use crate::alligator::server::ClientType;
use crate::alligator::swarm::Swarm;
use actix::prelude::{Context, Handler, Message as ActixMessage};

#[derive(ActixMessage)]
pub(crate) struct Disconnect {
    pub session_id: usize,
    pub client: ClientType,
}

// Handles disconnect message for the swarm.
impl Handler<Disconnect> for Swarm {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        // Improve error message by specifying the type of client that disconnected.
        println!("CLient disconnected");

        // Remove the client recient address from the respective swarm node.
    }
}
