use crate::router::RouterError;
use crate::router::{RequestJson, ResponseJson};

// struct Command {
//     client:
// }

pub(crate) fn send_command(_req: &RequestJson) -> Result<ResponseJson, RouterError> {
    Ok(ResponseJson {
        message: "Success".to_string(),
    })
}
