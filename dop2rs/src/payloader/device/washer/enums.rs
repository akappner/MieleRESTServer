use crate::payloader::prelude::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum DetergentType {
    NoDetergent = 0,
    UltraPhase1 = 1,
    UltraPhase2 = 2,
    UltraWhite = 3,
    UltraColor = 4,
}

crate::impl_tryfrom_wrapper!(DetergentType, E8);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum DryingStep {
    ExtraDry = 0,
    NormalPlus = 1,
    Normal = 2,
    SlightlyDry = 3,
    HandIron1 = 4,
    HandIron2 = 5,
    MachineIron = 6,
    HygieneDry = 7,
}

crate::impl_tryfrom_wrapper!(DryingStep, E8);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum Light {
    NotSupported = 0,
    Enabled = 1,
    Disabled = 2,
}

crate::impl_tryfrom_wrapper!(Light, E8);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum ProgramType {
    BuiltInFunction = 1,
    UserDefined = 2,
    Automatic = 3,
    CleaningProgram = 4,
    CustomerService = 5,
    Helper = 6,
}

crate::impl_tryfrom_wrapper!(ProgramType, E8);

