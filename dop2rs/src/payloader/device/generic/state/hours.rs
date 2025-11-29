use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CSHoursOfOperation {
    #[dop2field(1, Dop2Payloads::U32)]
    hours_of_operation: u32,
    #[dop2field(2, Dop2Payloads::U32)]
    hours_of_operation_before_replacement: u32,
    #[dop2field(3, Dop2Payloads::U32)]
    hours_of_operation_since_last_maintenance: u32,
    #[dop2field(4, Dop2Payloads::U32)]
    hours_of_operation_mode1: u32,
    #[dop2field(5, Dop2Payloads::U32)]
    hours_of_operation_mode2: u32,
}

impl_tryfrom_dop2struct!(CSHoursOfOperation);

