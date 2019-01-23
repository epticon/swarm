use crate::alligator::swarm::{Connect, Disconnect, Message, Swarm};
use crate::router::{RequestJson, ResponseJson, Router, RouterError};
use actix::fut;
use actix::prelude::{
    Actor, ActorContext, ActorFuture, Addr, AsyncContext, ContextFutureSpawner, Handler,
    StreamHandler, WrapFuture,
};
use actix_web::ws;
use rand::Rng;
use serde_json::Result as JsonResult;
use std::time::{Duration, Instant};

// Header key that signifies the type of the client making a connection to
// the alligator server. The supported types are `Pilot` and `Drone`.
const CLIENT_TYPE_HEADER_KEY: &str = "Alligator-Client-Type";

// Maximum time of inactivity before a client response reports a timeout.
const MAX_CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// How often are heartbeat pings sent to client.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(4);

// Todo: Remove this and replace with hash retrieved from database.
const FAKE_PILOT_CLIENT_HASH: &str = "23hbch2r2jhdb2hjfb2j3hb2jlfhbjfbh2jfb2jfhbjrbjh1fb";

#[derive(Debug)]
pub(crate) enum ClientType {
    // Pilot are owners of a swarm.
    Pilot {
        hash: String,
    },

    Drone {
        hash: String,
        owner_hash: String,
        division_name: String,
    },
}

pub(crate) struct AlligatorServerState {
    pub address: Addr<Swarm>,
    pub router: Router<ResponseJson>,
}

pub(crate) struct AlligatorServer {
    // Unique session id.
    session_id: usize,
    last_heartbeat_time: Instant,
    client_type: Option<ClientType>,
}

impl Default for AlligatorServer {
    fn default() -> Self {
        AlligatorServer {
            last_heartbeat_time: Instant::now(),
            session_id: 0,
            client_type: None,
        }
    }
}

impl AlligatorServer {
    // Starts the process of heartbeat. The heartbeat  is simply a helper function
    // that pings the pilot or drone clients connected.
    fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self, AlligatorServerState>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat_time) > MAX_CLIENT_TIMEOUT {
                println!("Disconnecting client because heartbeat failed.");

                let client_type = &mut act.client_type;

                ctx.state().address.do_send(Disconnect {
                    session_id: act.session_id,
                    // Unwrapping is safe because no client can be created
                    // without a `client_type` with Some(_) containing all of the client information.
                    //
                    // `take()` replaces the client_type with a `None` type
                    client: client_type.take().unwrap(),
                });

                // Stops the specified actor.
                ctx.stop();

                // Prevent any further ping.
                return;
            }

            ctx.ping("");
        });
    }

    // Identify the client type of the connection, i.e. if it is a drone or a pilot.
    fn client_type(
        &self,
        ctx: &mut ws::WebsocketContext<Self, AlligatorServerState>,
    ) -> Option<ClientType> {
        ctx.request()
            .headers()
            .get(CLIENT_TYPE_HEADER_KEY)
            .and_then(|value| match value.to_str().ok()?.to_lowercase().as_ref() {
                "drone" => Some(ClientType::Drone {
                    // Todo: Get owner hash from header and validate in the database.
                    owner_hash: FAKE_PILOT_CLIENT_HASH.to_string(),

                    // Todo: get this from the header in production.
                    hash: rand::thread_rng().gen::<usize>().to_string(),

                    // General is the default channel drone is connected to by default
                    division_name: "General".to_string(),
                }),

                "pilot" => Some(ClientType::Pilot {
                    hash: FAKE_PILOT_CLIENT_HASH.to_string(),
                }),

                _ => None,
            })
    }
}

impl Handler<Message> for AlligatorServer {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0)
    }
}

impl Actor for AlligatorServer {
    type Context = ws::WebsocketContext<Self, AlligatorServerState>;

    // Todo:
    // On swarm owner connecting (i.e. this started function ), we add register the owner
    // to the network. The owner can then manage their respective division/nodes.
    // Before registering the owner, check if any of the drones have been connected, and
    // if connected, then surely, the client node has been created, just simply
    // add the ownder address to the connection slot.
    //
    // On the drone connected, register the drone to their owner/pilots swarm. If
    // the pilot hasn't been logged in/connected, chck the database to find out
    // the owners unique_id (which is the ownser) and the swarm_id/node_id of the
    // owner.
    //
    //
    // A collection of nodes is what forms the swarm.
    fn started(&mut self, ctx: &mut Self::Context) {
        // Identify if the client connecting is a drone or a pilot.
        if let Some(client_type) = self.client_type(ctx) {
            self.start_heartbeat(ctx);

            // Todo: Get the pilot client id from the database in production version.
            // Pending that time, use the `FAKE_PILOT_CLIENT_HASH` value.

            let addr = ctx.address();

            ctx.state()
                .address
                .send(Connect {
                    client_id: FAKE_PILOT_CLIENT_HASH.to_string(),
                    client_type,
                    address: addr.recipient(),
                })
                .into_actor(self)
                .then(|res, act, ctx| {
                    match res {
                        Ok(res) => act.session_id = res,
                        Err(err) => {
                            println!("{}", err);
                            ctx.stop();
                        }
                    }

                    fut::ok(())
                })
                .wait(ctx);
        } else {
            ctx.close(Some(ws::CloseReason {
                code: ws::CloseCode::Invalid,
                description: Some("`Alligator-Client-Type` header value is missing.".to_string()),
            }));
        }
    }
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
                        let callback = ctx.state().router.match_route(&json.path());

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
