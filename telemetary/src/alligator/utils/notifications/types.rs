use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) enum NotificationTypes {
    #[serde(rename = "DRONES_DOWN")]
    DronesDown,
}
