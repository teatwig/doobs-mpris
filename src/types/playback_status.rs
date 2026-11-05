use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use zbus::{OwnedVariant, Type, Variant};

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
    const SIGNATURE: &'static zbus::Signature = &zbus::Signature::Str;
}

impl From<PlaybackStatus> for Variant<'_> {
    fn from(value: PlaybackStatus) -> Self {
        Variant::from(value.to_string())
    }
}

impl TryFrom<Variant<'_>> for PlaybackStatus {
    type Error = zbus::Error;

    fn try_from(value: Variant) -> std::result::Result<Self, Self::Error> {
        let value: &str = value.downcast_ref::<&str>()?;
        let value = PlaybackStatus::from_str(value)?;
        Ok(value)
    }
}

impl TryFrom<OwnedVariant> for PlaybackStatus {
    type Error = zbus::Error;

    fn try_from(value: OwnedVariant) -> std::result::Result<Self, Self::Error> {
        let value = value.downcast_ref::<&str>()?;
        let value = PlaybackStatus::from_str(value)?;
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use zbus::wire::serialized::Context;
    use zbus::wire::{LE, to_bytes};
    use zbus::{OwnedVariant, Variant};

    use super::PlaybackStatus;

    #[test]
    fn success() {
        let expected = PlaybackStatus::Paused;

        // lowercase string works
        let v = Variant::Str("paused".into());
        let status = PlaybackStatus::try_from(v.clone()).unwrap();

        assert_eq!(status, expected);
        assert_eq!(Variant::Str("Paused".into()), Variant::from(expected));

        let ov = OwnedVariant::try_from(v.clone()).unwrap();
        let duration = PlaybackStatus::try_from(ov.clone()).unwrap();

        assert_eq!(duration, expected);
    }

    #[test]
    fn wrong_enum_type() {
        let v = Variant::Str("unknown".into());
        let err = PlaybackStatus::try_from(v.clone()).unwrap_err();

        assert!(matches!(err, zbus::Error::Failure(_)));

        let ov = OwnedVariant::try_from(v).unwrap();
        let err = PlaybackStatus::try_from(ov).unwrap_err();

        assert!(matches!(err, zbus::Error::Failure(_)));
    }

    #[test]
    fn wrong_dbus_type() {
        let v = Variant::U64(5);
        let err = PlaybackStatus::try_from(v.clone()).unwrap_err();

        assert_eq!(zbus::Error::IncorrectType, err);

        let ov = OwnedVariant::try_from(v).unwrap();
        let err = PlaybackStatus::try_from(ov).unwrap_err();

        assert_eq!(zbus::Error::IncorrectType, err);
    }

    #[test]
    fn serialize() {
        let ctxt = Context::new(LE, 0);
        let encoded = to_bytes(ctxt, &PlaybackStatus::Stopped).unwrap();
        let decoded: &str = encoded.deserialize().unwrap().0;

        assert_eq!(decoded, "Stopped");
    }

    #[test]
    fn deserialize() {
        let ctxt = Context::new(LE, 0);
        let encoded = to_bytes(ctxt, "Stopped").unwrap();
        let decoded: PlaybackStatus = encoded.deserialize().unwrap().0;

        assert_eq!(decoded, PlaybackStatus::Stopped);
    }
}
