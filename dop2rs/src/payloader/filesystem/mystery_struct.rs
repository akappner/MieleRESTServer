use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

/// Filesystem structure for attribute 1593 as seen in `heisluftplus.json`.
///
/// Field mapping (from JSON):
/// 1: Unsigned16
/// 2: Unsigned32
/// 3: Unsigned64
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct HeisluftPlus {
    #[dop2field(1, Dop2Payloads::U16)]
    request_id: u16,

    #[dop2field(2, Dop2Payloads::U32)]
    value: u32,

    #[dop2field(3, Dop2Payloads::U64)]
    extra: u64,
}

impl_tryfrom_dop2struct!(HeisluftPlus);


