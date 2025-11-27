use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

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

impl SuperVisionListConfig
{
    pub const ATTRIBUTE_IDS : &[u16] = &[1570];
}

impl_tryfrom_dop2struct!(SuperVisionListConfig);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SuperVisionListItem {
    /*    #[dop2field(1, Dop2Payloads::Boolean)]
        active : bool,
        #[dop2field(2, Dop2Payloads::Boolean)]
        on_error_only : bool,
        #[dop2field(3, Dop2Payloads::Boolean)]
        active : bool,
   //     #[dop2field(4, Dop2Payloads::Boolean)]
     //   on_error_only : bool, */
       // #[dop2field(5, Dop2Payloads::Boolean)]
        //active : bool,
        //#[dop2field(6, Dop2Payloads::Boolean)]
        //on_error_only : bool,
}

impl SuperVisionListItem
{
    pub const ATTRIBUTE_IDS : &[u16] = &[1571];
}

impl_tryfrom_dop2struct!(SuperVisionListItem);