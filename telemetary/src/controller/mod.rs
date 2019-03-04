pub(crate) mod command_controller;
pub(crate) mod telemetary_controller;
pub(crate) mod division_controller;

use crate::{
    alligator::server::AlligatorServer, alligator::server::AlligatorServerState,
    router::RouterError,
};
use actix_web::ws::WebsocketContext;
use serde::Deserialize;
use serde_json::{from_value, Value};

type AlligatorSocketContext = WebsocketContext<AlligatorServer, AlligatorServerState>;

fn serialize_value<T>(data: Value) -> Result<T, RouterError>
where
    for<'de> T: Deserialize<'de>,
{
    Ok(from_value::<T>(data).map_err(|e| {
        // Leave this println!(), in case of fields that are yet to be serialized
        println!("{:?}", e);
        RouterError::InvalidJson
    })?)
}
