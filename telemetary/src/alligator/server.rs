use crate::alligator::constants::swarm_info::{HEARTBEAT_INTERVAL, MAX_CLIENT_TIMEOUT};
use crate::alligator::swarm::{Connect, Disconnect, Message, Swarm};
use crate::alligator::utils;
use crate::router::Body;
use crate::router::{RequestJson, ResponseJson, Router, RouterError};
use actix::{
    fut,
    prelude::{
        Actor, ActorContext, ActorFuture, Addr, AsyncContext, ContextFutureSpawner, Handler,
        StreamHandler, WrapFuture,
    },
};
use actix_web::ws;
use actix_web::ws::WebsocketContext;
use actix_web::ws::WsWriter;
use serde_derive::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub router: Router<ResponseJson, ClientType, WebsocketContext<AlligatorServer, Self>>,
}

pub(crate) struct AlligatorServer {
    session_id: usize, // Unique session id.
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
    fn start_heartbeat(&self, ctx: &mut <AlligatorServer as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat_time) > MAX_CLIENT_TIMEOUT {
                println!(
                    "Disconnecting {:?} because heartbeat failed.",
                    &act.client_type
                );

                ctx.state().address.do_send(Disconnect {
                    session_id: act.session_id,
                    // Unwrapping is safe because no client can be created
                    // without a `client_type` with Some(_) containing all of the client information.
                    //
                    // `take()` replaces the client_type with a `None` type
                    client: act.client_type.take().unwrap(),
                });

                // Stops the specified actor.
                ctx.stop();

                // Prevent any further ping.
                return;
            }

            ctx.ping("");
        });
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

        match utils::extract_client_type(ctx) {
            Ok(client) => {
                self.start_heartbeat(ctx);

                ctx.state()
                    .address
                    .send(Connect {
                        client: client.clone(), // look for an improvement
                        address: ctx.address().recipient(),
                    })
                    .into_actor(self)
                    .then(|res, act, ctx| {
                        match res {
                            Ok(res) => {
                                act.session_id = res;
                                act.client_type = Some(client);
                            }

                            Err(err) => {
                                println!("{}", err);
                                ctx.stop();
                            }
                        }

                        fut::ok(())
                    })
                    .wait(ctx); // I'm not sure we should block the processing of events
            }

            Err(err) => ctx.send_close(Some(ws::CloseReason {
                code: ws::CloseCode::Invalid,
                description: Some(err.to_string()),
            })),
        }
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for AlligatorServer {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => {
                self.last_heartbeat_time = Instant::now();
                ctx.pong(&msg);
            }

            ws::Message::Pong(_) => {
                self.last_heartbeat_time = Instant::now();
            }

            ws::Message::Text(msg) => {
                let request = serde_json::from_str::<RequestJson>(&msg);

                match request {
                    // Valid json
                    Ok(json) => {
                        let callback = ctx.state().router.match_route(&json.route());

                        match callback(
                            Body::new(json.data()),
                            // Unwrapping client_type is safe, as clients cannot be
                            // created without having a client_type.
                            &self.client_type.as_ref().unwrap(),
                            ctx,
                        ) {
                            Ok(response) => ctx.text(response),
                            Err(err) => ctx.text(err),
                        }
                    }

                    // Invalid Json
                    Err(_) => ctx.text(RouterError::MissingRoute),
                }
            }

            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}
