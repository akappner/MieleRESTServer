use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FeatureListOven {
    #[dop2field(1, Dop2Payloads::U16)]
    device_id: u16,
}

impl_tryfrom_dop2struct!(FeatureListOven);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FeatureList {
    #[dop2field(1, Dop2Payloads::E8)]
    device_id: E8,
    #[dop2field(2, Dop2Payloads::E8)]
    device_class: E8,
    #[dop2field(3, Dop2Payloads::U16)]
    device_sub_class: u16,
    #[dop2field(5, Dop2Payloads::Boolean)]
    has_search: bool,
    #[dop2field(6, Dop2Payloads::Boolean)]
    has_camera: bool,
    #[dop2field(7, Dop2Payloads::E8)]
    device_id_sub_type: E8,
    #[dop2field(131, Dop2Payloads::MStruct)]
    feature_list_oven: FeatureListOven,
}

impl FeatureList {
    pub const ATTRIBUTE_IDS: &[u16] = &[348];
}

impl_tryfrom_dop2struct!(FeatureList);

