use crate::alligator::swarm::clients::uavs::Drone;
use crate::alligator::swarm::nodes::{DroneNode, Session};
use crate::alligator::swarm::Message;
use crate::alligator::swarm::Swarm;
use crate::alligator::utils::unhash_string;
use actix::dev::SendError;
use actix::prelude::{Context, Handler, Message as ActixMessage};
use std::collections::HashMap;

type Response = Result<String, SendError<Message>>;
type DivisionNamesResponse = Result<Vec<String>, SendError<Message>>;
type DivisionsResponse = Result<HashMap<String, HashMap<Session, Drone>>, SendError<Message>>;

#[derive(ActixMessage, Debug)]
#[rtype(DivisionsResponse)]
pub(crate) struct GetAllDivisions;

#[derive(ActixMessage, Debug)]
#[rtype(DivisionNamesResponse)]
pub(crate) struct GetAllDivisionNames;

#[derive(ActixMessage, Debug)]
#[rtype(Response)]
pub(crate) struct CreateDivision(pub String);

#[derive(ActixMessage, Debug)]
#[rtype(Response)]
pub(crate) struct DeleteDivision(pub String);

#[derive(ActixMessage, Debug)]
#[rtype(Response)]
pub(crate) struct ChangeDivision {
    pub drone_session: usize,
    pub from: String,
    pub to: String,
}

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

impl<'a> Handler<GetAllDivisionNames> for Swarm {
    type Result = DivisionNamesResponse;

    fn handle(&mut self, _: GetAllDivisionNames, _: &mut Context<Self>) -> Self::Result {
        Ok(self
            .network
            .division_names()
            .into_iter()
            .map(|e| unhash_string(e))
            .collect())
    }
}

impl<'a> Handler<ChangeDivision> for Swarm {
    type Result = Response;

    fn handle(&mut self, cmd: ChangeDivision, _: &mut Context<Self>) -> Self::Result {
        let node = self.network.drones_node_as_mut();
        let division = node
            .get_mut(&cmd.from)
            .ok_or_else(|| SendError::Closed(Message(String::new())))?;

        let drone = division
            .remove(cmd.drone_session)
            .ok_or_else(|| SendError::Closed(Message(String::new())))?;

        let session = cmd.drone_session;
        node.entry(cmd.to.to_string())
            .and_modify(|e| {
                e.insert(drone.clone(), Some(session));
            })
            .or_insert_with(|| {
                let mut new_node = DroneNode::new();
                new_node.insert(drone, Some(session));
                new_node
            });

        Ok(format!(
            "Successfully moved drone from division: {} to {}",
            cmd.from, cmd.to
        ))
    }
}

impl<'a> Handler<GetAllDivisions> for Swarm {
    type Result = DivisionsResponse;

    fn handle(&mut self, _: GetAllDivisions, _: &mut Context<Self>) -> Self::Result {
        Ok(self
            .network
            .drones_node()
            .iter()
            .map(|division| {
                let division_name = unhash_string(division.0);
                let mut map = HashMap::new();
                for drone in (*division.1).drones().iter() {
                    map.insert(*drone.0, (drone.1).1.clone());
                }

                (division_name, map)
            })
            .collect())
    }
}
