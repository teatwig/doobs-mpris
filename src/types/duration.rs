// SPDX-License-Identifier: MPL-2.0
use std::fmt::Debug;
use std::ops::Deref;

use jiff::SignedDuration;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zvariant::{OwnedValue, Type, Value};

/// Simple wrapper around [jiff::SignedDuration] to allow using it as a zbus type.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MprisDuration(SignedDuration);

impl MprisDuration {
    pub fn from_micros(micros: i64) -> Self {
        Self(SignedDuration::from_micros(micros))
    }

    fn as_micros_i64(&self) -> i64 {
        // it should be fine to truncate here
        // mpris uses i64 (x) anyway, and a track is unlikely to be longer than ~70 hours
        self.as_micros() as i64
    }
}

impl Deref for MprisDuration {
    type Target = SignedDuration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for MprisDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl From<SignedDuration> for MprisDuration {
    fn from(value: SignedDuration) -> Self {
        Self(value)
    }
}

impl From<MprisDuration> for SignedDuration {
    fn from(value: MprisDuration) -> Self {
        value.0
    }
}

// traits necessary to use it as a type in zbus

impl Type for MprisDuration {
    const SIGNATURE: &'static zvariant::Signature = &zvariant::Signature::I64;
}

impl From<MprisDuration> for Value<'_> {
    fn from(value: MprisDuration) -> Self {
        Value::from(value.as_micros_i64())
    }
}

impl TryFrom<Value<'_>> for MprisDuration {
    type Error = zvariant::Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let micros = value.downcast_ref::<i64>()?;
        Ok(MprisDuration::from_micros(micros))
    }
}

impl From<MprisDuration> for OwnedValue {
    fn from(value: MprisDuration) -> Self {
        OwnedValue::from(value.as_micros_i64())
    }
}

impl TryFrom<OwnedValue> for MprisDuration {
    type Error = zvariant::Error;

    fn try_from(value: OwnedValue) -> std::result::Result<Self, Self::Error> {
        let micros = value.downcast_ref::<i64>()?;
        Ok(MprisDuration::from_micros(micros))
    }
}

impl Serialize for MprisDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.as_micros_i64())
    }
}

impl<'de> Deserialize<'de> for MprisDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let micros = i64::deserialize(deserializer)?;
        Ok(MprisDuration::from_micros(micros))
    }
}
