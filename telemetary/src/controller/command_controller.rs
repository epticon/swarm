use crate::{
    alligator::swarm::SendCommandToDrones,
    constants::drone_routes,
    controller::{serialize_value, AlligatorSocketContext},
    mavlink::MavLinkCommands,
    router::Body,
    router::{ResponseJson, RouterError},
    ClientType,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
enum Instruction {
    #[serde(alias = "mission", rename = "mission")]
    Mission(Vec<MavLinkCommands>),

    #[serde(alias = "clear_mission", rename = "clear_mission")]
    ClearMission,

    #[serde(alias = "land", rename = "land")]
    Land,

    #[serde(alias = "navigate", rename = "navigate")]
    Navigate { lat: i32, long: i32, altitude: i16 },
}

#[derive(Deserialize, Serialize)]
struct Command {
    division_name: String,
    command: Instruction,
}

pub(crate) fn send_command(
    body: Body,
    client: &ClientType,
    ctx: &mut AlligatorSocketContext,
) -> Result<ResponseJson, RouterError> {
    let body = body
        .content()
        .ok_or_else(RouterError::body_missing)?
        .clone();

    let command = &serialize_value::<Command>(body)?;

    match client {
        ClientType::Pilot { .. } => {
            let message = SendCommandToDrones {
                division_name: command.division_name.clone(),
                message: stringify_command(&command),
                skip_id: None,
            };

            ctx.state()
                .address
                .try_send(message)
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::message_sent())
        }

        _ => Err(RouterError::UnsupportedClient(client.to_owned())),
    }
}

fn stringify_command(command: &Command) -> String {
    serde_json::json!({
        "route": drone_routes::COMMAND,
        "command": &command
    })
    .to_string()
}
