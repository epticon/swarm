use crate::{
    alligator::swarm::SendCommandToPilots,
    constants::pilot_routes,
    controller::{serialize_value, AlligatorSocketContext},
    router::{Body, ResponseJson, RouterError},
    ClientType,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
enum TelemetaryTypes {}

#[derive(Deserialize, Serialize)]
struct Telemetary(TelemetaryTypes);

pub(crate) fn send_telemetary(
    body: Body,
    client: &ClientType,
    ctx: &AlligatorSocketContext,
) -> Result<ResponseJson, RouterError> {
    let body = body
        .content()
        .ok_or_else(RouterError::body_missing)?
        .clone();

    match client {
        ClientType::Drone { .. } => {
            let message = SendCommandToPilots {
                skip_id: None,
                message: stringify_telemetary(serialize_value::<Telemetary>(body)?),
            };

            ctx.state()
                .address
                .try_send(message)
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::message_sent())
        }

        _ => Err(RouterError::UnsupportedClient(client.clone())),
    }
}

fn stringify_telemetary(telemetary: Telemetary) -> String {
    serde_json::json!({
        "route": pilot_routes::TELEMETARY,
        "telemetary": telemetary
    })
    .to_string()
}
