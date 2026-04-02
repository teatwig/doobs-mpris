// SPDX-License-Identifier: MPL-2.0
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Type, Value};

/// This path indicates "no track" when returned as the current Track ID,
/// or the start of a playlist when inserting into one.
pub const TRACK_ID_NO_TRACK: ObjectPath =
    ObjectPath::from_static_str_unchecked("/org/mpris/MediaPlayer2/TrackList/NoTrack");

/// Unique track identifier.
///
/// If the media player implements the TrackList interface and allows the same track to appear multiple times in the tracklist, this must be unique within the scope of the tracklist.
///
/// Note that this should be a valid D-Bus object id, although clients should not assume that any object is actually exported with any interfaces at that path.
///
/// Media players may not use any paths starting with /org/mpris unless explicitly allowed by this specification. Such paths are intended to have special meaning, such as /org/mpris/MediaPlayer2/TrackList/NoTrack to indicate "no track".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Type, Serialize, Deserialize, Value)]
pub struct TrackId(OwnedObjectPath);

impl TrackId {
    pub fn into_inner(self) -> OwnedObjectPath {
        self.0
    }

    pub fn into_static_path(self) -> ObjectPath<'static> {
        self.0.into_inner().into_owned()
    }
}

impl Deref for TrackId {
    type Target = OwnedObjectPath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> AsRef<ObjectPath<'a>> for TrackId {
    fn as_ref(&self) -> &ObjectPath<'a> {
        &self.0
    }
}

impl PartialEq<ObjectPath<'_>> for TrackId {
    fn eq(&self, other: &ObjectPath) -> bool {
        self.as_ref() == other
    }
}

impl PartialOrd for TrackId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TrackId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.as_str().cmp(other.0.as_str())
    }
}

impl Display for TrackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl From<TrackId> for OwnedValue {
    fn from(value: TrackId) -> Self {
        value.into_static_path().into()
    }
}

impl From<ObjectPath<'_>> for TrackId {
    fn from(value: ObjectPath) -> Self {
        Self(value.into())
    }
}

impl From<OwnedObjectPath> for TrackId {
    fn from(value: OwnedObjectPath) -> Self {
        Self(value)
    }
}

impl TryFrom<OwnedValue> for TrackId {
    type Error = zvariant::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let oop: OwnedObjectPath = value.try_into()?;
        Ok(Self(oop))
    }
}

impl TryFrom<&str> for TrackId {
    type Error = zvariant::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let oop: OwnedObjectPath = value.try_into()?;
        Ok(Self(oop))
    }
}

impl TryFrom<String> for TrackId {
    type Error = zvariant::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

#[cfg(test)]
mod test {
    use super::{TRACK_ID_NO_TRACK, TrackId};

    #[test]
    fn eq() {
        assert_eq!(TrackId::from(TRACK_ID_NO_TRACK), TRACK_ID_NO_TRACK);
    }
}
