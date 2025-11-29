use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::{AnnotatedBool, GenericU8};
use crate::payloader::device::generic::program_selection::enums::ProgramIdOven;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CSContextParametersOven {
    #[dop2field(1, Dop2Payloads::MStruct)]
    open: AnnotatedBool,
    #[dop2field(2, Dop2Payloads::MStruct)]
    lock: AnnotatedBool,
    #[dop2field(3, Dop2Payloads::MStruct)]
    on: AnnotatedBool,
    #[dop2field(4, Dop2Payloads::MStruct)]
    level: GenericU8,
}

impl_tryfrom_dop2struct!(CSContextParametersOven);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CSContextParametersCoffeeMaker {
    #[dop2field(5, Dop2Payloads::MStruct)]
    ceramic_valve: GenericU8,
    #[dop2field(6, Dop2Payloads::MStruct)]
    brewing_unit: GenericU8,
    #[dop2field(7, Dop2Payloads::MStruct)]
    pump: GenericU8,
    #[dop2field(8, Dop2Payloads::MStruct)]
    spout: GenericU8,
    #[dop2field(12, Dop2Payloads::MStruct)]
    fan: GenericU8,
}

impl_tryfrom_dop2struct!(CSContextParametersCoffeeMaker);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CSContext {
    #[dop2field(1, Dop2Payloads::E16)]
    pub(crate) program_id: ProgramIdOven,
    #[dop2field(3, Dop2Payloads::MStruct)]
    context_oven: CSContextParametersOven,
    #[dop2field(4, Dop2Payloads::MStruct)]
    context_coffee_maker: CSContextParametersCoffeeMaker,
}

impl_tryfrom_dop2struct!(CSContext);

