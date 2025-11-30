use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::parser::DopArray;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ProgramList {
    #[dop2field(1, Dop2Payloads::U8)]
    valid: u8,
    #[dop2field(2, Dop2Payloads::ArrayU16)]
    program_ids: DopArray<u16>,
    #[dop2field(3, Dop2Payloads::ArrayU16)]
    remaining_time: DopArray<u16>,
    #[dop2field(4, Dop2Payloads::ArrayU16)]
    temperature: DopArray<u16>,
    #[dop2field(5, Dop2Payloads::ArrayU8)]
    temperature_info: DopArray<u8>,
}

impl_tryfrom_dop2struct!(ProgramList);

