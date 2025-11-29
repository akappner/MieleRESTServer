use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;


#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ProgramStepInfoOven 
{
    #[dop2field(1, Dop2Payloads::U8 )]
    step_number : u8,
    #[dop2field(2, Dop2Payloads::E8 )]
    step_type : E8,
    #[dop2field(3, Dop2Payloads::U16 )]
    operation_mode : u16,
    #[dop2field(4, Dop2Payloads::U16 )]
    temperature_setpoint : u8,
    #[dop2field(5, Dop2Payloads::U8 )]
    set_grill_level : u8,
    #[dop2field(6, Dop2Payloads::U8 )]
    mw_power : u8,
    #[dop2field(7, Dop2Payloads::U32 )]
    duration : u32,
    #[dop2field(8, Dop2Payloads::U16 )]
    moistset_core_temperature : u16, // TODO: add remaining fields
}

impl_tryfrom_dop2struct!(ProgramStepInfoOven);
