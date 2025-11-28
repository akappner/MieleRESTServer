use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceAttributesCCA {
    #[dop2field(11, Dop2Payloads::E8)]
    door_lock: E8,
}

impl_tryfrom_dop2struct!(DeviceAttributesCCA);

