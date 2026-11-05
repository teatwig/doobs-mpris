use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use zbus::{OwnedVariant, Type, Variant};

use crate::{Error, Result};

/// A repeat / loop status
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopStatus {
    /// The playback will stop when there are no more tracks to play.
    None,
    /// The current track will start again from the begining once it has finished playing.
    Track,
    /// The playback loops through a list of tracks.
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
    const SIGNATURE: &'static zbus::Signature = &zbus::Signature::Str;
}

impl From<LoopStatus> for Variant<'_> {
    fn from(value: LoopStatus) -> Self {
        Variant::from(value.to_string())
    }
}

impl TryFrom<Variant<'_>> for LoopStatus {
    type Error = zbus::Error;

    fn try_from(value: Variant) -> std::result::Result<Self, Self::Error> {
        let value = value.downcast_ref::<&str>()?;
        let value = LoopStatus::from_str(value)?;
        Ok(value)
    }
}

impl TryFrom<OwnedVariant> for LoopStatus {
    type Error = zbus::Error;

    fn try_from(value: OwnedVariant) -> std::result::Result<Self, Self::Error> {
        let value = value.downcast_ref::<&str>()?;
        let value = LoopStatus::from_str(value)?;
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use zbus::wire::serialized::Context;
    use zbus::wire::{LE, to_bytes};
    use zbus::{OwnedVariant, Variant};

    use super::LoopStatus;

    #[test]
    fn success() {
        let expected = LoopStatus::Track;

        // lowercase string works
        let v = Variant::Str("track".into());
        let status = LoopStatus::try_from(v.clone()).unwrap();

        assert_eq!(status, expected);
        assert_eq!(Variant::Str("Track".into()), Variant::from(expected));

        let ov = OwnedVariant::try_from(v.clone()).unwrap();
        let duration = LoopStatus::try_from(ov.clone()).unwrap();

        assert_eq!(duration, expected);
    }

    #[test]
    fn wrong_enum_type() {
        let v = Variant::Str("unknown".into());
        let err = LoopStatus::try_from(v.clone()).unwrap_err();

        assert!(matches!(err, zbus::Error::Failure(_)));

        let ov = OwnedVariant::try_from(v).unwrap();
        let err = LoopStatus::try_from(ov).unwrap_err();

        assert!(matches!(err, zbus::Error::Failure(_)));
    }

    #[test]
    fn wrong_dbus_type() {
        let v = Variant::U64(5);
        let err = LoopStatus::try_from(v.clone()).unwrap_err();

        assert_eq!(zbus::Error::IncorrectType, err);

        let ov = OwnedVariant::try_from(v).unwrap();
        let err = LoopStatus::try_from(ov).unwrap_err();

        assert_eq!(zbus::Error::IncorrectType, err);
    }

    #[test]
    fn serialize() {
        let ctxt = Context::new(LE, 0);
        let encoded = to_bytes(ctxt, &LoopStatus::Playlist).unwrap();
        let decoded: &str = encoded.deserialize().unwrap().0;

        assert_eq!(decoded, "Playlist");
    }

    #[test]
    fn deserialize() {
        let ctxt = Context::new(LE, 0);
        let encoded = to_bytes(ctxt, "Playlist").unwrap();
        let decoded: LoopStatus = encoded.deserialize().unwrap().0;

        assert_eq!(decoded, LoopStatus::Playlist);
    }
}
