use crate::controller::command_controller;
use crate::router::ResponseJson;
use crate::router::Router;

pub(crate) fn get_routes<C, W>() -> Router<ResponseJson, C, W> {
    let mut router = Router::default();
    router.add_route("/command", command_controller::send_command);

    router
}
