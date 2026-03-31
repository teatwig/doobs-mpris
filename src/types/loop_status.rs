use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Type, Value};

use crate::{Error, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopStatus {
    /// The playback will stop when there are no more tracks to play
    None,
    /// The current track will start again from the begining once it has finished playing
    Track,
    /// The playback loops through a list of tracks
    Playlist,
}

impl FromStr for LoopStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().trim() {
            "none" => Ok(Self::None),
            "track" => Ok(Self::Track),
            "playlist" => Ok(Self::Playlist),
            _ => Err(Error::InvalidEnum {
                got: s.to_string(),
                expected: &["Playing", "Paused", "Stopped"],
            }),
        }
    }
}

impl Display for LoopStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None",
                Self::Track => "Track",
                Self::Playlist => "Playlist",
            }
        )
    }
}

// traits necessary to use it as a type in zbus

impl Type for LoopStatus {
    const SIGNATURE: &'static zvariant::Signature = &zvariant::Signature::Str;
}

impl From<LoopStatus> for Value<'_> {
    fn from(value: LoopStatus) -> Self {
        Value::from(value.to_string())
    }
}

impl TryFrom<Value<'_>> for LoopStatus {
    type Error = zvariant::Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let value = value.downcast_ref::<&str>()?;
        let value = LoopStatus::from_str(value)?;
        Ok(value)
    }
}

impl TryFrom<OwnedValue> for LoopStatus {
    type Error = zvariant::Error;

    fn try_from(value: OwnedValue) -> std::result::Result<Self, Self::Error> {
        let value = value.downcast_ref::<&str>()?;
        let value = LoopStatus::from_str(value)?;
        Ok(value)
    }
}
