use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, EnumIter, EnumString, strum_macros::Display, IntoPrimitive)]
pub enum UserRequestOven {
    Nop = 0,
    Start = 1,
    Stop = 2,
    Pause = 3,
    StartDelay = 8,
    DoorOpen = 11,
    DoorClose = 12,
    LightOn = 13,
    LightOff = 14,
    FactorySettingReset = 15,
    SwitchOn = 16,
    Next = 17,
    Back = 18,
    SwitchOff = 19,
    ResetPinCode = 20,
    KeepWarm = 21,
    Step = 22,
    StartRemoteUpdateInstall = 23,
    ProgramStop = 54,
    ProgramAbort = 55,
    ProgramFinalize = 56,
    ProgramSave = 61,
    MotorizedFrontPanelOpen = 65,
    MotorizedFrontPanelClose = 66,
    HoldingBreak = 68,
    HoldingStart = 69,
    WifiOff = 112,
}

crate::impl_tryfrom_wrapper!(UserRequestOven, E16);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct UserRequest {
    #[dop2field(1, Dop2Payloads::E16)]
    pub request_id: UserRequestOven,
}

impl UserRequest {
    pub const ATTRIBUTE_IDS: &[u16] = &[1583];
}

impl_tryfrom_dop2struct!(UserRequest);

