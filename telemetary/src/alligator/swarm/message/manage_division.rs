use crate::alligator::swarm::Message;
use crate::alligator::swarm::Swarm;
use actix::dev::SendError;
use actix::prelude::{Context, Handler, Message as ActixMessage};

type Response = Result<String, SendError<Message>>;

#[derive(ActixMessage, Debug)]
#[rtype(Response)]
pub(crate) struct CreateDivision(pub String);

#[derive(ActixMessage, Debug)]
#[rtype(Response)]
pub(crate) struct DeleteDivision(pub String);

impl Handler<CreateDivision> for Swarm {
    type Result = Response;

    fn handle(&mut self, msg: CreateDivision, _: &mut Context<Self>) -> Self::Result {
        self.network.get_or_create_division(&msg.0);

        Ok("Successfully created division".to_string())
    }
}

impl Handler<DeleteDivision> for Swarm {
    type Result = Response;

    fn handle(&mut self, msg: DeleteDivision, _: &mut Context<Self>) -> Self::Result {
        self.network.delete_division(&msg.0);

        Ok("Successfully deleted division".to_string())
    }
}
