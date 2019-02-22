use crate::{
    alligator::swarm::SendCommandToPilots,
    constants::pilot_routes,
    controller::{serialize_value, AlligatorSocketContext},
    router::{Body, ResponseJson, RouterError},
    ClientType,
};
use std::collections::HashMap;
// use serde_derive::{Deserialize, Serialize};

// We are serializing the telemetary data so as to reduce
// any form of latency(from serializaion). Instead, as the telemetary
// information are being generated from the drone client,
// they are forwarded untouced. The serialization and verification
// should be performed on the clients browser.
// Another alternative is to both parse the data here on the server
// and serialize to support telemetary types, before messaging pilot
// with valid JSON values.
//
// Telemetary parsing has been carried out in the copter telemetary project.
type Telemetary = HashMap<String, String>;

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
