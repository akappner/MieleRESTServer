use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use super::enums::{ProgramIdOven, SelectionType};

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct PsSelect {
    #[dop2field(1, Dop2Payloads::E16)]
    pub(crate) program_id: ProgramIdOven,
    #[dop2field(2, Dop2Payloads::U16)]
    pub(crate) selection_parameter: u16,
    #[dop2field(3, Dop2Payloads::E8)]
    pub(crate) selection_type: SelectionType,
}

impl PsSelect {
    pub const ATTRIBUTE_IDS: &[u16] = &[1577];
}

impl_tryfrom_dop2struct!(PsSelect);

