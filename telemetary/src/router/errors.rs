use crate::alligator::server::ClientType;
use crate::router::ResponseJson;
use actix::MailboxError;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum RouterError {
    InvalidRoute,
    InvalidJson,
    MissingRoute,
    MissingField(String),
    MailboxClosed,
    MailboxTimeout,
    ClientDown(ClientType),
    UnsupportedClient(ClientType),
}

impl RouterError {
    pub fn body_missing() -> Self {
        RouterError::MissingField("body".to_string())
    }
}

impl Into<ResponseJson> for RouterError {
    fn into(self) -> ResponseJson {
        match self {
            RouterError::InvalidRoute => ResponseJson {
                message: "The specified route doesn't exist".to_string(),
            },
            RouterError::InvalidJson => ResponseJson {
                message: "Invalid json specified".to_string(),
            },
            RouterError::MissingField(field) => ResponseJson {
                message: format!("Field `{}` is missing.", field),
            },
            RouterError::MissingRoute => ResponseJson {
                message: "Param `route` is missing.".to_string(),
            },
            RouterError::ClientDown(client) => ResponseJson {
                message: match client {
                    ClientType::Drone { .. } => format!("Drone: {:?} is down", client),
                    ClientType::Pilot { .. } => format!("Pilot: {:?} is down", client),
                },
            },
            RouterError::UnsupportedClient(client) => ResponseJson {
                message: format!("{:?} is down", client),
            },
            RouterError::MailboxClosed => ResponseJson {
                message: "Client mailbox is closed.".to_string(),
            },
            RouterError::MailboxTimeout => ResponseJson {
                message: "Client mailbox is currently down".to_string(),
            },
        }
    }
}

impl Into<actix_web::Binary> for ResponseJson {
    fn into(self) -> actix_web::Binary {
        actix_web::Binary::from(serde_json::to_string(&self).unwrap())
    }
}

impl From<RouterError> for actix_web::Binary {
    fn from(err: RouterError) -> Self {
        let e: ResponseJson = err.into();
        e.into()
    }
}

impl From<MailboxError> for RouterError {
    fn from(err: MailboxError) -> Self {
        match err {
            MailboxError::Closed => RouterError::MailboxClosed,
            MailboxError::Timeout => RouterError::MailboxTimeout,
        }
    }
}
