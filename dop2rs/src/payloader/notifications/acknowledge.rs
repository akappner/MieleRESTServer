use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

/// CS_OperationCycleCounter / NotificationAcknowledge (attribute 138)
///
/// This structure shares a signature with "OperationRuntimeCounter".
/// Fields E16(1), E16(2), U32(3), E16(4) and E8(5) must be included.
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct NotificationAcknowledge {
    #[dop2field(1, Dop2Payloads::E16)]
    pub(crate) notification_info_id: E16,
    #[dop2field(2, Dop2Payloads::E16)]
    pub(crate) notification_message_id: E16,
    #[dop2field(3, Dop2Payloads::U32)]
    pub(crate) notification_error_id: u32,
    #[dop2field(4, Dop2Payloads::E16)]
    pub(crate) notification_query_id: E16,
    #[dop2field(5, Dop2Payloads::E8)]
    pub(crate) acknowledge_option: E8,
   // #[dop2field(6, Dop2Payloads::U64)]
   // pub(crate) field6: u64,
}

impl_tryfrom_dop2struct!(NotificationAcknowledge);

