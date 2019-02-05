use crate::alligator::{
    constants::{CLIENT_TYPE_HEADER_KEY, FAKE_PILOT_CLIENT_HASH},
    server::{AlligatorServer, ClientType},
};
use actix::Actor;
use rand::Rng;
use std::fmt;

pub(crate) enum HeaderError<T> {
    MissingHeaderKey(T),
    InvalidHeaderKey(T),
    InvalidHeaderValue(T),
}

impl<T> fmt::Display for HeaderError<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HeaderError::MissingHeaderKey(v) => write!(f, "Header value `{}` is missing", v),
            HeaderError::InvalidHeaderKey(v) => {
                write!(f, "Header key `{}` is in an incorrect format", v)
            }
            HeaderError::InvalidHeaderValue(v) => {
                write!(f, "Header value for `{}` is in an incorrect format", v)
            }
        }
    }
}

// Identify the client type of the connection, i.e. if it is a drone or a pilot.
pub(crate) fn extract_client_type(
    ctx: &mut <AlligatorServer as Actor>::Context,
) -> Result<ClientType, HeaderError<String>> {
    let value = ctx
        .request()
        .headers()
        .get(CLIENT_TYPE_HEADER_KEY)
        .ok_or_else(|| HeaderError::MissingHeaderKey(CLIENT_TYPE_HEADER_KEY.to_string()))?
        .to_str()
        .map_err(|_| HeaderError::InvalidHeaderKey(CLIENT_TYPE_HEADER_KEY.to_string()))?
        .to_lowercase();

    match value.as_ref() {
        "drone" => Ok(ClientType::Drone {
            // Todo: Get owner hash from header and validate in the database.
            owner_hash: FAKE_PILOT_CLIENT_HASH.to_string(),

            // Todo: get this from the header in production.
            hash: rand::thread_rng().gen::<usize>().to_string(),

            // General is the default channel drone is connected to by default
            division_name: "General".to_string(),
        }),

        "pilot" => Ok(ClientType::Pilot {
            hash: FAKE_PILOT_CLIENT_HASH.to_string(),
        }),

        _ => Err(HeaderError::InvalidHeaderValue(
            CLIENT_TYPE_HEADER_KEY.to_string(),
        )),
    }
}
