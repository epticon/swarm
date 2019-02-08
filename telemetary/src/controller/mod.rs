pub(crate) mod command_controller;
pub(crate) mod telemetary_controller;

use crate::alligator::server::AlligatorServer;
use crate::alligator::server::AlligatorServerState;
use crate::router::RouterError;
use actix_web::ws::WebsocketContext;
use serde::Deserialize;
use serde_json::from_value;
use serde_json::Value;

type AlligatorSocketContext = WebsocketContext<AlligatorServer, AlligatorServerState>;

fn serialize_value<T>(data: Value) -> Result<T, RouterError>
where
  for<'de> T: Deserialize<'de>,
{
  Ok(from_value::<T>(data).map_err(|_| RouterError::InvalidJson)?)
}
