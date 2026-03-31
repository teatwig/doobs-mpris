// SPDX-License-Identifier: MPL-2.0
use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use std::ops::{Deref, DerefMut};

use jiff::{SignedDuration, Timestamp};
use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedObjectPath, Value as ZValue};
use zvariant::{OwnedValue, Type};

use crate::{Error, Result};

// list of metadata properties, see: https://www.freedesktop.org/wiki/Specifications/mpris-spec/metadata/

// MPRIS-specific properties

/// D-Bus path: A unique identity for this track within the context of an MPRIS object (eg: tracklist).
const FIELD_TRACK_ID: &str = "mpris:trackid";
/// 64-bit integer: The duration of the track in microseconds.
const FIELD_LENGTH: &str = "mpris:length";
/// URI: The location of an image representing the track or album.
/// Clients should not assume this will continue to exist when the media player stops giving out the URL.
const FIELD_ART_URL: &str = "mpris:artUrl";

// Common Xesam properties

/// String: The album name.
const FIELD_ALBUM: &str = "xesam:album";
/// List of Strings: The album artist(s).
const FIELD_ALBUM_ARTIST: &str = "xesam:albumArtist";
/// List of Strings: The track artist(s).
const FIELD_ARTIST: &str = "xesam:artist";
/// String: The track lyrics.
const FIELD_AS_TEXT: &str = "xesam:asText";
/// Integer: The speed of the music, in beats per minute.
const FIELD_AUDIO_BPM: &str = "xesam:audioBPM";
/// Float: An automatically-generated rating, based on things such as how often it has been played.
/// This should be in the range 0.0 to 1.0.
const FIELD_AUTO_RATING: &str = "xesam:autoRating";
/// List of Strings: A (list of) freeform comment(s).
const FIELD_COMMENT: &str = "xesam:comment";
/// List of Strings: The composer(s) of the track.
const FIELD_COMPOSER: &str = "xesam:composer";
/// Date/Time: When the track was created. Usually only the year component will be useful.
const FIELD_CONTENT_CREATED: &str = "xesam:contentCreated";
/// Integer: The disc number on the album that this track is from.
const FIELD_DISC_NUMBER: &str = "xesam:discNumber";
/// Date/Time: When the track was first played.
const FIELD_FIRST_USED: &str = "xesam:firstUsed";
/// List of Strings: The genre(s) of the track.
const FIELD_GENRE: &str = "xesam:genre";
/// Date/Time: When the track was last played.
const FIELD_LAST_USED: &str = "xesam:lastUsed";
/// List of Strings: The lyricist(s) of the track.
const FIELD_LYRICIST: &str = "xesam:lyricist";
/// String: The track title.
const FIELD_TITLE: &str = "xesam:title";
/// Integer: The track number on the album disc.
const FIELD_TRACK_NUMBER: &str = "xesam:trackNumber";
/// URI: The location of the media file.
const FIELD_URL: &str = "xesam:url";
/// Integer: The number of times the track has been played.
const FIELD_USE_COUNT: &str = "xesam:useCount";
/// Float: A user-specified rating. This should be in the range 0.0 to 1.0.
const FIELD_USER_RATING: &str = "xesam:userRating";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Metadata {
    inner: HashMap<String, OwnedValue>,
}

impl Metadata {
    /// `xesam:album`: The track artist(s).
    pub fn album(&self) -> Option<String> {
        self.inner
            .get(FIELD_ALBUM)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:artist`: The track artist(s).
    pub fn artists(&self) -> Option<Vec<String>> {
        self.inner
            .get(FIELD_ARTIST)
            .cloned()
            .and_then(|artists| artists.try_into().ok())
    }

    /// `xesam:asText`: The track lyrics.
    pub fn lyrics(&self) -> Option<String> {
        self.inner
            .get(FIELD_AS_TEXT)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:albumArtist`: The album artist(s).
    pub fn album_artists(&self) -> Option<Vec<String>> {
        self.inner
            .get(FIELD_ALBUM_ARTIST)
            .cloned()
            .and_then(|artists| artists.try_into().ok())
    }

    /// `xesam:audioBPM`: The speed of the music, in beats per minute.
    pub fn bpm(&self) -> Option<u64> {
        self.inner
            .get(FIELD_AUDIO_BPM)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:autoRating`: An automatically-generated rating, based on things such as how often it has been played.
    /// This should be in the range 0.0 to 1.0.
    pub fn auto_rating(&self) -> Option<f64> {
        self.inner
            .get(FIELD_AUTO_RATING)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:comment`: A (list of) freeform comment(s).
    pub fn comments(&self) -> Option<Vec<String>> {
        self.inner
            .get(FIELD_COMMENT)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:composer`: The composer(s) of the track.
    pub fn composers(&self) -> Option<Vec<String>> {
        self.inner
            .get(FIELD_COMPOSER)
            .cloned()
            .and_then(|composers| composers.try_into().ok())
    }

    /// `xesam:contentCreated`: When the track was created. Usually only the year component will be useful.
    pub fn created(&self) -> Option<Timestamp> {
        self.inner
            .get(FIELD_CONTENT_CREATED)
            .cloned()
            .and_then(|v| try_value_into_date(v).ok())
    }

    /// `xesam:discNumber`: The disc number on the album that this track is from.
    pub fn disc_number(&self) -> Option<u64> {
        self.inner
            .get(FIELD_DISC_NUMBER)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:firstUsed`: When the track was first played.
    pub fn first_played(&self) -> Option<Timestamp> {
        self.inner
            .get(FIELD_FIRST_USED)
            .cloned()
            .and_then(|v| try_value_into_date(v).ok())
    }

    /// `xesam:genre`: The genre(s) of the track.
    pub fn genres(&self) -> Option<Vec<String>> {
        self.inner
            .get(FIELD_GENRE)
            .cloned()
            .and_then(|genres| genres.try_into().ok())
    }

    /// `xesam:lastUsed`: When the track was last played.
    pub fn last_played(&self) -> Option<Timestamp> {
        self.inner
            .get(FIELD_LAST_USED)
            .cloned()
            .and_then(|v| try_value_into_date(v).ok())
    }

    /// `xesam:lyricist`: The lyricist(s) of the track.
    pub fn lyricists(&self) -> Option<Vec<String>> {
        self.inner
            .get(FIELD_LYRICIST)
            .cloned()
            .and_then(|lyricists| lyricists.try_into().ok())
    }

    /// `xesam:title`: The track title.
    pub fn title(&self) -> Option<String> {
        self.inner
            .get(FIELD_TITLE)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:trackNumber`: The track number on the album that this track is from.
    pub fn track_number(&self) -> Option<u64> {
        self.inner
            .get(FIELD_TRACK_NUMBER)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:url`: The location of the media file.
    pub fn url(&self) -> Option<String> {
        self.inner
            .get(FIELD_URL)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:useCount`: The number of times the track has been played.
    pub fn play_count(&self) -> Option<u64> {
        self.inner
            .get(FIELD_USE_COUNT)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:userRating`: The user's rating of the track.
    pub fn user_rating(&self) -> Option<f64> {
        self.inner
            .get(FIELD_USER_RATING)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `mpris:trackid`: D-Bus path: A unique identity for this track within the context of an MPRIS object (eg: tracklist).
    pub fn track_id(&self) -> Option<OwnedObjectPath> {
        self.inner
            .get(FIELD_TRACK_ID)
            .cloned()
            .and_then(|path| OwnedObjectPath::try_from(path).ok())
    }

    /// `mpris:length`: The length of the track in microseconds.
    pub fn length(&self) -> Option<SignedDuration> {
        self.inner
            .get(FIELD_LENGTH)
            .cloned()
            .and_then(|v| i64::try_from(v).ok())
            .map(SignedDuration::from_micros)
    }

    /// `mpris:artUrl`: The location of an image representing the track or album.
    /// Clients should not assume this will continue to exist when the media player stops giving out the URL.
    pub fn art_url(&self) -> Option<String> {
        self.inner
            .get(FIELD_ART_URL)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }
}

impl Deref for Metadata {
    type Target = HashMap<String, OwnedValue>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut iter = self.inner.iter().peekable();
        while let Some((k, v)) = iter.next() {
            write!(f, "{k}: ")?;
            write_zvalue(f, v)?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl From<HashMap<String, OwnedValue>> for Metadata {
    fn from(value: HashMap<String, OwnedValue>) -> Self {
        Self { inner: value }
    }
}

// traits necessary to use it as a type in zbus

impl Type for Metadata {
    const SIGNATURE: &'static zvariant::Signature = &zvariant::signature!("a{sv}");
}

impl From<Metadata> for ZValue<'_> {
    fn from(value: Metadata) -> Self {
        value.inner.into()
    }
}

impl TryFrom<OwnedValue> for Metadata {
    type Error = zvariant::Error;

    fn try_from(value: OwnedValue) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            inner: value.clone().try_into()?,
        })
    }
}

pub fn try_value_into_date(value: OwnedValue) -> Result<Timestamp> {
    let value: String = value
        .clone()
        .try_into()
        .map_err(|_| Error::IncorrectValue {
            wanted: "Str",
            actual: value,
        })?;
    let value = value
        .parse::<Timestamp>()
        .map_err(Error::InvalidTimestamp)?;
    Ok(value)
}

fn write_zvalue(f: &mut fmt::Formatter<'_>, value: &ZValue) -> fmt::Result {
    match value {
        ZValue::U8(b) => write!(f, "{b}"),
        ZValue::Bool(b) => write!(f, "{b}"),
        ZValue::I16(i) => write!(f, "{i}"),
        ZValue::U16(u) => write!(f, "{u}"),
        ZValue::I32(i) => write!(f, "{i}"),
        ZValue::U32(u) => write!(f, "{u}"),
        ZValue::I64(i) => write!(f, "{i}"),
        ZValue::U64(u) => write!(f, "{u}"),
        ZValue::F64(d) => write!(f, "{d}"),
        ZValue::Str(s) => write!(f, "\"{s}\""),
        ZValue::ObjectPath(p) => write!(f, "\"{p}\""),
        ZValue::Array(a) => {
            write!(f, "[")?;
            let mut iter = a.iter().peekable();
            while let Some(value) = iter.next() {
                if iter.peek().is_some() {
                    write!(f, "{value}, ")?;
                } else {
                    write!(f, "{value}")?;
                }
            }
            write!(f, "]")
        }
        ZValue::Dict(d) => {
            write!(f, "{{")?;
            let mut iter = d.iter().peekable();
            while let Some((k, v)) = iter.next() {
                if iter.peek().is_some() {
                    write!(f, "{k}: {v}, ")?;
                } else {
                    write!(f, "{k}: {v}")?;
                }
            }
            write!(f, "}}")
        }
        ZValue::Value(value) => write_zvalue(f, value),
        ZValue::Signature(_) => write!(f, "(unsupported signature)"),
        ZValue::Structure(_) => write!(f, "(unsupported structure)"),
        ZValue::Fd(_) => write!(f, "(unsuppored fd)"),
    }
}
