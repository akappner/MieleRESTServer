use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum FileOperation {
    Open = 0,
    Write = 1,
    Close = 2,
    Discard = 3,
    Sha256 = 4,
    DiscardValidationFailed = 5,
    Delete = 6,
    Read = 7,
    Finalize = 8,
    Invalid = 255,
}

crate::impl_tryfrom_wrapper!(FileOperation, E8);

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum FileOperationStatus {
    NoError = 0,
    CannotOpen = 1,
    FileNotFound = 2,
    EndOfFile = 3,
    WriteFailed = 4,
    VerifyFailed = 5,
    FileDiscard = 6,
    OutOfMemory = 7,
    ReadInProgress = 8,
    Invalid = 255,
}

crate::impl_tryfrom_wrapper!(FileOperationStatus, E8);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
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

impl FileTransfer {
    pub const ATTRIBUTE_IDS: &[u16] = &[336];
}

impl_tryfrom_dop2struct!(FileTransfer);

