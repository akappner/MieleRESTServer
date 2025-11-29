use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use super::enums::MieleDeviceId;

/// GLOBAL_CETS_AvailableDevice - Available device information for cooking end time synchronization
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CetsAvailableDevice {
    #[dop2field(1, Dop2Payloads::ArrayU8)]
    pub(crate) serial_no: DopArray<u8>,
    #[dop2field(2, Dop2Payloads::E8)]
    pub(crate) device_type: MieleDeviceId,
    #[dop2field(4, Dop2Payloads::Boolean)]
    pub(crate) is_synchable: bool,
    #[dop2field(5, Dop2Payloads::U32)]
    pub(crate) duration_total: u32,
    #[dop2field(6, Dop2Payloads::U32)]
    pub(crate) end_at: u32,
    #[dop2field(7, Dop2Payloads::Boolean)]
    pub(crate) is_synchronization_selected: bool,
    #[dop2field(8, Dop2Payloads::Boolean)]
    pub(crate) is_synchronization_active: bool,
}

impl_tryfrom_dop2struct!(CetsAvailableDevice);

/// GLOBAL_CETS_CloudStatus - Cooking End Time Synchronization Status (attribute 412)
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CookingEndTimeSynchronizationStatus {
    #[dop2field(2, Dop2Payloads::U32)]
    pub(crate) longest_duration: u32,
    #[dop2field(3, Dop2Payloads::U32)]
    pub(crate) end_at: u32,
    #[dop2field(4, Dop2Payloads::AStruct)]
    pub(crate) available_devices: Vec<CetsAvailableDevice>,
}

impl_tryfrom_dop2struct!(CookingEndTimeSynchronizationStatus);

