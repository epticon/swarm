use crate::{
    alligator::server::{AlligatorServer, AlligatorServerState, ClientType},
    constants::{drone_routes::COMMAND, pilot_routes::TELEMETARY},
    controller::{command_controller, telemetary_controller::send_telemetary},
    router::{ResponseJson, Router},
};
use actix_web::ws::WebsocketContext;

type Response = WebsocketContext<AlligatorServer, AlligatorServerState>;

pub(crate) fn get_routes() -> Router<ResponseJson, ClientType, Response> {
    let mut router = Router::default();
    router
        .add_route(COMMAND, command_controller::send_command)
        .add_route(TELEMETARY, send_telemetary);

    router
}
