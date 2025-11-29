use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::comm_module::config::ip::WifiSecurityProtocol;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct XkmConfigSsidList {
    #[dop2field(1, Dop2Payloads::ArrayU8)]
    ssid: DopArray<u8>, // WiFi SSID (32 bytes, null-terminated string)

    #[dop2field(2, Dop2Payloads::E8)]
    wlan_security: WifiSecurityProtocol,

    #[dop2field(3, Dop2Payloads::I8)]
    rssi: i8, // Signal strength in dBm

    #[dop2field(4, Dop2Payloads::E8)]
    wifi_channel: E8,
}

impl_tryfrom_dop2struct!(XkmConfigSsidList);