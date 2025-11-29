use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct RsaKey {
    #[dop2field(1, Dop2Payloads::ArrayU8)]
    key: DopArray<u8>,
}

impl_tryfrom_dop2struct!(RsaKey);

