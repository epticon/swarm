use crate::{
    alligator::swarm::{
        CreateDivision as CreateDivisionCommand, DeleteDivision as DeleteDivisionCommand,
        SendCommandToPilots,
    },
    constants::pilot_routes,
    controller::{serialize_value, AlligatorSocketContext},
    router::Body,
    router::{ResponseJson, RouterError},
    ClientType,
};
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CreateDivision(String);

#[derive(Deserialize, Serialize)]
struct DeleteDivision(String);

pub(crate) fn delete_division(
    body: Body,
    client: &ClientType,
    ctx: &AlligatorSocketContext,
) -> Result<ResponseJson, RouterError> {
    let body = body
        .content()
        .ok_or_else(RouterError::body_missing)?
        .clone();

    let delete = &serialize_value::<DeleteDivision>(body)?;

    match client {
        ClientType::Pilot { .. } => {
            ctx.state()
                .address
                .try_send(DeleteDivisionCommand(delete.0.to_string()))
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            ctx.state()
                .address
                .try_send(SendCommandToPilots {
                    skip_id: None,
                    message: stringify_response(&delete, pilot_routes::DELETE_DIVISION),
                })
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::message_sent())
        }

        _ => Err(RouterError::UnsupportedClient(client.to_owned())),
    }
}

pub(crate) fn create_division(
    body: Body,
    client: &ClientType,
    ctx: &AlligatorSocketContext,
) -> Result<ResponseJson, RouterError> {
    let body = body
        .content()
        .ok_or_else(RouterError::body_missing)?
        .clone();

    let create = &serialize_value::<CreateDivision>(body)?;

    match client {
        ClientType::Pilot { .. } => {
            ctx.state()
                .address
                .try_send(CreateDivisionCommand(create.0.to_string()))
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            ctx.state()
                .address
                .try_send(SendCommandToPilots {
                    skip_id: None,
                    message: stringify_response(&create, pilot_routes::CREATE_DIVISION),
                })
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::message_sent())
        }

        _ => Err(RouterError::UnsupportedClient(client.to_owned())),
    }
}

fn stringify_response<T: Serialize>(command: &T, route: &str) -> String {
    serde_json::json!({
        "route": route,
        "response": &command
    })
    .to_string()
}
