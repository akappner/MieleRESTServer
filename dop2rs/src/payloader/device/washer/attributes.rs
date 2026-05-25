use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::{
    AnnotatedU8, AnnotatedU16, AnnotatedU32, AnnotatedI8, AnnotatedBool,
    GenericU8, GenericU16, GenericU32, GenericBool,
};
use crate::payloader::device::washer::enums::{DoorState, EcoFeedbackFilterState};

/// GLOBAL_DeviceOptCapDetectInfo - OptCap detection info (stub, fields TBD)
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceOptCapDetectInfo {}

impl_tryfrom_dop2struct!(DeviceOptCapDetectInfo);

/// GLOBAL_DosContainerInfo - Dosing container info (AutoDos cartridges)
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DosContainerInfo {
    #[dop2field(2, Dop2Payloads::U8)]
    pub info_bit_mask: u8,
    #[dop2field(3, Dop2Payloads::U16)]
    pub container_size: u16,
    #[dop2field(4, Dop2Payloads::U16)]
    pub initial_dosage: u16,
    #[dop2field(5, Dop2Payloads::U16)]
    pub current_dosage: u16,
    #[dop2field(6, Dop2Payloads::MStruct)]
    pub filling_level: AnnotatedU8,
    #[dop2field(7, Dop2Payloads::MStruct)]
    pub clean_level: AnnotatedU8,
    #[dop2field(8, Dop2Payloads::MStruct)]
    pub info_clean_level: AnnotatedU8,
    #[dop2field(9, Dop2Payloads::MStruct)]
    pub max_clean_level: AnnotatedU8,
}

impl_tryfrom_dop2struct!(DosContainerInfo);

/// GLOBAL_DeviceAttributesDWTDWM - Washing machine / tumble dryer / washer-dryer device attributes
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceAttributesDWTDWM {
    #[dop2field(1, Dop2Payloads::E8)]
    pub door_state: DoorState,
//    #[dop2field(2, Dop2Payloads::MStruct)]
//    pub opt_cap_detect_info: DeviceOptCapDetectInfo,
    #[dop2field(3, Dop2Payloads::MStruct)]
    pub eco_feedback_energy_consumption_last_prog: GenericU16,
    #[dop2field(4, Dop2Payloads::MStruct)]
    pub eco_feedback_water_consumption_last_prog: GenericU16,
    #[dop2field(5, Dop2Payloads::MStruct)]
    pub eco_feedback_total_energy_consumption: GenericU32,
    #[dop2field(6, Dop2Payloads::MStruct)]
    pub eco_feedback_total_water_consumption: GenericU32,
    #[dop2field(7, Dop2Payloads::AStruct)]
    pub dos_container_info: Vec<DosContainerInfo>,
    #[dop2field(8, Dop2Payloads::U8)]
    pub salt_container: u8,
    #[dop2field(9, Dop2Payloads::U8)]
    pub rinse_aid: u8,
    #[dop2field(10, Dop2Payloads::U16)]
    pub tabs: u16,
    #[dop2field(11, Dop2Payloads::E8)]
    pub eco_feedback_filter_state: EcoFeedbackFilterState,
    #[dop2field(12, Dop2Payloads::Boolean)]
    pub eco_feedback_filter_state_valid: bool,
//    #[dop2field(13, Dop2Payloads::MStruct)]
//    pub demo_menu_access: GenericBool,
    #[dop2field(14, Dop2Payloads::MStruct)]
    pub motoe_position: AnnotatedU8,
    #[dop2field(15, Dop2Payloads::MStruct)]
    pub door_opening_from_extern_allowed: AnnotatedBool,
    #[dop2field(16, Dop2Payloads::Boolean)]
    pub cartridge_detected: bool,
    #[dop2field(20, Dop2Payloads::MStruct)]
    pub eco_feedback_total_energy_costs: AnnotatedU32,
    #[dop2field(21, Dop2Payloads::MStruct)]
    pub eco_feedback_total_water_costs: AnnotatedU32,
    #[dop2field(22, Dop2Payloads::MStruct)]
    pub eco_feedback_energy_costs_last_prog: AnnotatedU32,
    #[dop2field(23, Dop2Payloads::MStruct)]
    pub eco_feedback_water_costs_last_prog: AnnotatedU32,
    #[dop2field(24, Dop2Payloads::MStruct)]
    pub current_temperature: AnnotatedU8,
    #[dop2field(25, Dop2Payloads::MStruct)]
    pub interior_light_on: AnnotatedBool,
    #[dop2field(26, Dop2Payloads::MStruct)]
    pub heating_on: AnnotatedBool,
    #[dop2field(27, Dop2Payloads::MStruct)]
    pub hygiene_level: GenericU8,
    #[dop2field(28, Dop2Payloads::MStruct)]
    pub current_spin_speed: AnnotatedU16,
    #[dop2field(29, Dop2Payloads::MStruct)]
    pub current_power_consumption: AnnotatedU16,
    #[dop2field(30, Dop2Payloads::MStruct)]
    pub current_water_level: AnnotatedU16,
    #[dop2field(31, Dop2Payloads::MStruct)]
    pub current_water_volume: AnnotatedU16,
    #[dop2field(32, Dop2Payloads::MStruct)]
    pub current_residual_moisture: AnnotatedI8,
    #[dop2field(33, Dop2Payloads::MStruct)]
    pub show_salt_deficit: AnnotatedBool,
    #[dop2field(34, Dop2Payloads::MStruct)]
    pub show_rinse_aid_deficit: AnnotatedBool,
    #[dop2field(35, Dop2Payloads::MStruct)]
    pub hygiene_counter: AnnotatedU16,
    #[dop2field(36, Dop2Payloads::MStruct)]
    pub max_reached_temperature: AnnotatedU8,
}

impl_tryfrom_dop2struct!(DeviceAttributesDWTDWM);
