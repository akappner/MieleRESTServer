use crate::payloader::prelude::*;

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

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum FileAccessMode {
    NotAccessible = 0,
    Read = 1,
    Write = 2,
    ReadWrite = 3,
    ReadDirectory = 4,
    WriteDirectory = 5,
    ReadWriteDirectory = 6,
    Invalid = 255,
}

crate::impl_tryfrom_wrapper!(FileAccessMode, E8);

