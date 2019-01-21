use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseJson {
    pub message: String,
}
