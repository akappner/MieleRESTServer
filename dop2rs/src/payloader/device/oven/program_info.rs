use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ProgramInfoOven 
{
#[dop2field(1, Dop2Payloads::U8 )]
step_number : u8,

#[dop2field(2, Dop2Payloads::U8 )]
    current_step : u8,

#[dop2field(5, Dop2Payloads::Boolean )]
    start_delay : bool, // TODO: add remaining fields

}


impl ProgramInfoOven
    {
        pub const ATTRIBUTE_IDS : &[u16] = &[213]; // always in unit 2?
    }

    

    impl_tryfrom_dop2struct!(ProgramInfoOven);