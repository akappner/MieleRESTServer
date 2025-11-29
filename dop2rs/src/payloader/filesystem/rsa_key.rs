use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Clone, PartialEq, Eq, AssocTypes)]
pub struct RsaKey {
    #[dop2field(1, Dop2Payloads::ArrayU8)]
    key: DopArray<u8>,
}

impl_tryfrom_dop2struct!(RsaKey);

impl std::fmt::Debug for RsaKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RsaKey")
            .field("key", &format!("DopArray<u8>(count: {}, hex: {})", self.key.count, self.key.to_hex_str()))
            .finish()
    }
}

