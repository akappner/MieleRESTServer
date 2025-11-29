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
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq, IntoPrimitive)]
enum ApplianceState
{
    Unknown,
    Off,
    Synchronizing,
    Initializing,
    Normal,
    Demonstration,
    Service,
    Error,
    CheckAppliance,
    Standby,
    Supervisory,
    ShowWindow
}
#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq, IntoPrimitive)]
enum OperationState
{
    Unknown,
    EndOfLine,
    Service,
    Settings,
    InitialSettings,
    SelectProgram,
    RunDelay,
}
#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq, IntoPrimitive)]
enum ProcessState
{
    Unknown,
    NoProgram,
    ProgramSelected,
    ProgramStart,
    ProgramRunning
}

crate::impl_tryfrom_wrapper!(ApplianceState, E8);

crate::impl_tryfrom_wrapper!(ProcessState, E8);

crate::impl_tryfrom_wrapper!(OperationState, E8);
