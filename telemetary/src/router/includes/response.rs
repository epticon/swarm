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

    pub fn from(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}
