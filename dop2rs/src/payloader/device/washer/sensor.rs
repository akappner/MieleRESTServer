use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::{AnnotatedU8, AnnotatedU16, AnnotatedI16, AnnotatedBool};

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct Sensor {
    #[dop2field(1, Dop2Payloads::MStruct)]
    pub water_level: AnnotatedU16,
    #[dop2field(2, Dop2Payloads::MStruct)]
    pub water_inlet_way: AnnotatedU8,
    #[dop2field(3, Dop2Payloads::MStruct)]
    pub spin_speed: AnnotatedI16,
    #[dop2field(4, Dop2Payloads::MStruct)]
    pub door_switch: AnnotatedBool,
    #[dop2field(5, Dop2Payloads::MStruct)]
    pub door_lock_switch: AnnotatedBool,
    #[dop2field(6, Dop2Payloads::MStruct)]
    pub wps_switch: AnnotatedBool,
    #[dop2field(7, Dop2Payloads::MStruct)]
    pub twin_dos_switch_container1: AnnotatedBool,
    #[dop2field(8, Dop2Payloads::MStruct)]
    pub twin_dos_switch_container2: AnnotatedBool,
    #[dop2field(9, Dop2Payloads::MStruct)]
    pub ntc_temperature1: AnnotatedU8,
    #[dop2field(10, Dop2Payloads::MStruct)]
    pub ntc_temperature2: AnnotatedU8,
    #[dop2field(11, Dop2Payloads::MStruct)]
    pub lance_contact: AnnotatedBool,
    #[dop2field(12, Dop2Payloads::MStruct)]
    pub peak_load_signal: AnnotatedBool,
    #[dop2field(13, Dop2Payloads::MStruct)]
    pub detected_cap: AnnotatedU16,
    #[dop2field(14, Dop2Payloads::MStruct)]
    pub dispenser_drawer_switch: AnnotatedBool,
    #[dop2field(15, Dop2Payloads::MStruct)]
    pub steam_unit_temperature: AnnotatedU16,
    #[dop2field(16, Dop2Payloads::MStruct)]
    pub sens_coiner_payment: AnnotatedBool,
}

impl Sensor {
    pub const ATTRIBUTE_IDS: &[u16] = &[6193];
}

impl_tryfrom_dop2struct!(Sensor);

