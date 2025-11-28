use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct RsaKey {
    #[dop2field(1, Dop2Payloads::ArrayU8)]
    key: DopArray<u8>,
}

impl RsaKey {
    pub const ATTRIBUTE_IDS: &[u16] = &[287];
}

impl_tryfrom_dop2struct!(RsaKey);

