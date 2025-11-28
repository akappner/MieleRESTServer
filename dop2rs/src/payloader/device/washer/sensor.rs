use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::{AnnotatedU8, AnnotatedU16};

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct Sensor {
    #[dop2field(1, Dop2Payloads::MStruct)]
    pub water_level: AnnotatedU16,
    #[dop2field(2, Dop2Payloads::MStruct)]
    pub water_inlet_way: AnnotatedU16,
    #[dop2field(3, Dop2Payloads::MStruct)]
    pub spin_speed: AnnotatedU16,
    #[dop2field(4, Dop2Payloads::MStruct)]
    pub door_switch: AnnotatedU8,
    #[dop2field(5, Dop2Payloads::MStruct)]
    pub door_lock_switch: AnnotatedU8,
    #[dop2field(6, Dop2Payloads::MStruct)]
    pub wps_switch: AnnotatedU8,
    #[dop2field(7, Dop2Payloads::MStruct)]
    pub twin_dos_switch_container1: AnnotatedU8,
    #[dop2field(8, Dop2Payloads::MStruct)]
    pub twin_dos_switch_container2: AnnotatedU8,
    #[dop2field(9, Dop2Payloads::MStruct)]
    pub ntc_temperature1: AnnotatedU16,
    #[dop2field(10, Dop2Payloads::MStruct)]
    pub ntc_temperature2: AnnotatedU16,
    #[dop2field(11, Dop2Payloads::MStruct)]
    pub lance_contact: AnnotatedU8,
    #[dop2field(12, Dop2Payloads::MStruct)]
    pub peak_load_signal: AnnotatedU8,
    #[dop2field(13, Dop2Payloads::MStruct)]
    pub detected_cap: AnnotatedU8,
    #[dop2field(14, Dop2Payloads::MStruct)]
    pub dispenser_drawer_switch: AnnotatedU8,
    #[dop2field(15, Dop2Payloads::MStruct)]
    pub steam_unit_temperature: AnnotatedU16,
    #[dop2field(16, Dop2Payloads::MStruct)]
    pub sens_coiner_payment: AnnotatedU8,
}

impl Sensor {
    pub const ATTRIBUTE_IDS: &[u16] = &[6193];
}

impl_tryfrom_dop2struct!(Sensor);

