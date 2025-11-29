use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FileInfo {
    #[dop2field(1, Dop2Payloads::MString)]
    filename: String,

    #[dop2field(2, Dop2Payloads::ArrayU8)]
    sha256: DopArray<u8>,

    #[dop2field(3, Dop2Payloads::U32)]
    current_size: u32,

    #[dop2field(4, Dop2Payloads::U32)]
    max_size: u32,
}

impl_tryfrom_dop2struct!(FileInfo);

