use crate::controller::command_controller;
use crate::router::ResponseJson;
use crate::router::Router;

pub(crate) fn index() -> Router<ResponseJson> {
    let mut router = Router::default();
    router.add_route("/command", command_controller::send_command);

    router
}
