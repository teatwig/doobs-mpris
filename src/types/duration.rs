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
        let micros: i64 = value.try_into()?;
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
        let micros: i64 = value.try_into()?;
        Ok(MprisDuration::from_micros(micros))
    }
}

// IMPORTANT: we need to implement ser/de directly, because signals like `seeked` don't use `From`
// to convert to a dbus type and zbus doesn't know that SignedDuration should be serialized as i64.

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

#[cfg(test)]
mod test {
    use jiff::SignedDuration;
    use zvariant::serialized::Context;
    use zvariant::{LE, OwnedValue, Value, to_bytes};

    use super::MprisDuration;

    #[test]
    fn success_from_pos_i64() {
        let expected = SignedDuration::from_secs(42).into();

        let v = Value::I64(42 * 1000 * 1000);
        let duration = MprisDuration::try_from(v.clone()).unwrap();

        assert_eq!(duration, expected);
        assert_eq!(v, Value::from(expected));

        let ov = OwnedValue::try_from(v.clone()).unwrap();
        let duration = MprisDuration::try_from(ov.clone()).unwrap();

        assert_eq!(duration, expected);
        assert_eq!(ov, OwnedValue::from(expected));
    }

    #[test]
    fn success_from_neg_i64() {
        let expected = SignedDuration::from_secs(-15).into();

        let v = Value::I64(-15 * 1000 * 1000);
        let duration = MprisDuration::try_from(v.clone()).unwrap();

        assert_eq!(duration, expected);
        assert_eq!(v, Value::from(expected));

        let ov = OwnedValue::try_from(v).unwrap();
        let duration = MprisDuration::try_from(ov.clone()).unwrap();

        assert_eq!(duration, expected);
        assert_eq!(ov, OwnedValue::from(expected));
    }

    #[test]
    fn wrong_dbus_type() {
        let v = Value::U64(5);
        let err = MprisDuration::try_from(v.clone()).unwrap_err();

        assert_eq!(zvariant::Error::IncorrectType, err);

        let ov = OwnedValue::try_from(v).unwrap();
        let err = MprisDuration::try_from(ov).unwrap_err();

        assert_eq!(zvariant::Error::IncorrectType, err);
    }

    #[test]
    fn serialize() {
        let expected = 60 * 1000 * 1000;

        let ctxt = Context::new_dbus(LE, 0);
        let encoded = to_bytes(ctxt, &MprisDuration::from_micros(expected)).unwrap();
        let decoded: i64 = encoded.deserialize().unwrap().0;

        assert_eq!(decoded, expected);
    }

    #[test]
    fn deserialize() {
        let expected = 60 * 1000 * 1000;

        let ctxt = Context::new_dbus(LE, 0);
        let encoded = to_bytes(ctxt, &expected).unwrap();
        let decoded: MprisDuration = encoded.deserialize().unwrap().0;

        assert_eq!(decoded.as_micros_i64(), expected);
    }
}
