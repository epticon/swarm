use crate::alligator::swarm::Message;
use crate::alligator::swarm::Swarm;
use actix::dev::SendError;
use actix::prelude::{Context, Handler, Message as ActixMessage};

type MessagingResponse = Result<(), SendError<Message>>;

#[derive(ActixMessage, Debug)]
#[rtype(MessagingResponse)]
pub(crate) struct SendCommandToDrones {
    pub skip_id: Option<usize>,
    pub division_name: String,
    pub message: String,
}

#[derive(ActixMessage, Debug)]
#[rtype(MessagingResponse)]
pub(crate) struct SendCommandToPilots {
    pub skip_id: Option<usize>,
    pub message: String,
}

impl Handler<SendCommandToDrones> for Swarm {
    type Result = Result<(), SendError<Message>>;

    fn handle(&mut self, msg: SendCommandToDrones, _: &mut Context<Self>) -> Self::Result {
        self.send_message_to_drones(&msg.division_name, &msg.message, 3)
    }
}

impl Handler<SendCommandToPilots> for Swarm {
    type Result = Result<(), SendError<Message>>;

    fn handle(&mut self, msg: SendCommandToPilots, _: &mut Context<Self>) -> Self::Result {
        self.send_message_to_pilots(&msg.message)
    }
}
