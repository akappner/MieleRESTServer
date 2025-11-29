use chrono::{DateTime, Utc};
use core::fmt;
use dop2marshal::AssocTypes;
use derive_more::From;
use crate::payloader::prelude::*;
use crate::{MakeAnnotatedValueType, MakeGenericValueType, impl_to_bytes, impl_from_bytes};
use crate::payloader::root::Dop2ParseTreeExpressible;
use crate::payloader::parser::{DopArray, ToDop2Bytes, Dop2PayloadExpressible, Dop2Parser};
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

/// Represents a MAC address encoded as 8 bytes in a `DopArray<u8>`.
///
/// The expected layout is a `DopArray<u8>` with `count == 8` and exactly 8 elements.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dop2MacAddress(pub [u8; 8]);

impl fmt::Debug for Dop2MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7]
        )
    }
}

impl TryFrom<DopArray<u8>> for Dop2MacAddress {
    type Error = &'static str;

    fn try_from(value: DopArray<u8>) -> Result<Self, Self::Error> {
        if value.count != 8 || value.elements.len() != 8 {
            return Err("Dop2MacAddress expects a DopArray<u8> with count == 8");
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&value.elements);
        Ok(Dop2MacAddress(bytes))
    }
}

impl From<Dop2MacAddress> for DopArray<u8> {
    fn from(value: Dop2MacAddress) -> Self {
        DopArray {
            count: 8,
            elements: value.0.to_vec(),
        }
    }
}

/// Represents an IPv4 address encoded as 4 bytes in a `DopArray<u8>`.
///
/// The expected layout is a `DopArray<u8>` with `count == 4` and exactly 4 elements.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dop2IpV4Adress(pub [u8; 4]);

impl fmt::Debug for Dop2IpV4Adress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
        )
    }
}

impl TryFrom<DopArray<u8>> for Dop2IpV4Adress {
    type Error = &'static str;

    fn try_from(value: DopArray<u8>) -> Result<Self, Self::Error> {
        if value.count != 4 || value.elements.len() != 4 {
            return Err("Dop2IpV4Adress expects a DopArray<u8> with count == 4");
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&value.elements);
        Ok(Dop2IpV4Adress(bytes))
    }
}

impl From<Dop2IpV4Adress> for DopArray<u8> {
    fn from(value: Dop2IpV4Adress) -> Self {
        DopArray {
            count: 4,
            elements: value.0.to_vec(),
        }
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

