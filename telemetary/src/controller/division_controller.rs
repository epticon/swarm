use crate::{
    alligator::swarm::{CreateDivision, DeleteDivision, GetAllDivisions, SendCommandToPilots},
    constants::pilot_routes,
    controller::{serialize_value, AlligatorSocketContext},
    router::Body,
    router::{ResponseJson, RouterError},
    ClientType,
};
use futures::future::Future;
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CreateDivisionRq(String);

#[derive(Deserialize, Serialize)]
struct DeleteDivisionRq(String);

pub(crate) fn delete(
    body: Body,
    client: &ClientType,
    ctx: &mut AlligatorSocketContext,
) -> Result<ResponseJson, RouterError> {
    let body = body
        .content()
        .ok_or_else(RouterError::body_missing)?
        .clone();

    let delete = &serialize_value::<DeleteDivisionRq>(body)?;

    match client {
        ClientType::Pilot { .. } => {
            ctx.state()
                .address
                .try_send(DeleteDivision(delete.0.to_string()))
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

pub(crate) fn create(
    body: Body,
    client: &ClientType,
    ctx: &mut AlligatorSocketContext,
) -> Result<ResponseJson, RouterError> {
    let body = body
        .content()
        .ok_or_else(RouterError::body_missing)?
        .clone();

    let create = &serialize_value::<CreateDivisionRq>(body)?;

    match client {
        ClientType::Pilot { .. } => {
            ctx.state()
                .address
                .try_send(CreateDivision(create.0.to_string()))
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

pub(crate) fn get_all(
    _: Body,
    client: &ClientType,
    ctx: &mut AlligatorSocketContext,
) -> Result<ResponseJson, RouterError> {
    match client {
        ClientType::Pilot { .. } => {
            let divisions = ctx
                .state()
                .address
                .send(GetAllDivisions)
                .map_err(|s| s.into())
                .and_then(|res| res.map_err(|_| RouterError::ClientDown(client.clone())))
                .wait()?;

            Ok(ResponseJson::from(&stringify_response(
                &divisions,
                pilot_routes::DIVISIONS,
            )))
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
