// SPDX-License-Identifier: MPL-2.0
use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use std::ops::{Deref, DerefMut};

use jiff::{SignedDuration, Timestamp};
use serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Type, Value};

use crate::types::TrackId;
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
    pub fn new(track_id: TrackId) -> Self {
        let mut metadata = Self {
            inner: HashMap::new(),
        };
        metadata.insert(FIELD_TRACK_ID.to_string(), track_id.into());
        metadata
    }

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
    pub fn bpm(&self) -> Option<u32> {
        self.inner
            .get(FIELD_AUDIO_BPM)
            .cloned()
            .and_then(|v| try_value_into_integer(v).ok())
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
    pub fn disc_number(&self) -> Option<u32> {
        self.inner
            .get(FIELD_DISC_NUMBER)
            .cloned()
            .and_then(|v| try_value_into_integer(v).ok())
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
    pub fn track_number(&self) -> Option<u32> {
        self.inner
            .get(FIELD_TRACK_NUMBER)
            .cloned()
            .and_then(|v| try_value_into_integer(v).ok())
    }

    /// `xesam:url`: The location of the media file.
    pub fn url(&self) -> Option<String> {
        self.inner
            .get(FIELD_URL)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `xesam:useCount`: The number of times the track has been played.
    pub fn play_count(&self) -> Option<u32> {
        self.inner
            .get(FIELD_USE_COUNT)
            .cloned()
            .and_then(|v| try_value_into_integer(v).ok())
    }

    /// `xesam:userRating`: The user's rating of the track.
    pub fn user_rating(&self) -> Option<f64> {
        self.inner
            .get(FIELD_USER_RATING)
            .cloned()
            .and_then(|v| v.try_into().ok())
    }

    /// `mpris:trackid`: D-Bus path: A unique identity for this track within the context of an MPRIS object (eg: tracklist).
    pub fn track_id(&self) -> Option<TrackId> {
        self.inner
            .get(FIELD_TRACK_ID)
            .cloned()
            .and_then(|path| TrackId::try_from(path).ok())
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

impl From<Metadata> for Value<'_> {
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

/// Tries to convert the given value into an Integer.
pub fn try_value_into_integer(value: OwnedValue) -> Result<u32> {
    // There is no clear consensus what an "Integer" in the Xesam properties is.
    // But most projects seemt to use i32 (`i`).
    // This also aligns with the `track_length` being differentiated as a "64-bit integer".
    // For maximum compatiblity we parse all possible integer types into a u32.
    //
    // A u32 (or rather u31, if we ignore the signed part) is big enough to hold all sensible values.
    //
    // * `bpm`/`track_number`/`disc_number` are comparitively small.
    // * `use_count` could outgrow a u16 if someone listens to a track *a lot*, but not a u32.
    match *value {
        Value::U8(v) => Ok(v as u32),
        Value::Bool(v) => Ok(v as u32),
        Value::I16(v) => Ok(v as u32),
        Value::U16(v) => Ok(v as u32),
        Value::I32(v) => Ok(v as u32),
        Value::U32(v) => Ok(v),
        Value::I64(v) => Ok(v as u32),
        Value::U64(v) => Ok(v as u32),
        _ => Err(Error::IncorrectValue {
            wanted: "Integer",
            actual: value,
        }),
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

fn write_zvalue(f: &mut fmt::Formatter<'_>, value: &Value) -> fmt::Result {
    match value {
        Value::U8(b) => write!(f, "{b}"),
        Value::Bool(b) => write!(f, "{b}"),
        Value::I16(i) => write!(f, "{i}"),
        Value::U16(u) => write!(f, "{u}"),
        Value::I32(i) => write!(f, "{i}"),
        Value::U32(u) => write!(f, "{u}"),
        Value::I64(i) => write!(f, "{i}"),
        Value::U64(u) => write!(f, "{u}"),
        Value::F64(d) => write!(f, "{d}"),
        Value::Str(s) => write!(f, "\"{s}\""),
        Value::ObjectPath(p) => write!(f, "\"{p}\""),
        Value::Array(a) => {
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
        Value::Dict(d) => {
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
        Value::Value(value) => write_zvalue(f, value),
        _ => write!(f, "{value:?}"),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::str::FromStr;

    use jiff::SignedDuration;
    use zvariant::serialized::Context;
    use zvariant::{
        Array, Dict, LE, ObjectPath, OwnedValue, Signature, Value, signature, to_bytes,
        to_bytes_for_signature,
    };

    use super::*;

    #[test]
    fn success() {
        let mut expected: Metadata = HashMap::new().into();
        expected.insert(
            FIELD_TRACK_ID.to_string(),
            ObjectPath::try_from("/hii").unwrap().into(),
        );

        let mut v = Dict::new(&Signature::Str, &Signature::Variant);
        v.add(
            FIELD_TRACK_ID,
            Value::ObjectPath("/hii".try_into().unwrap()),
        )
        .unwrap();
        let v = Value::Dict(v);
        let ov = OwnedValue::try_from(v.clone()).unwrap();
        let metadata = Metadata::try_from(ov).unwrap();

        assert_eq!(metadata, expected);
        assert_eq!(v, Value::from(expected));
    }

    #[test]
    fn wrong_dbus_type() {
        let v = Value::U64(5);
        let ov = OwnedValue::try_from(v).unwrap();
        let err = Metadata::try_from(ov).unwrap_err();

        assert_eq!(zvariant::Error::IncorrectType, err);
    }

    #[test]
    fn new() {
        let op = TrackId::try_from("/hii").unwrap();
        let metadata = Metadata::new(op);
        assert_eq!(
            metadata.track_id(),
            Some(TrackId::try_from("/hii").unwrap())
        );
    }

    #[test]
    fn get_track_id() {
        let metadata = new_metadata(FIELD_TRACK_ID, ObjectPath::try_from("/hii").unwrap());
        assert_eq!(
            metadata.track_id(),
            Some(TrackId::try_from("/hii").unwrap())
        );
    }

    #[test]
    fn get_length() {
        let metadata = new_metadata(FIELD_LENGTH, Value::I64(42 * 1000 * 1000));
        assert_eq!(metadata.length(), Some(SignedDuration::from_secs(42)));
    }

    #[test]
    fn get_art_url() {
        let metadata = new_metadata(FIELD_ART_URL, Value::Str("https://my/cool/art".into()));
        assert_eq!(metadata.art_url(), Some("https://my/cool/art".to_string()));
    }

    #[test]
    fn get_album() {
        let metadata = new_metadata(FIELD_ALBUM, Value::Str("The Album".into()));
        assert_eq!(metadata.album(), Some("The Album".to_string()));
    }

    #[test]
    fn get_album_artist() {
        let mut arr = Array::new(&Signature::Str);
        arr.append(Value::Str("Foo Artist".into())).unwrap();
        arr.append(Value::Str("Bar Artist".into())).unwrap();
        let metadata = new_metadata(FIELD_ALBUM_ARTIST, arr);
        assert_eq!(
            metadata.album_artists(),
            Some(vec!["Foo Artist".to_string(), "Bar Artist".to_string()])
        );
    }

    #[test]
    fn get_artist() {
        let mut arr = Array::new(&Signature::Str);
        arr.append(Value::Str("Foo Artist".into())).unwrap();
        arr.append(Value::Str("Bar Artist".into())).unwrap();
        let metadata = new_metadata(FIELD_ARTIST, arr);
        assert_eq!(
            metadata.artists(),
            Some(vec!["Foo Artist".to_string(), "Bar Artist".to_string()])
        );
    }

    #[test]
    fn get_lyrics() {
        let metadata = new_metadata(
            FIELD_AS_TEXT,
            Value::Str("If I can live through this, I can do anything".into()),
        );
        assert_eq!(
            metadata.lyrics(),
            Some("If I can live through this, I can do anything".to_string())
        );
    }

    #[test]
    fn get_bpm() {
        let metadata = new_metadata(FIELD_AUDIO_BPM, Value::I32(120));
        assert_eq!(metadata.bpm(), Some(120));
    }

    #[test]
    fn get_auto_rating() {
        let metadata = new_metadata(FIELD_AUTO_RATING, Value::F64(0.7));
        assert_eq!(metadata.auto_rating(), Some(0.7));
    }

    #[test]
    fn get_comments() {
        let mut arr = Array::new(&Signature::Str);
        arr.append(Value::Str("Some comment".into())).unwrap();
        arr.append(Value::Str("Another comment".into())).unwrap();
        let metadata = new_metadata(FIELD_COMMENT, arr);
        assert_eq!(
            metadata.comments(),
            Some(vec![
                "Some comment".to_string(),
                "Another comment".to_string()
            ])
        );
    }

    #[test]
    fn get_composers() {
        let mut arr = Array::new(&Signature::Str);
        arr.append(Value::Str("Chopin".into())).unwrap();
        arr.append(Value::Str("Franchomme".into())).unwrap();
        let metadata = new_metadata(FIELD_COMPOSER, arr);
        assert_eq!(
            metadata.composers(),
            Some(vec!["Chopin".to_string(), "Franchomme".to_string()])
        );
    }

    #[test]
    fn get_created() {
        let metadata = new_metadata(
            FIELD_CONTENT_CREATED,
            Value::Str("2007-04-29T13:56+01:00".into()),
        );
        assert_eq!(
            metadata.created(),
            Some(Timestamp::from_str("2007-04-29T13:56+01:00").unwrap())
        );
    }

    #[test]
    fn get_disc_number() {
        let metadata = new_metadata(FIELD_DISC_NUMBER, Value::I32(2));
        assert_eq!(metadata.disc_number(), Some(2));
    }

    #[test]
    fn get_first_played() {
        let metadata = new_metadata(
            FIELD_FIRST_USED,
            Value::Str("2007-04-29T13:56+01:00".into()),
        );
        assert_eq!(
            metadata.first_played(),
            Some(Timestamp::from_str("2007-04-29T13:56+01:00").unwrap())
        );
    }

    #[test]
    fn get_genres() {
        let mut arr = Array::new(&Signature::Str);
        arr.append(Value::Str("Pop".into())).unwrap();
        arr.append(Value::Str("Rock".into())).unwrap();
        let metadata = new_metadata(FIELD_GENRE, arr);
        assert_eq!(
            metadata.genres(),
            Some(vec!["Pop".to_string(), "Rock".to_string()])
        );
    }

    #[test]
    fn get_last_played() {
        let metadata = new_metadata(FIELD_LAST_USED, Value::Str("2007-04-29T13:56+01:00".into()));
        assert_eq!(
            metadata.last_played(),
            Some(Timestamp::from_str("2007-04-29T13:56+01:00").unwrap())
        );
    }

    #[test]
    fn get_lyricists() {
        let mut arr = Array::new(&Signature::Str);
        arr.append(Value::Str("Frog in a Car".into())).unwrap();
        arr.append(Value::Str("Potato Chip".into())).unwrap();
        let metadata = new_metadata(FIELD_LYRICIST, arr);
        assert_eq!(
            metadata.lyricists(),
            Some(vec!["Frog in a Car".to_string(), "Potato Chip".to_string()])
        );
    }

    #[test]
    fn get_title() {
        let metadata = new_metadata(FIELD_TITLE, Value::Str("The Grid".into()));
        assert_eq!(metadata.title(), Some("The Grid".to_string()));
    }

    #[test]
    fn get_track_number() {
        let metadata = new_metadata(FIELD_TRACK_NUMBER, Value::I32(7));
        assert_eq!(metadata.track_number(), Some(7));
    }

    #[test]
    fn get_url() {
        let metadata = new_metadata(FIELD_URL, Value::Str("https://the/best/song/ever".into()));
        assert_eq!(
            metadata.url(),
            Some("https://the/best/song/ever".to_string())
        );
    }

    #[test]
    fn get_play_count() {
        let metadata = new_metadata(FIELD_USE_COUNT, Value::I32(123));
        assert_eq!(metadata.play_count(), Some(123));
    }

    #[test]
    fn get_user_rating() {
        let metadata = new_metadata(FIELD_USER_RATING, Value::F64(0.9));
        assert_eq!(metadata.user_rating(), Some(0.9));
    }

    #[test]
    fn serialize() {
        let ctxt = Context::new_dbus(LE, 0);
        let encoded = to_bytes(ctxt, &new_metadata(FIELD_TRACK_NUMBER, 7)).unwrap();
        let decoded: HashMap<String, OwnedValue> = encoded.deserialize().unwrap().0;

        assert_eq!(
            decoded.get(FIELD_TRACK_NUMBER).unwrap().deref(),
            &Value::from(7)
        );
    }

    #[test]
    fn deserialize() {
        let mut hashmap: HashMap<String, OwnedValue> = HashMap::new();
        hashmap.insert(FIELD_TRACK_NUMBER.to_string(), OwnedValue::from(7i32));

        let ctxt = Context::new_dbus(LE, 0);
        let encoded = to_bytes_for_signature(ctxt, signature!("a{sv}"), &hashmap).unwrap();
        let decoded: Metadata = encoded.deserialize().unwrap().0;

        assert_eq!(
            decoded.get(FIELD_TRACK_NUMBER).unwrap().deref(),
            &Value::from(7)
        );
    }

    fn new_metadata<'a, K, V>(key: K, value: V) -> Metadata
    where
        K: Into<String>,
        V: Into<Value<'a>>,
    {
        let mut metadata: Metadata = HashMap::new().into();
        metadata.insert(key.into(), value.into().try_into().unwrap());
        metadata
    }
}
