use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;


#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ProgramStepInfoOven 
{
    #[dop2field(1, Dop2Payloads::U8 )]
    stepNumber : u8,
    #[dop2field(2, Dop2Payloads::E8 )]
    stepType : E8,
    #[dop2field(3, Dop2Payloads::U16 )]
    operationMode : u16,
    #[dop2field(4, Dop2Payloads::U16 )]
    temperatureSetpoint : u8,
    #[dop2field(5, Dop2Payloads::U8 )]
    setGrillLevel : u8,
    #[dop2field(6, Dop2Payloads::U8 )]
    mwPower : u8,
    #[dop2field(7, Dop2Payloads::U32 )]
    duration : u32,
    #[dop2field(8, Dop2Payloads::U16 )]
    moistsetCoreTemperature : u16, // TODO: add remaining fields
}

impl ProgramStepInfoOven
    {
        pub const ATTRIBUTE_IDS : &[u16] = &[214]; // always in unit 2?
    }



impl_tryfrom_dop2struct!(ProgramStepInfoOven);
