use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

/// GLOBAL_EnumUpdateState – state of the firmware update process.
#[repr(u8)]
#[derive(
    Debug,
    Clone,
    TryFromPrimitive,
    IntoPrimitive,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    strum_macros::Display,
)]
pub enum UpdateState {
    Deactivated = 0,
    ReadyIdle = 1,
    Downloading = 2,
    DownloadedNotValidated = 3,
    DownloadedValidationFailed = 4,
    DownloadedValidated = 5,
    InstallationStarted = 6,
    UpdateInProgress = 7,
    DownloadedValidationFailedRsa = 8,
    DownloadedValidationFailedAuthorization = 9,
    DownloadedHashing = 10,
    FileSizeTooLarge = 11,
    UpdateAborted = 12,
    Erasing = 13,
    DownloadedValidating = 14,
    DownloadedBasedOnIdMismatch = 15,
    DownloadedValidationFailedNoTar = 16,
    DownloadedValidationFailedWrongFormat = 17,
    AbortedAfterFatalError = 18,
    AbortedByCustomerService = 19,
    AbortedCouldNotCreateUpdateList = 20,
    AbortedFeatureMismatch = 21,
    Unknown = 255,
}

crate::impl_tryfrom_wrapper!(UpdateState, E8);

/// FT_UpdateControl (attribute 170 on unit 15)
///
/// Mirrors `DOP2UpdateControl` in `MieleDop2Structures.py`:
///  - update_state     (field 1, enum)
///  - filename         (field 2, string)
///  - flash_accessible (field 3, bool)
///  - progress         (field 4, u16)
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct UpdateControl {
    #[dop2field(1, Dop2Payloads::E8)]
    update_state: UpdateState,

    #[dop2field(2, Dop2Payloads::MString)]
    filename: String,

    #[dop2field(3, Dop2Payloads::Boolean)]
    flash_accessible: bool,

    #[dop2field(4, Dop2Payloads::U16)]
    progress: u16,
}

impl_tryfrom_dop2struct!(UpdateControl);


