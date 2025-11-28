use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::helper::types::Dop2TimestampUtc;

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FailureListItem {
    #[dop2field(1, Dop2Payloads::U32)]
    failure_code: u32,
    #[dop2field(2, Dop2Payloads::Boolean)]
    present_now: bool,
}

impl_tryfrom_dop2struct!(FailureListItem);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FailureList {
    #[dop2field(1, Dop2Payloads::AStruct)]
    items: Vec<FailureListItem>,
}

impl FailureList {
    pub const ATTRIBUTE_IDS: &[u16] = &[148];
}

impl_tryfrom_dop2struct!(FailureList);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct Failure {
    #[dop2field(1, Dop2Payloads::U32)]
    failure_code: u32,
    #[dop2field(2, Dop2Payloads::Boolean)]
    active: bool,
    #[dop2field(3, Dop2Payloads::U16)]
    occurrence_frequency: u16,
    #[dop2field(4, Dop2Payloads::U64)]
    occurrence_time: Dop2TimestampUtc,
    #[dop2field(5, Dop2Payloads::U32)]
    operation_seconds: u32,
    #[dop2field(6, Dop2Payloads::U16)]
    prog_id: u16,
    #[dop2field(7, Dop2Payloads::U16)]
    block_number: u16,
}

impl Failure {
    pub const ATTRIBUTE_IDS: &[u16] = &[117];
}

impl_tryfrom_dop2struct!(Failure);

