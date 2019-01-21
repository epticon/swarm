use crate::alligator::swarm::Swarm;
use crate::router::{RequestJson, ResponseJson, Router, RouterError};
use actix::{Actor, Addr, StreamHandler};
use actix_web::ws;
use serde_json::Result as JsonResult;

pub(crate) struct AlligatorServerState {
    pub address: Addr<Swarm>,
}

pub(crate) struct AlligatorServer {
    router: Router<ResponseJson>,
}

impl Default for AlligatorServer {
    fn default() -> Self {
        AlligatorServer {
            router: Router::default(),
        }
    }
}

impl Actor for AlligatorServer {
    type Context = ws::WebsocketContext<Self, AlligatorServerState>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for AlligatorServer {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => {
                ctx.pong(&msg);
            }

            ws::Message::Text(msg) => {
                let request: JsonResult<RequestJson> = serde_json::from_str(&msg);

                match request {
                    // Valid json
                    Ok(json) => {
                        let callback = self.router.match_route(&json.path());

                        match callback(&json) {
                            Ok(response) => ctx.text(response),
                            Err(err) => ctx.text(err),
                        }
                    }

                    // Invalid Json
                    Err(_err) => ctx.text(RouterError::InvalidRoute),
                }
            }

            _ => ctx.pong("Invalid"),
        }
    }
}
