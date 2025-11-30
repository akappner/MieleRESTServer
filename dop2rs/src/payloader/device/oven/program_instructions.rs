use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::E16;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ProgramInstructionsCA {
    #[dop2field(1, Dop2Payloads::E16)]
    pub info_id: E16,
    #[dop2field(2, Dop2Payloads::E16)]
    pub message_id: E16,
    #[dop2field(3, Dop2Payloads::ArrayU64)]
    pub value: DopArray<u64>,
}

impl_tryfrom_dop2struct!(ProgramInstructionsCA);

