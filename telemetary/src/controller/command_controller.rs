use crate::{
    alligator::swarm::{SendCommandToDrones, SendCommandToPilots},
    router::{ResponseJson, RouterError},
    AlligatorServer, AlligatorServerState, ClientType,
};
use actix_web::ws::WebsocketContext;
use serde_derive::Deserialize;
use serde_json::{from_value, Value};

#[derive(Copy, Clone, Deserialize)]
#[allow(dead_code)]
enum CommandType {
    Land,
    Navigate { lat: i32, long: i32, altitude: i16 },
}

#[derive(Clone, Deserialize)]
struct Command {
    division_name: String,
    message: String,
}

pub(crate) fn send_command(
    req: Value,
    client: &ClientType,
    ctx: &WebsocketContext<AlligatorServer, AlligatorServerState>,
) -> Result<ResponseJson, RouterError> {
    let command = from_value::<Command>(req).unwrap();
    let swarm_address = &ctx.state().address;

    match client {
        ClientType::Pilot { .. } => Ok(swarm_address
            .try_send(SendCommandToDrones {
                division_name: command.division_name.clone(),
                message: command.message.clone(),
                skip_id: None,
            })
            .map_err(|_| RouterError::DroneDown {
                client: client.clone(), // improve this later (don't clone)
            })
            .and_then(|_| {
                Ok(ResponseJson {
                    message: "Message sent".to_string(),
                })
            })?),

        ClientType::Drone { .. } => Ok(swarm_address
            .try_send(SendCommandToPilots {
                message: command.message.clone(),
                skip_id: None,
            })
            .map_err(|_| RouterError::PilotDown {
                client: client.clone(), // improve this later
            })
            .and_then(|_| {
                Ok(ResponseJson {
                    message: "Message sent".to_string(),
                })
            })?),
    }
}
