use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum NotificationAckOption {
    None = 0,
    Ok,
    Continue,
    Abort,
    Back,
    Start,
    Stop,
    Yes,
    No,
    StepOver,
    SwitchOff,
    Change,
}

crate::impl_tryfrom_wrapper!(NotificationAckOption, E8);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct NotificationInfo {
    #[dop2field(1, Dop2Payloads::E16)]
    id: E16,
    #[dop2field(3, Dop2Payloads::ArrayE8)]
    ack_options: Vec<NotificationAckOption>,
}

impl_tryfrom_dop2struct!(NotificationInfo);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct MessageInfo {
    #[dop2field(1, Dop2Payloads::E16)]
    id: E16,
    #[dop2field(3, Dop2Payloads::ArrayE8)]
    ack_options: Vec<NotificationAckOption>,
}

impl_tryfrom_dop2struct!(MessageInfo);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ErrorInfo {
    #[dop2field(1, Dop2Payloads::U32)]
    id: u32,
    #[dop2field(2, Dop2Payloads::ArrayE8)]
    ack_options: Vec<NotificationAckOption>,
}

impl_tryfrom_dop2struct!(ErrorInfo);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct QueryInfo {
    #[dop2field(1, Dop2Payloads::E16)]
    id: E16,
}

impl_tryfrom_dop2struct!(QueryInfo);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceNotifications {
    #[dop2field(2, Dop2Payloads::AStruct)]
    messages: Vec<MessageInfo>,
    #[dop2field(3, Dop2Payloads::AStruct)]
    errors: Vec<ErrorInfo>,
}

impl_tryfrom_dop2struct!(DeviceNotifications);

