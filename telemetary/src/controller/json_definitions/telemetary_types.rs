
#[derive(Serialize, Deserialize)]
pub enum TelemetaryTypes {
    #[serde(rename = "last_heartbeat")]
    LastHeartbeat(Option<f64>),

    #[serde(rename = "attitude")]
    Attitude(Option<Vec<Attitude>>),

    #[serde(rename = "location_global_relative")]
    LocationGlobalRelative(Option<Vec<LocationGlobal>>),

    #[serde(rename = "location_global")]
    LocationGlobal(Option<Vec<LocationGlobal>>),

    #[serde(rename = "velocity")]
    Velocity(Option<Vec<f64>>),

    #[serde(rename = "battery")]
    Battery(Option<Vec<Battery>>),

    #[serde(rename = "heading")]
    Heading(Option<f64>),

    #[serde(rename = "airspeed")]
    Airspeed(Option<f64>),

    #[serde(rename = "groundspeed")]
    Groundspeed(Option<f64>),

    #[serde(rename = "gps_info")]
    GpsInfo(Option<Vec<GpsInfo>>),

    #[serde(rename = "location_local")]
    LocationLocal(Option<Vec<LocationLocal>>),
}

#[derive(Serialize, Deserialize)]
pub struct Attitude {
    #[serde(rename = "pitch")]
    pitch: Option<f64>,

    #[serde(rename = "yaw")]
    yaw: Option<f64>,

    #[serde(rename = "roll")]
    roll: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Battery {
    #[serde(rename = "voltage")]
    voltage: Option<f64>,

    #[serde(rename = "current")]
    current: Option<f64>,

    #[serde(rename = "level")]
    level: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct GpsInfo {
    #[serde(rename = "fix")]
    fix: Option<f64>,

    #[serde(rename = "num_sat")]
    num_sat: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct LocationGlobal {
    #[serde(rename = "lat")]
    lat: Option<f64>,

    #[serde(rename = "lon")]
    lon: Option<f64>,

    #[serde(rename = "alt")]
    alt: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct LocationLocal {
    #[serde(rename = "north")]
    north: Option<f64>,

    #[serde(rename = "east")]
    east: Option<f64>,

    #[serde(rename = "down")]
    down: Option<f64>,
}
