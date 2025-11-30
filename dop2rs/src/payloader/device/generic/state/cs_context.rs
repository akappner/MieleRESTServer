use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::{AnnotatedBool, GenericU8, GenericU16};
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
pub struct CSContextParametersWasher {
    #[dop2field(1, Dop2Payloads::MStruct)]
    pub on_off: AnnotatedBool,
    #[dop2field(2, Dop2Payloads::MStruct)]
    pub water_level: GenericU16,
    #[dop2field(3, Dop2Payloads::MStruct)]
    pub water_inlet_way: GenericU8,
    #[dop2field(4, Dop2Payloads::MStruct)]
    pub speed: GenericU16,
    #[dop2field(5, Dop2Payloads::MStruct)]
    pub actuator_level: GenericU8,
    #[dop2field(6, Dop2Payloads::MStruct)]
    pub residual_moisture_resistance: AnnotatedBool,
    #[dop2field(7, Dop2Payloads::MStruct)]
    pub rss_calibration: AnnotatedBool,
    #[dop2field(8, Dop2Payloads::MStruct)]
    pub user_interface: AnnotatedBool,
}

impl_tryfrom_dop2struct!(CSContextParametersWasher);

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
// TODO: Make these optional where needed
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CSContext {
    #[dop2field(1, Dop2Payloads::E16)]
    pub program_id: ProgramIdOven,
    #[dop2field(2, Dop2Payloads::MStruct)]
    pub context_washer: Option<CSContextParametersWasher>,
    #[dop2field(3, Dop2Payloads::MStruct)]
    pub context_oven: Option<CSContextParametersOven>,
  //  #[dop2field(4, Dop2Payloads::MStruct)]
   // context_coffee_maker: CSContextParametersCoffeeMaker,
}

impl_tryfrom_dop2struct!(CSContext);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CSBarcode {
    #[dop2field(1, Dop2Payloads::ArrayU8)]
    pub(crate) partname: String,
    #[dop2field(2, Dop2Payloads::ArrayU8)]
    pub(crate) barcode: DopArray<u8>,
}

impl_tryfrom_dop2struct!(CSBarcode);

