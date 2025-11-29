use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use super::supported_applications::SupportedApplications;

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum DeviceType {
    None,
    Washer,
    Dryer,
    WasherSemiPro,
    DryerSemiPro,
    WasherPro,
    DryerPro,
    Dishwasher,
    DishwasherSemiPro,
    DishwasherPro,
    Cooker,
    Microwave,
    Oven,
    OvenMicrowaveCombo,
}

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum ProtocolType {
    Unknown = 0,
    Uart = 1,
    MeterBusDop1 = 2,
    MeterBusDop2 = 3,
    HdrDop2 = 4,
    HdrMaci,
    SpiMaci,
    SdioMaci,
    UartMaci,
    DbusDop2 = 200,
    TodDop2 = 201,
    UsbDop2 = 202,
}

crate::impl_tryfrom_wrapper!(DeviceType, E8);
crate::impl_tryfrom_wrapper!(ProtocolType, E8);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceIdent {
    #[dop2field(1, Dop2Payloads::E8)]
    device_type: DeviceType,
    #[dop2field(2, Dop2Payloads::E8)]
    protocol_type: ProtocolType,
    #[dop2field(5, Dop2Payloads::MStruct)]
    supported_apps: SupportedApplications,
    #[dop2field(9, Dop2Payloads::E16)]
    rf_variant: E16,
}

impl_tryfrom_dop2struct!(DeviceIdent);

