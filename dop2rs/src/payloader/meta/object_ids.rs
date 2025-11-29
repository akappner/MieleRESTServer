use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::meta::link_info::SwLinkInfo;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SysObjectId {
    #[dop2field(1, Dop2Payloads::U16)]
    pub(crate) object_id: u16,
    #[dop2field(2, Dop2Payloads::U16)]
    pub(crate) instances: u16,
    #[dop2field(3, Dop2Payloads::U16)]
    pub(crate) auth_read: u16,
    #[dop2field(4, Dop2Payloads::U16)]
    pub(crate) auth_write: u16,
    #[dop2field(5, Dop2Payloads::U16)]
    pub(crate) auth_subscribe: u16,
}

impl_tryfrom_dop2struct!(SysObjectId);

/// SYS_SoftwareIds - Software IDs structure (attribute 17)
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SoftwareIds {
    #[dop2field(1, Dop2Payloads::U16)]
    pub(crate) num_of_valid_sw_ids: u16,
    #[dop2field(2, Dop2Payloads::AStruct)]
    pub(crate) sw_ids: Vec<SwLinkInfo>,
}

impl_tryfrom_dop2struct!(SoftwareIds);

