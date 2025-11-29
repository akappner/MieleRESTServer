use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SwLinkInfo {
    #[dop2field(1, Dop2Payloads::U16)]
    pub(crate) id: u16,
    #[dop2field(2, Dop2Payloads::ArrayU8)]
    pub(crate) date: String,
    #[dop2field(3, Dop2Payloads::ArrayU8)]
    pub(crate) time: String,
}

impl_tryfrom_dop2struct!(SwLinkInfo);

