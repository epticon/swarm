use crate::alligator::swarm::Swarm;
use crate::router::Body;
use crate::{
    alligator::swarm::{SendCommandToDrones, SendCommandToPilots},
    router::{ResponseJson, RouterError},
    AlligatorServer, AlligatorServerState, ClientType,
};
use actix::Addr;
use actix_web::ws::WebsocketContext;
use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};

#[derive(Copy, Clone, Deserialize, Serialize)]
#[allow(dead_code)]
enum Instruction {
    Land,
    Navigate { lat: i32, long: i32, altitude: i16 },
}

#[derive(Clone, Deserialize)]
struct Command {
    division_name: String,
    instruction: Instruction,
}

pub(crate) fn send_command(
    body: Body,
    client: &ClientType,
    ctx: &WebsocketContext<AlligatorServer, AlligatorServerState>,
) -> Result<ResponseJson, RouterError> {
    let body = body
        .content()
        .ok_or_else(RouterError::missing_data_field)?
        .clone();

    process_command(
        &serialize_command::<Command>(body)?,
        client,
        &ctx.state().address,
    )
}

fn process_command(
    command: &Command,
    client: &ClientType,
    swarm_address: &Addr<Swarm>,
) -> Result<ResponseJson, RouterError> {
    match client {
        ClientType::Pilot { .. } => {
            swarm_address
                .try_send(SendCommandToDrones {
                    division_name: command.division_name.clone(),
                    message: serde_json::to_string(&command.instruction).unwrap(),
                    skip_id: None,
                })
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::message_sent())
        }

        ClientType::Drone { .. } => {
            swarm_address
                .try_send(SendCommandToPilots {
                    message: serde_json::to_string(&command.instruction).unwrap(),
                    skip_id: None,
                })
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::message_sent())
        }
    }
}

fn serialize_command<T>(data: Value) -> Result<T, RouterError>
where
    for<'de> T: Deserialize<'de>,
{
    Ok(from_value::<T>(data).map_err(|_| RouterError::InvalidJson)?)
}
