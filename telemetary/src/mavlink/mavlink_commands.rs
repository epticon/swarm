use crate::mavlink::mavlink_code::*;
use serde::{Serialize, Serializer};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum MavLinkCommands {
    // Custom
    Clear,

    // Navigation commands
    Waypoint {
        delay: String,
        yaw_angle: String,
        lat: String,
        long: String,
        alt: String,
    },
    SplineWaypoint {
        delay: String,
        lat: String,
        long: String,
        alt: String,
    },
    LoiterUnlim {
        lat: String,
        long: String,
        alt: String,
    },
    LoiterTurns {
        turn: String,
        dir: String,
        lat: String,
        long: String,
        alt: String,
    },
    LoiterTime {
        times: String,
        lat: String,
        long: String,
        alt: String,
    },
    ReturnToLaunch,
    Land {
        lat: String,
        long: String,
    },
    Takeoff {
        alt: String,
    },

    // Condition commands
    ConditionDelay {
        time: String,
    },
    ConditionYaw {
        angle: String,
        speed: String,
        direction: String,
        absolute_angle: String,
    },
    ConditionDistance {
        distance: String,
    },

    // DO Commands
    DoJump {
        wp: String,
        repeat: String,
    },
    DoSetMode {
        mode: String,
    },
    DoChangeSpeed {
        speed_type: String,
        speed: String,
        throttle: String,
        absolute_or_relative: String,
    },
    DoSetHome {
        current: String,
        lat: String,
        long: String,
        alt: String,
    },
    DoSetServo {
        ser_no: String,
        pwm: String,
    },
    DoRepeatServo {
        ser_no: String,
        pwm: String,
        repeat: String,
        delay: String,
    },
    DoDigicamControl {
        on_or_off: String,
        focus_lock: String,
        shutter_cmd: String,
    },
    DoDigicamConfigure {
        mode: String,
        shutter_speed: String,
        aperture: String,
        iso: String,
        engine_cut_off: String,
    },
    DoSetRelay {
        relay_no: String,
        state: String,
    },
    DoRepeatRelay {
        relay_no: String,
        repeat: String,
        delay: String,
    },
    DoSetCamTriggDist {
        distance: String,
    },
    DoMountControl {
        pitch: String,
        roll: String,
        yaw: String,
    },
}

impl Serialize for MavLinkCommands {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            MavLinkCommands::Clear => {
                serializer.serialize_str(&format!("{}", CUSTOM_MAV_CMD_CLEAR))
            }

            MavLinkCommands::Waypoint {
                ref delay,
                ref yaw_angle,
                ref lat,
                ref long,
                ref alt,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t0\t0\t{}\t{}\t{}\t{}",
                MAV_CMD_NAV_WAYPOINT, delay, yaw_angle, lat, long, alt
            )),

            MavLinkCommands::SplineWaypoint {
                ref delay,
                ref lat,
                ref long,
                ref alt,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t0\t0\t0\t{}\t{}\t{}",
                MAV_CMD_NAV_SPLINE_WAYPOINT, delay, lat, long, alt
            )),

            MavLinkCommands::LoiterUnlim {
                ref lat,
                ref long,
                ref alt,
            } => serializer.serialize_str(&format!(
                "{}\t0\t0\t0\t0\t{}\t{}\t{}",
                MAV_CMD_NAV_LOITER_UNLIM, lat, long, alt
            )),

            MavLinkCommands::LoiterTurns {
                ref turn,
                ref dir,
                ref lat,
                ref long,
                ref alt,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t0\t{}\t0\t{}\t{}\t{}",
                MAV_CMD_NAV_LOITER_TURNS, turn, dir, lat, long, alt
            )),

            MavLinkCommands::LoiterTime {
                ref times,
                ref lat,
                ref long,
                ref alt,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t0\t0\t0\t{}\t{}\t{}",
                MAV_CMD_NAV_LOITER_TIME, times, lat, long, alt
            )),

            MavLinkCommands::ReturnToLaunch => serializer.serialize_str(&format!(
                "{}\t0\t0\t0\t0\t0\t0\t0",
                MAV_CMD_NAV_RETURN_TO_LAUNCH
            )),

            MavLinkCommands::Land { ref lat, ref long } => serializer.serialize_str(&format!(
                "{}\t0\t0\t0\t0\t{}\t{}\t0",
                MAV_CMD_NAV_LAND, lat, long
            )),

            MavLinkCommands::Takeoff { ref alt } => serializer.serialize_str(&format!(
                "{}\t0\t0\t0\t0\t0\t0\t{}",
                MAV_CMD_NAV_TAKEOFF, alt
            )),

            // Condition commands
            MavLinkCommands::ConditionDelay { ref time } => serializer.serialize_str(&format!(
                "{}\t{}\t0\t0\t0\t0\t0\t0",
                MAV_CMD_CONDITION_DELAY, time
            )),

            MavLinkCommands::ConditionYaw {
                ref angle,
                ref speed,
                ref direction,
                ref absolute_angle,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t0\t0\t0",
                MAV_CMD_CONDITION_YAW, angle, speed, direction, absolute_angle
            )),

            MavLinkCommands::ConditionDistance { ref distance } => {
                serializer.serialize_str(&format!(
                    "{}\t{}\t0\t0\t0\t0\t0\t0",
                    MAV_CMD_CONDITION_DISTANCE, distance
                ))
            }

            // DO Commands
            MavLinkCommands::DoJump { ref wp, ref repeat } => serializer.serialize_str(&format!(
                "{}\t{}\t{}\t0\t0\t0\t0\t0",
                MAV_CMD_DO_JUMP, wp, repeat
            )),

            MavLinkCommands::DoSetMode { ref mode } => serializer.serialize_str(&format!(
                "{}\t{}\t0\t0\t0\t0\t0\t0",
                MAV_CMD_DO_SET_MODE, mode
            )),

            MavLinkCommands::DoChangeSpeed {
                ref speed_type,
                ref speed,
                ref throttle,
                ref absolute_or_relative,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t0\t0\t0",
                MAV_CMD_DO_CHANGE_SPEED, speed_type, speed, throttle, absolute_or_relative
            )),

            MavLinkCommands::DoSetHome {
                ref current,
                ref lat,
                ref long,
                ref alt,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t0\t0\t0\t{}\t{}\t{}",
                MAV_CMD_DO_SET_HOME, current, lat, long, alt
            )),

            MavLinkCommands::DoSetServo {
                ref ser_no,
                ref pwm,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t{}\t0\t0\t0\t0\t0",
                MAV_CMD_DO_SET_SERVO, ser_no, pwm
            )),

            MavLinkCommands::DoRepeatServo {
                ref ser_no,
                ref pwm,
                ref repeat,
                ref delay,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t0\t0\t0",
                MAV_CMD_DO_REPEAT_SERVO, ser_no, pwm, repeat, delay
            )),

            MavLinkCommands::DoDigicamControl {
                ref on_or_off,
                ref focus_lock,
                ref shutter_cmd,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t0\t0\t{}\t{}\t0\t0",
                MAV_CMD_DO_DIGICAM_CONTROL, on_or_off, focus_lock, shutter_cmd
            )),

            MavLinkCommands::DoDigicamConfigure {
                ref mode,
                ref shutter_speed,
                ref aperture,
                ref iso,
                ref engine_cut_off,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t0\t0\t{}",
                MAV_CMD_DO_DIGICAM_CONFIGURE, mode, shutter_speed, aperture, iso, engine_cut_off
            )),

            MavLinkCommands::DoSetRelay {
                ref relay_no,
                ref state,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t{}\t0\t0\t0\t0\t0",
                MAV_CMD_DO_SET_RELAY, relay_no, state
            )),

            MavLinkCommands::DoRepeatRelay {
                ref relay_no,
                ref repeat,
                ref delay,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t{}\t{}\t0\t0\t0\t0",
                MAV_CMD_DO_REPEAT_RELAY, relay_no, repeat, delay
            )),

            MavLinkCommands::DoSetCamTriggDist { ref distance } => {
                serializer.serialize_str(&format!(
                    "{}\t{}\t0\t0\t0\t0\t0\t0",
                    MAV_CMD_DO_SET_CAM_TRIGG_DIST, distance
                ))
            }

            MavLinkCommands::DoMountControl {
                ref pitch,
                ref roll,
                ref yaw,
            } => serializer.serialize_str(&format!(
                "{}\t{}\t{}\t{}\t0\t0\t0\t0",
                MAV_CMD_DO_MOUNT_CONTROL, pitch, roll, yaw
            )),
        }
    }
}
