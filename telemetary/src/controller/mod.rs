use crate::router::RequestJson;
use actix_web::Error;

pub(crate) fn get_index(req: &RequestJson) -> Result<String, Error> {
    Ok(String::new())
}
