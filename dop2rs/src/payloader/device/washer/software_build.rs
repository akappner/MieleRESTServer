use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SoftwareBuild {
    #[dop2field(1, Dop2Payloads::MString)]
    date: String,
    #[dop2field(2, Dop2Payloads::MString)]
    time: String,
    #[dop2field(3, Dop2Payloads::U16)]
    id: u16,
    #[dop2field(4, Dop2Payloads::U16)]
    version: u16,
}

impl_tryfrom_dop2struct!(SoftwareBuild);

