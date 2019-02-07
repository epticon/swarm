use crate::alligator::swarm::Swarm;
use crate::constants::drone_routes;
use crate::router::Body;
use crate::{
    alligator::swarm::SendCommandToDrones,
    router::{ResponseJson, RouterError},
    AlligatorServer, AlligatorServerState, ClientType,
};
use actix::Addr;
use actix_web::ws::WebsocketContext;
use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};

#[allow(dead_code)]
#[derive(Copy, Clone, Deserialize, Serialize)]
enum Instruction {
    #[serde(alias = "land", rename = "land")]
    Land,

    #[serde(alias = "navigate", rename = "navigate")]
    Navigate { lat: i32, long: i32, altitude: i16 },
}

#[derive(Clone, Deserialize, Serialize)]
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
        .ok_or_else(RouterError::body_missing)?
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
                    message: generate_json_command_message(drone_routes::COMMAND, &command),
                    skip_id: None,
                })
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::message_sent())
        }
        _ => Err(RouterError::UnsupportedClient(client.to_owned())),
    }
}

fn serialize_command<T>(data: Value) -> Result<T, RouterError>
where
    for<'de> T: Deserialize<'de>,
{
    Ok(from_value::<T>(data).map_err(|_| RouterError::InvalidJson)?)
}

fn generate_json_command_message(route: &str, command: &Command) -> String {
    serde_json::json!({
        "route": route,
        "command": &command
    })
    .to_string()
}
