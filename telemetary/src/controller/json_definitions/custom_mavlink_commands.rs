use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum CustomCommands {
    #[serde(alias = "clear_mission", rename = "clear_mission")]
    ClearMission,

    #[serde(alias = "land", rename = "land")]
    Land,

    #[serde(alias = "navigate", rename = "navigate")]
    Navigate { lat: i32, lng: i32, altitude: i16 },
}
