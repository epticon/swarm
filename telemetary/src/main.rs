use crate::alligator::{server::*, swarm::Swarm};
use actix::{Arbiter, System};
use actix_web::{server::HttpServer, ws, App, Error, HttpRequest, HttpResponse};
use dotenv::var as env;

mod alligator;
mod controller;
mod router;

fn swarm_index_route(req: &HttpRequest<AlligatorServerState>) -> Result<HttpResponse, Error> {
    ws::start(req, AlligatorServer::default())
}

fn main() {
    env_logger::init();
    let server_address = &env("APP_URL").unwrap();
    let server_port = &env("PORT").unwrap();

    let sys = System::new("Alligator Swarm");

    let swarm_server = Arbiter::start(|_| Swarm::default());
    HttpServer::new(move || {
        let state = AlligatorServerState {
            address: swarm_server.clone(),
        };

        App::with_state(state).resource("/ws/", |resource| resource.route().f(swarm_index_route))
    })
    .bind(format!("{}:{}", server_address, server_port))
    .unwrap()
    .start();

    println!("Server started at {}:{}", server_address, server_port);

    sys.run();
}
