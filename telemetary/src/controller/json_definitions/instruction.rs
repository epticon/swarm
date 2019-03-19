use crate::controller::json_definitions::custom_mavlink_commands::CustomCommands;
use crate::mavlink::MavLinkCommands;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Instruction {
    #[serde(alias = "mission", rename = "mission")]
    Mission(Vec<MavLinkCommands>),

    #[serde(alias = "single", rename = "single")]
    Single(CustomCommands),
}
