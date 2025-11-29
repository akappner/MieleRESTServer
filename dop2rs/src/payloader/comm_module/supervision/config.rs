use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::device::generic::enums::MieleDeviceId;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SuperVisionListConfig {
        #[dop2field(1, Dop2Payloads::Boolean)]
        active : bool,
        #[dop2field(2, Dop2Payloads::Boolean)]
        on_error_only : bool,
        #[dop2field(3, Dop2Payloads::Boolean)]
        is_time_master : bool,
   //     #[dop2field(4, Dop2Payloads::Boolean)]
     //   on_error_only : bool,
       // #[dop2field(5, Dop2Payloads::Boolean)]
        //active : bool,
        //#[dop2field(6, Dop2Payloads::Boolean)]
        //on_error_only : bool,
}

impl_tryfrom_dop2struct!(SuperVisionListConfig);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SuperVisionListItem {
        /// Raw device ID (matches `deviceId` in Python)
        #[dop2field(1, Dop2Payloads::U8)]
        device_id: u8,

        /// Device ID enum (matches `deviceIdEnum` in Python)
        #[dop2field(2, Dop2Payloads::E8)]
        device_id_enum: MieleDeviceId,

        /// Device name bytes (null-terminated, padded string)
        ///
        /// In Python this is exposed as `deviceName` using `getStringAtIndex(3)`.
        #[dop2field(3, Dop2Payloads::ArrayU8)]
        device_name: DopArray<u8>,

        /// Connection state (matches `connectionState` in Python)
        #[dop2field(4, Dop2Payloads::U8)]
        connection_state: u8,

        /// Whether the device is displayed in the SuperVision list
        #[dop2field(5, Dop2Payloads::Boolean)]
        display_setting: bool,

        /// Whether signal tones are enabled
        #[dop2field(6, Dop2Payloads::Boolean)]
        signal_setting: bool,

        /// Whether SuperVision is activated for this device
        #[dop2field(7, Dop2Payloads::Boolean)]
        super_vision_activate: bool,

        /// Screen to show for this SuperVision entry
        #[dop2field(8, Dop2Payloads::E8)]
        super_vision_display_screen_enum: E8,

        /// Text to show for this SuperVision entry
        #[dop2field(9, Dop2Payloads::E16)]
        super_vision_display_text_enum: E16,

        /// Timestamp / UTC time (field 10 in decoded tree)
        #[dop2field(10, Dop2Payloads::U32)]
        utc_time: u32,

        /// Time offset (field 11 in decoded tree)
        #[dop2field(11, Dop2Payloads::U32)]
        time_offset: u32,

        /// Process data (field 12, array of I32)
        #[dop2field(12, Dop2Payloads::ArrayI32)]
        process_data: DopArray<i32>,

        /// Program type (field 13, E8)
        #[dop2field(13, Dop2Payloads::E8)]
        program_type: E8,

        /// Program phase (field 14, E16)
        #[dop2field(14, Dop2Payloads::E16)]
        program_phase: E16,

        /// Door signal (field 15, bool)
        #[dop2field(15, Dop2Payloads::Boolean)]
        signal_door: bool,

        /// Info signal (field 16, bool)
        #[dop2field(16, Dop2Payloads::Boolean)]
        signal_info: bool,

        /// Long address / identifier string bytes (field 17)
        #[dop2field(17, Dop2Payloads::ArrayU8)]
        long_address: DopArray<u8>,

        /// Remote enable (field 18, U8)
        #[dop2field(18, Dop2Payloads::U8)]
        remote_enable: u8,

        /// Standby state / state enum (field 19, E8)
        #[dop2field(19, Dop2Payloads::E8)]
        standby_state: E8,

        /// Reserved / unknown U8 (field 20)
        #[dop2field(20, Dop2Payloads::U8)]
        field20: u8,

        /// Reserved / unknown U8 (field 21)
        #[dop2field(21, Dop2Payloads::U8)]
        field21: u8,

        /// Reserved / unknown U8 (field 22)
        #[dop2field(22, Dop2Payloads::U8)]
        field22: u8,

        /// Reserved / unknown enum (field 23, E8)
        #[dop2field(23, Dop2Payloads::E8)]
        field23: E8,

        /// Program ID associated with this SuperVision entry (field 24, U16)
        #[dop2field(24, Dop2Payloads::U16)]
        program_id: u16,
}

impl_tryfrom_dop2struct!(SuperVisionListItem);