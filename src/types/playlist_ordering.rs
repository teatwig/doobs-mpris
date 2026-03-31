// SPDX-License-Identifier: MPL-2.0
use std::fmt::{self, Display};
use std::str::FromStr;

use serde::de::{self, Deserialize, Visitor};
use serde::ser::{Serialize, Serializer};
use zvariant::{Signature, Type, Value};

use crate::{Error, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlaylistOrdering {
    /// Alphabetical ordering by name, ascending.
    Alphabetical,
    /// Ordering by creation date, oldest first.
    CreationDate,
    /// Ordering by last modified date, oldest first.
    ModifiedDate,
    /// Ordering by date of last playback, oldest first.
    LastPlayDate,
    /// A user-defined ordering.
    UserDefined,
}

impl Type for PlaylistOrdering {
    const SIGNATURE: &'static Signature = String::SIGNATURE;
}

impl TryFrom<Value<'_>> for PlaylistOrdering {
    type Error = zvariant::Error;

    fn try_from(value: Value) -> zvariant::Result<Self> {
        let value: &str = value.downcast_ref::<&str>()?;
        let value = PlaylistOrdering::from_str(value)?;
        Ok(value)
    }
}

impl From<PlaylistOrdering> for Value<'_> {
    fn from(ordering: PlaylistOrdering) -> Self {
        Value::Str(ordering.to_string().into())
    }
}

impl FromStr for PlaylistOrdering {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().trim() {
            "alphabetical" => Ok(Self::Alphabetical),
            "created" => Ok(Self::CreationDate),
            "modified" => Ok(Self::ModifiedDate),
            "played" => Ok(Self::LastPlayDate),
            "user" => Ok(Self::UserDefined),
            _ => Err(Error::InvalidEnum {
                got: s.to_string(),
                expected: &["Alphabetical", "Created", "Modified", "Played", "User"],
            }),
        }
    }
}

impl Display for PlaylistOrdering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alphabetical => "Alphabetical",
                Self::CreationDate => "Created",
                Self::ModifiedDate => "Modified",
                Self::LastPlayDate => "Played",
                Self::UserDefined => "User",
            }
        )
    }
}

impl Serialize for PlaylistOrdering {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for PlaylistOrdering {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(PlaylistOrderingVisitor)
    }
}

struct PlaylistOrderingVisitor;

impl Visitor<'_> for PlaylistOrderingVisitor {
    type Value = PlaylistOrdering;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        PlaylistOrdering::from_str(s).map_err(de::Error::custom)
    }
}
