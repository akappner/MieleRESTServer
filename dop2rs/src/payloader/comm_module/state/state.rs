use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

/// Enum representing the XKM connection state (mirrors `EnumXKMState` in the
/// original implementation, but without the `GLOBAL_` prefix in the name
/// or the variants).
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
pub enum XkmState {
    Init = 0,
    NotConnected = 1,
    Connected = 2,
    ApInactive = 3,
    ApActive = 4,
    NetworkDeactivated = 7,
    Connecting = 8,
    ConnectedEthernet = 9,
}

crate::impl_tryfrom_wrapper!(XkmState, E8);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct XkmStateInfo {
    #[dop2field(1, Dop2Payloads::E8)]
    state: XkmState,

    #[dop2field(2, Dop2Payloads::U8)]
    signal_quality: u8,

    #[dop2field(3, Dop2Payloads::E8)]
    system_state: E8,

    #[dop2field(4, Dop2Payloads::E8)]
    request_active: E8,

    #[dop2field(5, Dop2Payloads::E8)]
    request_state: E8,

    #[dop2field(6, Dop2Payloads::E8)]
    sync_state: E8,

    #[dop2field(7, Dop2Payloads::Boolean)]
    config_state: bool,

    #[dop2field(8, Dop2Payloads::U16)]
    cloud_status: u16,

    #[dop2field(9, Dop2Payloads::U8)]
    connected_clients: u8,

    #[dop2field(10, Dop2Payloads::U16)]
    connected_system_peripherals: u16,

    #[dop2field(11, Dop2Payloads::U16)]
    wifi_freq_range: u16,

    #[dop2field(12, Dop2Payloads::E8)]
    wifi_channel: E8,

    #[dop2field(13, Dop2Payloads::I8)]
    rssi: i8,

    #[dop2field(14, Dop2Payloads::ArrayU8)]
    bssid: DopArray<u8>,

    #[dop2field(15, Dop2Payloads::E8)]
    bluetooth_state: E8,
}

impl_tryfrom_dop2struct!(XkmStateInfo);


