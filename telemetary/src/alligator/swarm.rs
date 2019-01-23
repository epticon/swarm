use crate::alligator::server::ClientType;
use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};

#[derive(ActixMessage)]
pub(crate) struct Disconnect {
    pub session_id: usize,
    pub client: ClientType,
}

#[derive(ActixMessage)]
pub struct Message(pub String);

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

// Handles disconnect message for the swarm.
impl Handler<Disconnect> for Swarm {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        // Improve error message by specifying the type of client that disconnected.
        println!("CLient disconnected");

        // Remove the client recient address from the respective swarm node.
    }
}

pub(crate) struct Swarm;

impl Actor for Swarm {
    type Context = Context<Self>;
}

impl Default for Swarm {
    fn default() -> Self {
        Swarm
    }
}
