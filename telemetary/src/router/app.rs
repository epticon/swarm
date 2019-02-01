use crate::alligator::server::AlligatorServer;
use crate::alligator::server::AlligatorServerState;
use crate::alligator::server::ClientType;
use crate::controller::command_controller;
use crate::router::{ResponseJson, Router};
use actix_web::ws::WebsocketContext;

type Response = WebsocketContext<AlligatorServer, AlligatorServerState>;

pub(crate) fn get_routes() -> Router<ResponseJson, ClientType, Response> {
    let mut router = Router::default();
    router.add_route("/command", command_controller::send_command);

    router
}
