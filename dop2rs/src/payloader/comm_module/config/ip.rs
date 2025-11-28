use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum WifiSecurityProtocol {
    Open = 0,
    WEP = 1,
    WPA = 2,
    WPA2 = 3,
    WPA3 = 4,
}

crate::impl_tryfrom_wrapper!(WifiSecurityProtocol, E8);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct XkmConfigIp {
    #[dop2field(1, Dop2Payloads::Boolean)]
    ip_auto: bool,

    #[dop2field(2, Dop2Payloads::ArrayU8)]
    ip_address: DopArray<u8>, // 4 bytes representing IPv4 address

    #[dop2field(3, Dop2Payloads::ArrayU8)]
    subnet_mask: DopArray<u8>, // 4 bytes representing IPv4 subnet mask

    #[dop2field(4, Dop2Payloads::ArrayU8)]
    gateway_address: DopArray<u8>, // 4 bytes representing IPv4 gateway

    #[dop2field(5, Dop2Payloads::Boolean)]
    dns_server_auto: bool,

    #[dop2field(6, Dop2Payloads::ArrayU8)]
    dns_server1: DopArray<u8>, // 4 bytes representing IPv4 DNS server 1

    #[dop2field(7, Dop2Payloads::ArrayU8)]
    dns_server2: DopArray<u8>, // 4 bytes representing IPv4 DNS server 2

    #[dop2field(8, Dop2Payloads::ArrayU8)]
    wifi_key: DopArray<u8>, // WiFi password (masked, 63 bytes)

    #[dop2field(9, Dop2Payloads::ArrayU8)]
    wifi_ssid: DopArray<u8>, // WiFi SSID (32 bytes, null-terminated string)

    #[dop2field(10, Dop2Payloads::E8)]
    wifi_security_type: WifiSecurityProtocol,

    #[dop2field(11, Dop2Payloads::E8)]
    wifi_channel: E8,
}

impl XkmConfigIp {
    pub const ATTRIBUTE_IDS: &[u16] = &[1573];
}

impl_tryfrom_dop2struct!(XkmConfigIp);

