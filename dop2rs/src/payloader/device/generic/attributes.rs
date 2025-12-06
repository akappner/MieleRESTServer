use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

/// GLOBAL_EnumBeanContainerState - Bean container state
#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum BeanContainerState {
    Unknown = 0,
    Empty = 1,
    Full = 2,
}

crate::impl_tryfrom_wrapper!(BeanContainerState, E8);
/// GLOBAL_StateBeanContainer - Bean container info with multiple compartments
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct BeanContainerInfo {
    #[dop2field(1, Dop2Payloads::E8)]
    compartment_one: BeanContainerState,
    #[dop2field(2, Dop2Payloads::E8)]
    compartment_two: BeanContainerState,
    #[dop2field(3, Dop2Payloads::E8)]
    compartment_three: BeanContainerState,
}

impl_tryfrom_dop2struct!(BeanContainerInfo);
/// GLOBAL_EnumDoorLock - Door lock state
#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum DoorLock {
    Unlocked = 0,
    Locking = 1,
    Locked = 2,
    Unlocking = 3,
}

crate::impl_tryfrom_wrapper!(DoorLock, E8);

/// GLOBAL_EnumFasciaPanelState - Front panel state
#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum FasciaPanelState {
    Undefined = 0,
    Closed = 1,
    Opened = 2,
    Closing = 3,
    Opening = 4,
    ErrorOpening = 5,
    ErrorClosing = 6,
}

crate::impl_tryfrom_wrapper!(FasciaPanelState, E8);

/// GLOBAL_DeviceAttributesCCA - Coffee machine device attributes
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceAttributesCCA {
    #[dop2field(2, Dop2Payloads::U32)]
    milk_cleaning_cntr: u32,
    #[dop2field(3, Dop2Payloads::U32)]
    brew_unit_degreasing_cntr: u32,
    #[dop2field(4, Dop2Payloads::U32)]
    manual_descaling_cntr: u32,
    #[dop2field(5, Dop2Payloads::U32)]
    drinks_till_descaling: u32,
    #[dop2field(6, Dop2Payloads::U32)]
    drinks_till_brew_unit_degrease: u32,
    #[dop2field(7, Dop2Payloads::E8)]
    state_descaling_cartridge: E8,
    #[dop2field(8, Dop2Payloads::E8)]
    state_cleaning_cartridge: E8,
    #[dop2field(9, Dop2Payloads::MStruct)]
    state_bean_container: BeanContainerInfo,
    #[dop2field(11, Dop2Payloads::E8)]
    door_lock: DoorLock,
    #[dop2field(12, Dop2Payloads::U16)]
    programs_till_descaling: u16,
    #[dop2field(13, Dop2Payloads::U32)]
    minutes_of_heating: u32,
    #[dop2field(14, Dop2Payloads::U32)]
    minutes_of_heating_descaling_threshold: u32,
    #[dop2field(15, Dop2Payloads::U8)]
    level_water_tank: u8,
    #[dop2field(16, Dop2Payloads::E8)]
    fresh_water_tank_state: E8,
    #[dop2field(17, Dop2Payloads::E8)]
    front_panel_state: FasciaPanelState,
    #[dop2field(18, Dop2Payloads::Boolean)]
    sabbat_active: bool,
    #[dop2field(19, Dop2Payloads::Boolean)]
    descaling_required: bool,
    #[dop2field(20, Dop2Payloads::Boolean)]
    cleaning_required: bool,
    #[dop2field(21, Dop2Payloads::Boolean)]
    traide_fair_mode_active: bool,
    #[dop2field(22, Dop2Payloads::U32)]
    supported_program_groups: u32,
    #[dop2field(23, Dop2Payloads::U16)]
    descaling_cartridge_level: u16,
    #[dop2field(24, Dop2Payloads::U16)]
    cleaning_cartridge_level: u16,
    #[dop2field(25, Dop2Payloads::U8)]
    days_till_milk_cleaning: u8,
    #[dop2field(26, Dop2Payloads::Boolean)]
    object_data_changed: bool,
    #[dop2field(27, Dop2Payloads::Boolean)]
    push_to_talk: bool,
    #[dop2field(28, Dop2Payloads::Boolean)]
    initial_grinding: bool,
    #[dop2field(29, Dop2Payloads::U16)]
    op_last_instance_changed: Option<u16>,
    #[dop2field(30, Dop2Payloads::U16)]
    op_last_instance_changed_counter: Option<u16>,
}

impl_tryfrom_dop2struct!(DeviceAttributesCCA);

