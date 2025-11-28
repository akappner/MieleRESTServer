use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SupportedApplications {
    #[dop2field(1, Dop2Payloads::Boolean)]
    miele_at_home: bool,
    #[dop2field(2, Dop2Payloads::Boolean)]
    remote_vision: bool,
    #[dop2field(3, Dop2Payloads::Boolean)]
    super_vision: bool,
    #[dop2field(4, Dop2Payloads::Boolean)]
    smart_grid: bool,
    #[dop2field(5, Dop2Payloads::Boolean)]
    mobile_control: bool,
    #[dop2field(6, Dop2Payloads::Boolean)]
    unknown1: bool,
    #[dop2field(7, Dop2Payloads::Boolean)]
    unknown2: bool,
    #[dop2field(8, Dop2Payloads::Boolean)]
    voice_control: bool,
    #[dop2field(9, Dop2Payloads::Boolean)]
    unknown3: bool,
    #[dop2field(10, Dop2Payloads::Boolean)]
    feature_list: bool,
    #[dop2field(11, Dop2Payloads::Boolean)]
    wash_to_dry: bool,
}

impl_tryfrom_dop2struct!(SupportedApplications);

