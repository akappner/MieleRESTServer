use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::{AnnotatedU8, AnnotatedU16, AnnotatedTimeStamp, GenericU8, GenericU16};

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct PSAttributesCCA {
    #[dop2field(1, Dop2Payloads::E16)]
    prog_phase: E16,
    #[dop2field(2, Dop2Payloads::E16)]
    prog_sub_phase: E16,
    #[dop2field(3, Dop2Payloads::MStruct)]
    progress: AnnotatedU16,
    #[dop2field(6, Dop2Payloads::MStruct)]
    display_temperature: AnnotatedU16,
    #[dop2field(7, Dop2Payloads::MStruct)]
    display_core_temperature: AnnotatedU16,
    #[dop2field(21, Dop2Payloads::MStruct)]
    temperature_setpoint: Option<AnnotatedU16>,
    #[dop2field(22, Dop2Payloads::MStruct)]
    moisture_setpoint: Option<AnnotatedU8>,
    #[dop2field(24, Dop2Payloads::MStruct)]
    power_setpoint: Option<AnnotatedU8>,
    #[dop2field(26, Dop2Payloads::MStruct)]
    start_time: Option<AnnotatedTimeStamp>,
    #[dop2field(29, Dop2Payloads::MStruct)]
    next_action_time: Option<AnnotatedTimeStamp>,
}

impl_tryfrom_dop2struct!(PSAttributesCCA);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct PSContextParametersOven {
    #[dop2field(1, Dop2Payloads::MStruct)]
    grill_level: GenericU8,
    #[dop2field(2, Dop2Payloads::MStruct)]
    moisture: GenericU8,
    #[dop2field(5, Dop2Payloads::MStruct)]
    level: GenericU8,
    #[dop2field(6, Dop2Payloads::MStruct)]
    temperature: GenericU16,
}

impl_tryfrom_dop2struct!(PSContextParametersOven);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct PSContext {
    #[dop2field(4, Dop2Payloads::MStruct)]
    context_oven: PSContextParametersOven,
    #[dop2field(7, Dop2Payloads::MStruct)]
    attributes_oven: PSAttributesCCA,
}

impl_tryfrom_dop2struct!(PSContext);

