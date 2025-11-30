use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct LastUpdateInfo {
    #[dop2field(1, Dop2Payloads::MString)]
    filename: String,
}

impl_tryfrom_dop2struct!(LastUpdateInfo);

