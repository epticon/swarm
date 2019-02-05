use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RequestJson {
    path: String,
    data: Option<Value>,
}

impl RequestJson {
    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    pub(crate) fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }
}
