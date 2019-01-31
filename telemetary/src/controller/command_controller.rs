use crate::router::{ResponseJson, RouterError};
use serde_derive::Deserialize;
use serde_json::{from_value, Value};

#[derive(Copy, Clone, Deserialize)]
struct Command {
    // client:
}

pub(crate) fn send_command<C, W>(
    req: Value,
    client: &C,
    _ctx: &W,
) -> Result<ResponseJson, RouterError> {
    let command = from_value::<Command>(req);

    Ok(ResponseJson {
        message: "Success".to_string(),
    })
}
