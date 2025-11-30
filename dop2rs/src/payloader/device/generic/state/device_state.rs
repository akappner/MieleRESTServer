use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::parser::DopArray;
use crate::payloader::helper::types::E16;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct RemoteEnable {
    #[dop2field(1, Dop2Payloads::Boolean)]
    field1: bool,
    #[dop2field(2, Dop2Payloads::Boolean)]
    field2: bool,
    #[dop2field(3, Dop2Payloads::Boolean)]
    field3: bool,
    #[dop2field(4, Dop2Payloads::Boolean)]
    field4: bool,
}

impl_tryfrom_dop2struct!(RemoteEnable);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceState {
    #[dop2field(1, Dop2Payloads::E8)]
    main_state: ValueInterpretation,
    #[dop2field(2, Dop2Payloads::MStruct)]
    remote_enable: RemoteEnable,
    #[dop2field(3, Dop2Payloads::E8)]
    program_type: ValueInterpretation,
    #[dop2field(4, Dop2Payloads::E16)]
    program_id: E16,
    #[dop2field(5, Dop2Payloads::E16)]
    program_phase: E16,
    #[dop2field(6, Dop2Payloads::U32)]
    start_time_relative: u32,
    #[dop2field(7, Dop2Payloads::U32)]
    remaining_time: u32,
    #[dop2field(8, Dop2Payloads::U32)]
    elapsed_time_relative: u32,
    #[dop2field(9, Dop2Payloads::ArrayI16)]
    process_temperature_set: DopArray<i16>,
    #[dop2field(10, Dop2Payloads::ArrayI16)]
    process_temperature_current: DopArray<i16>,
    #[dop2field(11, Dop2Payloads::ArrayI16)]
    core_temperature_set: DopArray<i16>,
    #[dop2field(12, Dop2Payloads::ArrayI16)]
    core_temperature_current: DopArray<i16>,
    #[dop2field(13, Dop2Payloads::Boolean)]
    signal_door: bool,
    #[dop2field(14, Dop2Payloads::Boolean)]
    signal_info: bool,
    #[dop2field(15, Dop2Payloads::U16)]
    spinning_speed: u16,
    #[dop2field(16, Dop2Payloads::E8)]
    drying_step: ValueInterpretation,
    #[dop2field(17, Dop2Payloads::E8)]
    light_state: ValueInterpretation,
    #[dop2field(18, Dop2Payloads::E8)]
    standby_state: ValueInterpretation,
    #[dop2field(19, Dop2Payloads::I32)]
    field19: i32,
    #[dop2field(20, Dop2Payloads::I32)]
    field20: i32,
    #[dop2field(21, Dop2Payloads::I32)]
    field21: i32,
}

impl_tryfrom_dop2struct!(DeviceState);

