use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseJson {
    pub message: String,
}

impl ResponseJson {
    pub fn message_sent() -> Self {
        Self {
            message: "Message sent".to_string(),
        }
    }
}
