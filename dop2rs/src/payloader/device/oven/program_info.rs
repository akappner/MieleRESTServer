use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ProgramInfoOven 
{
#[dop2field(1, Dop2Payloads::U8 )]
stepNumber : u8,

#[dop2field(2, Dop2Payloads::U8 )]
currentStep : u8,

#[dop2field(5, Dop2Payloads::Boolean )]
startDelay : bool, // TODO: add remaining fields

}


impl ProgramInfoOven
    {
        pub const ATTRIBUTE_IDS : &[u16] = &[213]; // always in unit 2?
    }

    #[macro_use]
    use crate::macros;

    impl_tryfrom_dop2struct!(ProgramInfoOven);