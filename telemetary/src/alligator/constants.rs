use std::time::Duration;

// Header key that signifies the type of the client making a connection to
// the alligator server. The supported types are `Pilot` and `Drone`.
pub(crate) const CLIENT_TYPE_HEADER_KEY: &str = "Alligator-Client-Type";

// Maximum time of inactivity before a client response reports a timeout.
pub(crate) const MAX_CLIENT_TIMEOUT: Duration = Duration::from_secs(10); // 2 minutes

// How often are heartbeat pings sent to client.
pub(crate) const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(4);

// Todo: Remove this and replace with hash retrieved from database.
pub(crate) const FAKE_PILOT_CLIENT_HASH: &str = "$$$$$";
