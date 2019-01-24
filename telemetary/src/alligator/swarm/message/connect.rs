use crate::alligator::server::ClientType;
use crate::alligator::swarm::{Message, Swarm};
use actix::prelude::{Context, Handler, Message as ActixMessage, Recipient};

#[derive(ActixMessage)]
#[rtype(usize)]
pub(crate) struct Connect {
    pub client_id: String,
    pub client_type: ClientType,
    pub address: Recipient<Message>,
}

// Handles connect message for the swarm.
impl Handler<Connect> for Swarm {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // Improve error message by specifying the type of client that disconnected.
        println!("Someone just connected");

        // Add the client recient address from the respective swarm node.
        4
    }
}
