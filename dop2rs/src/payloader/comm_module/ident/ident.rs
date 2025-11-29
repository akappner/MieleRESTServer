use crate::payloader::prelude::*;
use crate::payloader::helper::types::Dop2MacAddress;
use crate::Dop2ParseTreeExpressible;

/// Representation of the XKM identification information (unit 14, attribute 1565).
///
/// This mirrors the layout from `DOP2XKMIdent` in `MieleDop2Structures.py`:
/// - applicationType
/// - moduleType
/// - softwareVersion
/// - softwareId
/// - macAddressWifi
/// - applicationScope
/// - macAddressLan
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct XkmIdent {
    /// Application type (enum in Python, stored as E8 here)
    #[dop2field(2, Dop2Payloads::E8)]
    application_type: E8,

    /// Module type (enum in Python, stored as E8 here)
    #[dop2field(3, Dop2Payloads::E8)]
    module_type: E8,

    /// Software version string bytes (e.g. "09.14")
    #[dop2field(4, Dop2Payloads::ArrayU8)]
    software_version: DopArray<u8>,

    /// Software ID
    #[dop2field(5, Dop2Payloads::U16)]
    software_id: u16,

    /// WiFi MAC address
    #[dop2field(6, Dop2Payloads::ArrayU8)]
    mac_address_wifi: Dop2MacAddress,

    /// Application scope (enum, stored as E8)
    #[dop2field(7, Dop2Payloads::E8)]
    application_scope: E8,

    /// LAN MAC address
    #[dop2field(8, Dop2Payloads::ArrayU8)]
    mac_address_lan: Dop2MacAddress,
}

impl_tryfrom_dop2struct!(XkmIdent);


