use chrono::{DateTime, Utc};
use dop2marshal::AssocTypes;
use derive_more::From;
use crate::payloader::prelude::*;
use crate::{MakeAnnotatedValueType, MakeGenericValueType, Dop2ParseTreeExpressible, ToDop2Bytes, Dop2PayloadExpressible, Dop2Parser, impl_to_bytes, impl_from_bytes};
use crate::newtype_int;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dop2TimestampUtc(pub DateTime<Utc>);

impl TryFrom<u64> for Dop2TimestampUtc {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > i64::MAX as u64 {
            return Err("timestamp out of range");
        }

        let dt = DateTime::from_timestamp(value as i64, 0)
            .ok_or("timestamp out of range")?;
        Ok(Dop2TimestampUtc(dt))
    }
}

impl From<Dop2TimestampUtc> for u64 {
    fn from(value: Dop2TimestampUtc) -> Self {
        return value.0.timestamp() as u64;
    }
}

MakeAnnotatedValueType!(AnnotatedU8, U8, u8);
MakeAnnotatedValueType!(AnnotatedU16, U16, u16);
MakeAnnotatedValueType!(AnnotatedI16, I16, i16);
MakeAnnotatedValueType!(AnnotatedI32, I32, i32);
MakeAnnotatedValueType!(AnnotatedU64, U64, u64);
MakeAnnotatedValueType!(AnnotatedBool, Boolean, bool);
MakeAnnotatedValueType!(AnnotatedTimeStamp, U64, Dop2TimestampUtc);

MakeGenericValueType!(GenericU8, U8, u8);
MakeGenericValueType!(GenericU16, U16, u16);

impl_tryfrom_dop2struct!(AnnotatedU8);
impl_tryfrom_dop2struct!(AnnotatedU16);
impl_tryfrom_dop2struct!(AnnotatedI16);
impl_tryfrom_dop2struct!(AnnotatedI32);
impl_tryfrom_dop2struct!(AnnotatedU64);
impl_tryfrom_dop2struct!(AnnotatedBool);
impl_tryfrom_dop2struct!(AnnotatedTimeStamp);

impl_tryfrom_dop2struct!(GenericU8);
impl_tryfrom_dop2struct!(GenericU16);

// Implement traits for primitive integer types
impl_to_bytes!(u8);
impl_to_bytes!(i8);
impl_to_bytes!(u16);
impl_to_bytes!(i16);
impl_to_bytes!(u32);
impl_to_bytes!(i32);
impl_to_bytes!(u64);
impl_to_bytes!(i64);

impl_from_bytes!(u8);
impl_from_bytes!(i8);
impl_from_bytes!(u16);
impl_from_bytes!(i16);
impl_from_bytes!(u32);
impl_from_bytes!(i32);
impl_from_bytes!(u64);
impl_from_bytes!(i64);

newtype_int!(E8, u8);
newtype_int!(E16, u16);
newtype_int!(E32, u32);
newtype_int!(E64, u64);

