use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RequestJson {
    route: String,
    data: Option<Value>,
}

impl RequestJson {
    pub(crate) fn route(&self) -> &str {
        &self.route
    }

    pub(crate) fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }
}
