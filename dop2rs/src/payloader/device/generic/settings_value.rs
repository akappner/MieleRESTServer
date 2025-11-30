use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::device::generic::settings::SfId;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SfValue {
    #[dop2field(1, Dop2Payloads::E16)]
    sf_id: SfId,
    #[dop2field(2, Dop2Payloads::U8)]
    validity: u8,
    #[dop2field(3, Dop2Payloads::E8)]
    value_interpretation: ValueInterpretation,
    #[dop2field(4, Dop2Payloads::I16)]
    current_value: i16,
    #[dop2field(5, Dop2Payloads::I16)]
    min: i16,
    #[dop2field(6, Dop2Payloads::I16)]
    max: i16,
    #[dop2field(7, Dop2Payloads::I16)]
    default: i16,
    #[dop2field(8, Dop2Payloads::U16)]
    list_ref: u16,
    #[dop2field(9, Dop2Payloads::U8)]
    step_size: u8,
    #[dop2field(10, Dop2Payloads::Boolean)]
    ext_value: bool,
    #[dop2field(11, Dop2Payloads::Boolean)]
    fine_adjusted: bool,
}

impl_tryfrom_dop2struct!(SfValue);

