use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceCombiState {
        #[dop2field(1, Dop2Payloads::E8)]
        appliance_state : ApplianceState,
        #[dop2field(2, Dop2Payloads::E8)]
        operation_state : OperationState,
        #[dop2field(3, Dop2Payloads::E8)]
        process_state : ProcessState
}

impl_tryfrom_dop2struct!(DeviceCombiState);

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, PartialEq, Eq, IntoPrimitive)]
pub enum ApplianceState
{
    Unknown = 0,
    Off = 1,
    Synchronizing = 2,
    Initializing = 3,
    Normal = 4,
    Demonstration = 5,
    Service = 6,
    Error = 7,
    Check = 8,
    Standby = 9,
    Supervisory = 10,
    ShowWindow = 11,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, PartialEq, Eq, IntoPrimitive)]
pub enum OperationState
{
    Unknown = 0,
    EndOfLine = 1,
    Service = 2,
    Settings = 3,
    InitialSettings = 4,
    SelectProgram = 5,
    RunProgram = 6,
    RunDelay = 7,
    RunMaintenanceProcess = 8,
    VoltageBrownout = 9,
    WelcomeScreen = 10,
    Locked = 11,
    TimeSettingScreen = 12,
    DisplayOff = 15,
    ColdRising = 21,
    NormalRinsing = 22,
    EmergencyStop = 32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, PartialEq, Eq, IntoPrimitive)]
pub enum ProcessState
{
    Unknown = 0,
    NoProgram = 1,
    ProgramSelected = 2,
    ProgramStarted = 3,
    ProgramRunning = 4,
    ProgramStop = 5,
}

crate::impl_tryfrom_wrapper!(ApplianceState, E8);

crate::impl_tryfrom_wrapper!(ProcessState, E8);

crate::impl_tryfrom_wrapper!(OperationState, E8);
