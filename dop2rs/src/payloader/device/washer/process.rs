use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::{AnnotatedU8, AnnotatedU16, AnnotatedI16, AnnotatedI32, AnnotatedBool, GenericU8, GenericU16};

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct Process {
    #[dop2field(1, Dop2Payloads::MStruct)]
    pub block_number: GenericU8,
    #[dop2field(2, Dop2Payloads::MStruct)]
    pub block_step: GenericU8,
    #[dop2field(3, Dop2Payloads::MStruct)]
    pub load_level: GenericU8,
    #[dop2field(4, Dop2Payloads::MStruct)]
    pub remaining_time_in_minutes: AnnotatedU16,
    #[dop2field(5, Dop2Payloads::MStruct)]
    pub program_phase: AnnotatedU16,
    #[dop2field(6, Dop2Payloads::MStruct)]
    pub spin_profile_number: AnnotatedU8,
    #[dop2field(7, Dop2Payloads::MStruct)]
    pub current_level: AnnotatedI16,
    #[dop2field(8, Dop2Payloads::MStruct)]
    pub heater_relay: AnnotatedBool,
    #[dop2field(9, Dop2Payloads::MStruct)]
    pub lye_pump: AnnotatedBool,
    #[dop2field(10, Dop2Payloads::MStruct)]
    pub circulation_pump: AnnotatedBool,
    #[dop2field(11, Dop2Payloads::MStruct)]
    pub cold_water_valve: AnnotatedBool,
    #[dop2field(12, Dop2Payloads::MStruct)]
    pub hot_water_valve: AnnotatedBool,
    #[dop2field(13, Dop2Payloads::MStruct)]
    pub water_distributor_current_position: AnnotatedU16,
    #[dop2field(14, Dop2Payloads::MStruct)]
    pub water_distributor_target_position: AnnotatedU16,
    #[dop2field(15, Dop2Payloads::MStruct)]
    pub fu_temperature: AnnotatedU16,
    #[dop2field(16, Dop2Payloads::MStruct)]
    pub energy_consumed: AnnotatedU16,
    #[dop2field(17, Dop2Payloads::MStruct)]
    pub water_consumed_in_litres: AnnotatedU16,
    #[dop2field(18, Dop2Payloads::MStruct)]
    pub wash_block: GenericU16,
    #[dop2field(19, Dop2Payloads::MStruct)]
    pub wash_block_index: GenericU8,
    #[dop2field(20, Dop2Payloads::MStruct)]
    pub hygiene_counter: AnnotatedU8,
    #[dop2field(21, Dop2Payloads::MStruct)]
    pub total_impulses: AnnotatedI32,
    #[dop2field(22, Dop2Payloads::MStruct)]
    pub water_inlet_suction_time_1: AnnotatedI16,
    #[dop2field(23, Dop2Payloads::MStruct)]
    pub water_inlet_suction_time_2: AnnotatedI16,
    #[dop2field(24, Dop2Payloads::MStruct)]
    pub heating_target_temperature: GenericU8,
    #[dop2field(25, Dop2Payloads::MStruct)]
    pub heating_energy: AnnotatedU16,
    #[dop2field(26, Dop2Payloads::MStruct)]
    pub heating_time: AnnotatedU16,
    #[dop2field(27, Dop2Payloads::MStruct)]
    pub rpm_current: AnnotatedI16,
    #[dop2field(28, Dop2Payloads::MStruct)]
    pub current_unbalance_mass: AnnotatedU8,
    #[dop2field(29, Dop2Payloads::MStruct)]
    pub steam_release_result: AnnotatedU8,
    #[dop2field(30, Dop2Payloads::MStruct)]
    pub sew_flow_state: AnnotatedU8,
    #[dop2field(31, Dop2Payloads::MStruct)]
    pub sew_active: AnnotatedU8,
    #[dop2field(32, Dop2Payloads::MStruct)]
    pub sew_load_quantity: AnnotatedU16,
    #[dop2field(33, Dop2Payloads::MStruct)]
    pub sew_load_level: AnnotatedU8,
    #[dop2field(34, Dop2Payloads::MStruct)]
    pub sew_total_filled_quantity_u16: AnnotatedU16,
    #[dop2field(36, Dop2Payloads::MStruct)]
    pub sew_heating_time: AnnotatedU16,
    #[dop2field(39, Dop2Payloads::MStruct)]
    pub sew_mtv_moment_of_inertia_x10000: AnnotatedU16,
    #[dop2field(40, Dop2Payloads::MStruct)]
    pub sew_correction_factor_p_measurement_new: AnnotatedU16,
    #[dop2field(41, Dop2Payloads::MStruct)]
    pub gs_program_number: AnnotatedU8,
    #[dop2field(58, Dop2Payloads::MStruct)]
    pub step_advance_switching: GenericU8,
    #[dop2field(59, Dop2Payloads::MStruct)]
    pub throttle_temperature: AnnotatedU16,
    #[dop2field(60, Dop2Payloads::MStruct)]
    pub abort_error: AnnotatedU16,
    #[dop2field(61, Dop2Payloads::MStruct)]
    pub unbalance_mode: AnnotatedU8,
    #[dop2field(62, Dop2Payloads::MStruct)]
    pub tb_kg_result_bss1200: AnnotatedU16,
    #[dop2field(63, Dop2Payloads::MStruct)]
    pub tb_kg_result_bss600: AnnotatedU16,
    #[dop2field(64, Dop2Payloads::MStruct)]
    pub tb_kg_result_bss800: AnnotatedU16,
    #[dop2field(65, Dop2Payloads::MStruct)]
    pub tb_kg_result_integral110: AnnotatedU16,
    #[dop2field(66, Dop2Payloads::MStruct)]
    pub tb_kg_result_integral95: AnnotatedU16,
}

impl_tryfrom_dop2struct!(Process);
