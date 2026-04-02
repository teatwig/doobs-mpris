// SPDX-License-Identifier: MPL-2.0
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt::{self, Display};
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Type, Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Type, Serialize, Deserialize, Value)]
pub struct PlaylistId(OwnedObjectPath);

impl PlaylistId {
    pub fn into_inner(self) -> OwnedObjectPath {
        self.0
    }

    pub fn into_static_path(self) -> ObjectPath<'static> {
        self.0.into_inner().into_owned()
    }
}

impl Deref for PlaylistId {
    type Target = OwnedObjectPath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> AsRef<ObjectPath<'a>> for PlaylistId {
    fn as_ref(&self) -> &ObjectPath<'a> {
        &self.0
    }
}

impl PartialOrd for PlaylistId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlaylistId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.as_str().cmp(other.0.as_str())
    }
}

impl Display for PlaylistId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl From<PlaylistId> for OwnedValue {
    fn from(value: PlaylistId) -> Self {
        value.into_inner().into_inner().into()
    }
}

impl From<OwnedObjectPath> for PlaylistId {
    fn from(value: OwnedObjectPath) -> Self {
        Self(value)
    }
}

impl TryFrom<OwnedValue> for PlaylistId {
    type Error = zvariant::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let oop: OwnedObjectPath = value.try_into()?;
        Ok(Self(oop))
    }
}

impl TryFrom<&str> for PlaylistId {
    type Error = zvariant::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let oop: OwnedObjectPath = value.try_into()?;
        Ok(Self(oop))
    }
}

impl TryFrom<String> for PlaylistId {
    type Error = zvariant::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}
