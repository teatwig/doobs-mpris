// SPDX-License-Identifier: MPL-2.0
use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use std::ops::{Deref, DerefMut};

use jiff::{SignedDuration, Timestamp};
use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedObjectPath, Value as ZValue};
use zvariant::{OwnedValue, Type};

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    inner: HashMap<String, OwnedValue>,
}

impl Metadata {
    /// `xesam:album`: The track artist(s).
    pub fn album(&self) -> Option<String> {
        self.inner
            .get("xesam:album")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:artist`: The track artist(s).
    pub fn artists(&self) -> Option<Vec<String>> {
        self.inner
            .get("xesam:artist")
            .cloned()
            .and_then(|artists| artists.try_into().ok())
    }

    /// `xesam:asText`: The track lyrics.
    pub fn lyrics(&self) -> Option<String> {
        self.inner
            .get("xesam:asText")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:albumArtist`: The album artist(s).
    pub fn album_artists(&self) -> Option<Vec<String>> {
        self.inner
            .get("xesam:albumArtist")
            .cloned()
            .and_then(|artists| artists.try_into().ok())
    }

    /// `xesam:audioBPM`: The speed of the music, in beats per minute.
    pub fn bpm(&self) -> Option<u64> {
        self.inner
            .get("xesam:audioBPM")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:autoRating`: An automatically-generated rating, based on things such as how often it has been played.
    /// This should be in the range 0.0 to 1.0.
    pub fn auto_rating(&self) -> Option<f64> {
        self.inner
            .get("xesam:autoRating")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:composer`: The composer(s) of the track.
    pub fn composer(&self) -> Option<Vec<String>> {
        self.inner
            .get("xesam:composer")
            .cloned()
            .and_then(|composers| composers.try_into().ok())
    }

    /// `xesam:contentCreated`: When the track was created. Usually only the year component will be useful.
    pub fn created(&self) -> Option<Timestamp> {
        self.inner
            .get("xesam:contentCreated")
            .cloned()
            .and_then(|v| try_value_into_date(v).ok())
    }

    /// `xesam:discNumber`: The disc number on the album that this track is from.
    pub fn disc_number(&self) -> Option<u64> {
        self.inner
            .get("xesam:discNumber")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:firstUsed`: When the track was first played.
    pub fn first_played(&self) -> Option<Timestamp> {
        self.inner
            .get("xesam:firstUsed")
            .cloned()
            .and_then(|v| try_value_into_date(v).ok())
    }

    /// `xesam:genre`: The genre(s) of the track.
    pub fn genre(&self) -> Option<Vec<String>> {
        self.inner
            .get("xesam:genre")
            .cloned()
            .and_then(|genres| genres.try_into().ok())
    }

    /// `xesam:lastUsed`: When the track was last played.
    pub fn last_played(&self) -> Option<Timestamp> {
        self.inner
            .get("xesam:lastUsed")
            .cloned()
            .and_then(|v| try_value_into_date(v).ok())
    }

    /// `xesam:lyricist`: The lyricist(s) of the track.
    pub fn lyricist(&self) -> Option<Vec<String>> {
        self.inner
            .get("xesam:lyricist")
            .cloned()
            .and_then(|lyricists| lyricists.try_into().ok())
    }

    /// `xesam:title`: The track title.
    pub fn title(&self) -> Option<String> {
        self.inner
            .get("xesam:title")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:trackNumber`: The track number on the album that this track is from.
    pub fn track_number(&self) -> Option<u64> {
        self.inner
            .get("xesam:trackNumber")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:url`: The location of the media file.
    pub fn url(&self) -> Option<String> {
        self.inner
            .get("xesam:url")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:useCount`: The number of times the track has been played.
    pub fn use_count(&self) -> Option<u64> {
        self.inner
            .get("xesam:useCount")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:userRating`: The user's rating of the track.
    pub fn user_rating(&self) -> Option<f64> {
        self.inner
            .get("xesam:userRating")
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `mpris:trackid`: D-Bus path: A unique identity for this track within the context of an MPRIS object (eg: tracklist).
    pub fn track_id(&self) -> Option<OwnedObjectPath> {
        self.inner
            .get("mpris:trackid")
            .cloned()
            .and_then(|path| OwnedObjectPath::try_from(path).ok())
    }

    /// `mpris:length`: The length of the track in microseconds.
    pub fn length(&self) -> Option<SignedDuration> {
        self.inner
            .get("mpris:length")
            .cloned()
            .and_then(|v| i64::try_from(v).ok())
            .map(SignedDuration::from_micros)
    }

    /// `mpris:artUrl`: The location of an image representing the track or album.
    /// Clients should not assume this will continue to exist when the media player stops giving out the URL.
    pub fn art_url(&self) -> Option<String> {
        self.inner
            .get("mpris:artUrl")
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
