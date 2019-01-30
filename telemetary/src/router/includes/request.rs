use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RequestJson {
    path: String,
    data: Value,
}

impl RequestJson {
    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    pub(crate) fn data(&self) -> &Value {
        &self.data
    }
}
