use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Type, Value};

use crate::{Error, Result};

/// A playback state.
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
        let value = PlaybackStatus::from_str(value)?;
        Ok(value)
    }
}

impl TryFrom<OwnedValue> for PlaybackStatus {
    type Error = zvariant::Error;

    fn try_from(value: OwnedValue) -> std::result::Result<Self, Self::Error> {
        let value = value.downcast_ref::<&str>()?;
        let value = PlaybackStatus::from_str(value)?;
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use zvariant::serialized::Context;
    use zvariant::{LE, OwnedValue, Value, to_bytes};

    use super::PlaybackStatus;

    #[test]
    fn success() {
        let expected = PlaybackStatus::Paused;

        // lowercase string works
        let v = Value::Str("paused".into());
        let status = PlaybackStatus::try_from(v.clone()).unwrap();

        assert_eq!(status, expected);
        assert_eq!(Value::Str("Paused".into()), Value::from(expected));

        let ov = OwnedValue::try_from(v.clone()).unwrap();
        let duration = PlaybackStatus::try_from(ov.clone()).unwrap();

        assert_eq!(duration, expected);
    }

    #[test]
    fn wrong_enum_type() {
        let v = Value::Str("unknown".into());
        let err = PlaybackStatus::try_from(v.clone()).unwrap_err();

        assert!(matches!(err, zvariant::Error::Message(_)));

        let ov = OwnedValue::try_from(v).unwrap();
        let err = PlaybackStatus::try_from(ov).unwrap_err();

        assert!(matches!(err, zvariant::Error::Message(_)));
    }

    #[test]
    fn wrong_dbus_type() {
        let v = Value::U64(5);
        let err = PlaybackStatus::try_from(v.clone()).unwrap_err();

        assert_eq!(zvariant::Error::IncorrectType, err);

        let ov = OwnedValue::try_from(v).unwrap();
        let err = PlaybackStatus::try_from(ov).unwrap_err();

        assert_eq!(zvariant::Error::IncorrectType, err);
    }

    #[test]
    fn serialize() {
        let ctxt = Context::new_dbus(LE, 0);
        let encoded = to_bytes(ctxt, &PlaybackStatus::Stopped).unwrap();
        let decoded: &str = encoded.deserialize().unwrap().0;

        assert_eq!(decoded, "Stopped");
    }

    #[test]
    fn deserialize() {
        let ctxt = Context::new_dbus(LE, 0);
        let encoded = to_bytes(ctxt, "Stopped").unwrap();
        let decoded: PlaybackStatus = encoded.deserialize().unwrap().0;

        assert_eq!(decoded, PlaybackStatus::Stopped);
    }
}
