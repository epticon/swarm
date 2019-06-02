use crate::{
    alligator::swarm::{
        ChangeDivision, CreateDivision, DeleteDivision, GetAllDivisionNames, GetAllDivisions,
        SendCommandToPilots,
    },
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

#[derive(Deserialize, Serialize)]
struct ChangeDivisionRq {
    to: String,
    from: String,
    drone_session: usize,
}

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

            // Broadcast deletion to all pilots, to enable pilots have an
            // update-to-date info of the swarm locally.
            ctx.state()
                .address
                .try_send(SendCommandToPilots {
                    skip_id: None,
                    message: stringify_response(&delete, pilot_routes::DELETE_DIVISION),
                })
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::new(&format!(
                "Division {:?} deleted successfully.",
                delete.0
            )))
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

            // Broadcast division creating to all pilots, to enable pilots have an
            // update-to-date info of the swarm locally.
            ctx.state()
                .address
                .try_send(SendCommandToPilots {
                    skip_id: None,
                    message: stringify_response(&create, pilot_routes::CREATE_DIVISION),
                })
                .map_err(|_| RouterError::ClientDown(client.clone()))?;

            Ok(ResponseJson::new(&format!(
                "Division {:?} created successfully.",
                create.0
            )))
        }

        _ => Err(RouterError::UnsupportedClient(client.to_owned())),
    }
}

pub(crate) fn get_all_division_names(
    _: Body,
    client: &ClientType,
    ctx: &mut AlligatorSocketContext,
) -> Result<ResponseJson, RouterError> {
    match client {
        ClientType::Pilot { .. } => {
            let divisions = ctx
                .state()
                .address
                .send(GetAllDivisionNames)
                .map_err(std::convert::Into::into)
                .and_then(|res| res.map_err(|_| RouterError::ClientDown(client.clone())))
                .wait()?;

            // Send only to the client who initiated the request
            // to get all division names
            Ok(ResponseJson::new(&stringify_response(
                &divisions,
                pilot_routes::DIVISION_NAMES,
            )))
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
                .map_err(std::convert::Into::into)
                .and_then(|res| res.map_err(|_| RouterError::ClientDown(client.clone())))
                .wait()?;

            // Send only to the client who initiated the request
            // to get all division names.
            Ok(ResponseJson::new(&stringify_response(
                &divisions,
                pilot_routes::DIVISIONS,
            )))
        }

        _ => Err(RouterError::UnsupportedClient(client.to_owned())),
    }
}

pub(crate) fn change_division(
    body: Body,
    client: &ClientType,
    ctx: &mut AlligatorSocketContext,
) -> Result<ResponseJson, RouterError> {
    let body = body
        .content()
        .ok_or_else(RouterError::body_missing)?
        .clone();

    let info = &serialize_value::<ChangeDivisionRq>(body)?;

    match client {
        ClientType::Pilot { .. } => {
            let msg = ctx
                .state()
                .address
                .send(ChangeDivision {
                    to: info.to.to_string(),
                    from: info.from.to_string(),
                    drone_session: info.drone_session,
                })
                .map_err(std::convert::Into::into)
                .and_then(|res| {
                    res.map_err(|e| {
                        println!("{}", e);
                        RouterError::ClientDown(client.clone())
                    })
                })
                .wait()?;

            // Send only to the client who initiated the request
            // to get all division names.
            Ok(ResponseJson::new(&stringify_response(
                &msg,
                pilot_routes::CHANGE_DIVISION,
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
