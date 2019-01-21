use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RequestJson {
    path: String,
    command: String,
}

impl RequestJson {
    pub(crate) fn path(&self) -> &str {
        &self.path
    }
}
