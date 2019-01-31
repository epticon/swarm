use crate::alligator::{
    constants::{CLIENT_TYPE_HEADER_KEY, FAKE_PILOT_CLIENT_HASH},
    server::{AlligatorServer, ClientType},
};
use actix::Actor;
use rand::Rng;

// Identify the client type of the connection, i.e. if it is a drone or a pilot.
pub(crate) fn extract_client_type(
    ctx: &mut <AlligatorServer as Actor>::Context,
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
