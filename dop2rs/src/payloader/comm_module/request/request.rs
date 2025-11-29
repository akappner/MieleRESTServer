use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum XkmRequestId {
    None = 0,
    ResetXkm = 1,
    FactoryResetXkm = 2,
    OpenSoftAccessPointEndUser = 3,
    OpenSoftAccessPointCustomerService = 45,
    ShutdownXkm = 46,
    MieleSmartConnect = 47,
}

crate::impl_tryfrom_wrapper!(XkmRequestId, E8);


#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct XkmRequest {
    #[dop2field(1, Dop2Payloads::E8)]
        pub(crate) request_id : XkmRequestId
}
    impl XkmRequest
    {
        pub const ATTRIBUTE : u16 = 130; // typically unit 14
/*        pub fn from_parse_tree (payload: Dop2Payloads) -> Result<Self, String>
        {
            if let Dop2Payloads::MStruct(x)=payload // if payload cannot be deserialized as struct, fail
            {
                if let Dop2Payloads::E8(request_id) = x.fields[0].value
                {
                    return Ok(XkmRequest{request_id: request_id.0.try_into().unwrap() })
                }
                else 
                {
                    println!("{:?}", x.fields[0].value);                   
                    return Err("Entity mismatch while deserializing XKMRequest field".to_string())
                }
            }
            Err("Entity mismatch".to_string())
        }
*/
}


impl_tryfrom_dop2struct!(XkmRequest);