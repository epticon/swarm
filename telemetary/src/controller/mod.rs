use crate::router::RequestJson;
use actix_web::Error;

pub(crate) fn _get_index(_req: &RequestJson) -> Result<String, Error> {
    Ok(String::new())
}
