use crate::{
    alligator::swarm::SendCommandToDrones,
    constants::drone_routes,
    controller::{json_definitions::Instruction, serialize_value, AlligatorSocketContext},
    router::Body,
    router::{ResponseJson, RouterError},
    ClientType,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Command {
    division_name: String,
    instruction: Instruction,
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
        "division_name": &command.division_name,
        "instruction": &command.instruction
    })
    .to_string()
}
