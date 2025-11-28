use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::AnnotatedU8;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ActuatorData {
    #[dop2field(1, Dop2Payloads::MStruct)]
    pub heater1: AnnotatedU8,
    #[dop2field(2, Dop2Payloads::MStruct)]
    pub lye_pump: AnnotatedU8,
    #[dop2field(3, Dop2Payloads::MStruct)]
    pub intensive_flow_pump: AnnotatedU8,
    #[dop2field(4, Dop2Payloads::MStruct)]
    pub valve1: AnnotatedU8,
    #[dop2field(5, Dop2Payloads::MStruct)]
    pub valve2: AnnotatedU8,
    #[dop2field(6, Dop2Payloads::MStruct)]
    pub water_distributor_motor: AnnotatedU8,
    #[dop2field(7, Dop2Payloads::MStruct)]
    pub heater2: AnnotatedU8,
    #[dop2field(8, Dop2Payloads::MStruct)]
    pub twin_dos_pump1: AnnotatedU8,
    #[dop2field(9, Dop2Payloads::MStruct)]
    pub twin_dos_pump2: AnnotatedU8,
    #[dop2field(10, Dop2Payloads::MStruct)]
    pub steam_heater: AnnotatedU8,
    #[dop2field(11, Dop2Payloads::MStruct)]
    pub steam_pump: AnnotatedU8,
    #[dop2field(12, Dop2Payloads::MStruct)]
    pub dos_rel1: AnnotatedU8,
    #[dop2field(13, Dop2Payloads::MStruct)]
    pub dos_rel2: AnnotatedU8,
    #[dop2field(14, Dop2Payloads::MStruct)]
    pub dos_rel3: AnnotatedU8,
    #[dop2field(15, Dop2Payloads::MStruct)]
    pub dos_rel4: AnnotatedU8,
    #[dop2field(16, Dop2Payloads::MStruct)]
    pub dos_rel5: AnnotatedU8,
    #[dop2field(17, Dop2Payloads::MStruct)]
    pub dos_rel6: AnnotatedU8,
    #[dop2field(18, Dop2Payloads::MStruct)]
    pub act_coiner_end: AnnotatedU8,
    #[dop2field(19, Dop2Payloads::MStruct)]
    pub act_coiner_operation: AnnotatedU8,
    #[dop2field(20, Dop2Payloads::MStruct)]
    pub sens_peak_load: AnnotatedU8,
}

impl ActuatorData {
    pub const ATTRIBUTE_IDS: &[u16] = &[6192];
}

impl_tryfrom_dop2struct!(ActuatorData);

