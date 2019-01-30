use crate::alligator::server::{AlligatorServer, AlligatorServerState};
use crate::router::{RequestJson, ResponseJson, RouterError};
use actix_web::ws::WebsocketContext;

// struct Command {
//     client:
// }

pub(crate) fn send_command(
    _req: &RequestJson,
    _ctx: &WebsocketContext<AlligatorServer, AlligatorServerState>,
) -> Result<ResponseJson, RouterError> {
    Ok(ResponseJson {
        message: "Success".to_string(),
    })
}
