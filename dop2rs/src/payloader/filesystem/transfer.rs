use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::filesystem::enums::FileOperation;
use crate::payloader::filesystem::enums::FileOperationStatus;

#[derive(Clone, PartialEq, Eq, AssocTypes)]
pub struct FileTransfer {
    #[dop2field(1, Dop2Payloads::MString)]
    file_name: String,

    #[dop2field(2, Dop2Payloads::E8)]
    file_operation: E8,

    #[dop2field(3, Dop2Payloads::E8)]
    file_operation_status: E8,

    #[dop2field(4, Dop2Payloads::U32)]
    offset: u32,

    #[dop2field(5, Dop2Payloads::U32)]
    file_size: u32,

    #[dop2field(6, Dop2Payloads::U16)]
    data_length: u16,

    #[dop2field(8, Dop2Payloads::U32)]
    dummy: u32,

    #[dop2field(7, Dop2Payloads::ArrayU8)]
    data: DopArray<u8>,
}

impl_tryfrom_dop2struct!(FileTransfer);

impl std::fmt::Debug for FileTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileTransfer")
            .field("file_name", &self.file_name)
            .field("file_operation", &self.file_operation)
            .field("file_operation_status", &self.file_operation_status)
            .field("offset", &self.offset)
            .field("file_size", &self.file_size)
            .field("data_length", &self.data_length)
            .field("dummy", &self.dummy)
            .field("data", &format!("DopArray<u8>(count: {}, hex: {})", self.data.count, self.data.to_hex_str()))
            .finish()
    }
}

