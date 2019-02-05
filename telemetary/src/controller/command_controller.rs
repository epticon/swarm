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
    data: Option<Value>,
    client: &ClientType,
    ctx: &WebsocketContext<AlligatorServer, AlligatorServerState>,
) -> Result<ResponseJson, RouterError> {
    let swarm_address = &ctx.state().address;

    let command = from_value::<Command>(data.ok_or_else(RouterError::missing_data_field)?)
        .map_err(|_| RouterError::InvalidJson)?;

    match client {
        ClientType::Pilot { .. } => {
            swarm_address
                .try_send(SendCommandToDrones {
                    division_name: command.division_name.clone(),
                    message: command.message.clone(),
                    skip_id: None,
                })
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::message_sent())
        }

        ClientType::Drone { .. } => {
            swarm_address
                .try_send(SendCommandToPilots {
                    message: command.message.clone(),
                    skip_id: None,
                })
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::message_sent())
        }
    }
}
