use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::filesystem::enums::FileOperation;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FileWrite {
    #[dop2field(1, Dop2Payloads::E8)]
    file_operation: FileOperation,

    #[dop2field(2, Dop2Payloads::MString)]
    file_name: String,

    #[dop2field(3, Dop2Payloads::U32)]
    address: u32,

    #[dop2field(4, Dop2Payloads::U32)]
    size: u32,

    #[dop2field(5, Dop2Payloads::ArrayU8)]
    data: DopArray<u8>,
}

impl_tryfrom_dop2struct!(FileWrite);

