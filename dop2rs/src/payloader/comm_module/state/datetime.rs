use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::Dop2TimestampUtc;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DateTimeInfo {
    #[dop2field(1, Dop2Payloads::U64)]
    utc_time: Dop2TimestampUtc,
    #[dop2field(2, Dop2Payloads::I32)]
    utc_offset: i32,
}

impl_tryfrom_dop2struct!(DateTimeInfo);

