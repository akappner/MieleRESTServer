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
        pub const ATTRIBUTE_IDS : &[u16] = &[130];
        pub const ATTRIBUTE : u16 = 130; // typically unit 14
        pub fn to_dop2_payload (&self) -> Result<Dop2Payloads, String> // TODO: make this call to_dop2_struct
        {
            let mut fields: Vec<TaggedDopField> = vec!();
            let request_id_payload : Dop2Payloads = Dop2Payloads::E8(self.request_id.clone().into());
            let request_id_field : TaggedDopField = TaggedDopField{ field_index: 1, tag: Dop2PayloadsKind::from(request_id_payload.clone()), value: request_id_payload};
            fields.push(request_id_field);
            Ok(Dop2Payloads::MStruct(Dop2Struct::from_fields (fields)))
        }

        pub fn to_dop2_struct (&self) -> Result<Dop2Struct, String>
        {
            let mut fields: Vec<TaggedDopField> = vec!();
            let request_id_payload : Dop2Payloads = Dop2Payloads::E8(self.request_id.clone().into());
            let request_id_field : TaggedDopField = TaggedDopField{ field_index: 1, tag: Dop2PayloadsKind::from(request_id_payload.clone()), value: request_id_payload};
            fields.push(request_id_field);
            Ok(Dop2Struct::from_fields (fields))
        }
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