use chrono::{DateTime, NaiveDateTime, Utc};
use dop2marshal::AssocTypes;
use crate::payloader::prelude::*;
use crate::{MakeAnnotatedValueType, MakeGenericValueType, Dop2ParseTreeExpressible};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dop2TimestampUtc(pub DateTime<Utc>);

impl TryFrom<u64> for Dop2TimestampUtc {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > i64::MAX as u64 {
            return Err("timestamp out of range");
        }

        let naive = NaiveDateTime::from_timestamp(value as i64, 0);
        Ok(Dop2TimestampUtc(DateTime::<Utc>::from_utc(naive, Utc)))
    }
}

impl From<Dop2TimestampUtc> for u64 {
    fn from(value: Dop2TimestampUtc) -> Self {
        return value.0.timestamp() as u64;
    }
}

MakeAnnotatedValueType!(AnnotatedU8, U8, u8);
MakeAnnotatedValueType!(AnnotatedU16, U16, u16);
MakeAnnotatedValueType!(AnnotatedU64, U64, u64);
MakeAnnotatedValueType!(AnnotatedBool, Boolean, bool);
MakeAnnotatedValueType!(AnnotatedTimeStamp, U64, Dop2TimestampUtc);

MakeGenericValueType!(GenericU8, U8, u8);
MakeGenericValueType!(GenericU16, U16, u16);

