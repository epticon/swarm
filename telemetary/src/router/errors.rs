use crate::router::ResponseJson;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum RouterError {
    InvalidRoute,
    InvalidJson,
}

impl Into<ResponseJson> for RouterError {
    fn into(self) -> ResponseJson {
        match self {
            RouterError::InvalidRoute => ResponseJson {
                message: String::from("The specified route doesn't exist"),
            },
            RouterError::InvalidJson => ResponseJson {
                message: String::from("Invalid json specified"),
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
