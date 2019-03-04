use crate::alligator::swarm::Message;
use crate::alligator::swarm::Swarm;
use actix::dev::SendError;
use actix::prelude::{Context, Handler, Message as ActixMessage};
use std::ops::Deref;

type Response = Result<String, SendError<Message>>;
type DivisionListResponse = Result<Vec<String>, SendError<Message>>;

#[derive(ActixMessage, Debug)]
#[rtype(DivisionListResponse)]
pub(crate) struct GetAllDivisions;

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

impl<'a> Handler<GetAllDivisions> for Swarm {
    type Result = DivisionListResponse;

    fn handle(&mut self, _: GetAllDivisions, _: &mut Context<Self>) -> Self::Result {
        Ok(self
            .network
            .division_names()
            .into_iter()
            .map(|e| e.deref().to_string())
            .collect())
    }
}
