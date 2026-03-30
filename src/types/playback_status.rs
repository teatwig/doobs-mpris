use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Type, Value};

use crate::{Error, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybackStatus {
    /// A track is currently playing.
    Playing,
    /// A track is currently paused.
    Paused,
    /// There is no track currently playing.
    Stopped,
}

impl FromStr for PlaybackStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().trim() {
            "playing" => Ok(Self::Playing),
            "paused" => Ok(Self::Paused),
            "stopped" => Ok(Self::Stopped),
            _ => Err(Error::InvalidEnum {
                got: s.to_string(),
                expected: &["Playing", "Paused", "Stopped"],
            }),
        }
    }
}

impl Display for PlaybackStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Playing => "Playing",
                Self::Paused => "Paused",
                Self::Stopped => "Stopped",
            }
        )
    }
}

// traits necessary to use it as a type in zbus

impl Type for PlaybackStatus {
    const SIGNATURE: &'static zvariant::Signature = &zvariant::Signature::Str;
}

impl From<PlaybackStatus> for Value<'_> {
    fn from(value: PlaybackStatus) -> Self {
        Value::from(value.to_string())
    }
}

impl TryFrom<Value<'_>> for PlaybackStatus {
    type Error = zvariant::Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let value: &str = value.downcast_ref::<&str>()?;
        let value = PlaybackStatus::from_str(value)
            .map_err(|err| zvariant::Error::Message(err.to_string()))?;
        Ok(value)
    }
}

impl TryFrom<OwnedValue> for PlaybackStatus {
    type Error = zvariant::Error;

    fn try_from(value: OwnedValue) -> std::result::Result<Self, Self::Error> {
        let value = value.downcast_ref::<&str>()?;
        let value = PlaybackStatus::from_str(value)
            .map_err(|err| zvariant::Error::Message(err.to_string()))?;
        Ok(value)
    }
}
