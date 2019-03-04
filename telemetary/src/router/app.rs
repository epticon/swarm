use crate::{
    alligator::server::{AlligatorServer, AlligatorServerState, ClientType},
    constants::{drone_routes::*, pilot_routes::*},
    controller::{command_controller, division_controller, telemetary_controller},
    router::{ResponseJson, Router},
};
use actix_web::ws::WebsocketContext;

type Response = WebsocketContext<AlligatorServer, AlligatorServerState>;

pub(crate) fn get_routes() -> Router<ResponseJson, ClientType, Response> {
    let mut router = Router::default();
    router
        // Command
        .add_route(COMMAND, command_controller::send_command)
        // Telemetary
        .add_route(TELEMETARY, telemetary_controller::send_telemetary)
        // Divisions
        .add_route(DIVISIONS, division_controller::get_all)
        .add_route(CREATE_DIVISION, division_controller::create)
        .add_route(DELETE_DIVISION, division_controller::delete);

    router
}
