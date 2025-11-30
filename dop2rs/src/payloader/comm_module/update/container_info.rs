use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct UpdateContainerInformation {
    #[dop2field(1, Dop2Payloads::U8)]
    update_state: u8,
    #[dop2field(2, Dop2Payloads::ArrayU8)]
    field2: Option<String>,
    #[dop2field(3, Dop2Payloads::U32)]
    field3: Option<u32>,
    #[dop2field(4, Dop2Payloads::U32)]
    field4: Option<u32>,
    #[dop2field(5, Dop2Payloads::U32)]
    field5: Option<u32>,
    #[dop2field(6, Dop2Payloads::U32)]
    field6: Option<u32>,
    #[dop2field(7, Dop2Payloads::U32)]
    field7: Option<u32>,
    #[dop2field(8, Dop2Payloads::U32)]
    field8: Option<u32>,
    #[dop2field(9, Dop2Payloads::U32)]
    field9: Option<u32>,
    #[dop2field(10, Dop2Payloads::U32)]
    field10: Option<u32>,
    #[dop2field(11, Dop2Payloads::U32)]
    crc32: u32,
}

impl_tryfrom_dop2struct!(UpdateContainerInformation);

