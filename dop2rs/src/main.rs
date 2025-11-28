
use clap::Parser;
use num_enum::{TryFromPrimitive, IntoPrimitive};
use derive_more::From;

use payloader::device;
use payloads::{UnitIds};
use payloader::device::generic::state::combined::DeviceCombiState;
use payloader::device::generic::program_selection::enums::{ProgramIdOven, SelectionType};
use payloader::device::generic::request::UserRequestOven;
use payloader::device::generic::notifications::NotificationAckOption;
use payloader::device::generic::settings::SfId;
use payloader::device::generic::context::ShowMeHowId;
use payloader::device::generic::ident::ident::{DeviceType, ProtocolType};
use syn::token::{Struct, Type};

use crate::payloads::Dop2ParseTreeExpressible;

use paste::paste;

mod crypto;
mod device_api;

use strum_macros::{EnumIter, EnumString};

#[macro_use]
extern crate enum_kinds;

#[derive(Parser)]
#[command(name = "hex_parser")]
#[command(about = "DOP2 Recursive-descent parser")]
#[command(version)]


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum Dop2Type {
    Bool                        = 1,
    U8                          = 2,
    I8                          = 3,
    E8                          = 4,
    U16                         = 5,
    I16                         = 6,
    E16                         = 7,
    U32                         = 8,
    I32                         = 9,
    E32                         = 10,
    U64                         = 11,
    I64                         = 12,
    E64                         = 13,
    F32                         = 14,
    F64                         = 15,
    Struct                      = 16,
    ArrayBool                   = 17,
    ArrayU8                    = 18,
    ArrayI8                     = 19,
    ArrayE8                     = 20,
    ArrayU16                    = 21,
    ArrayI16                    = 22,
    ArrayE16                    = 23,
    ArrayU32                    = 24,
    ArrayI32                    = 25,
    ArrayE32                    = 26,
    ArrayU64                    = 27,
    ArrayI64                    = 28,
    ArrayE64                    = 29,
    ArrayF32                    = 30,
    ArrayF64                    = 31,
    MString                     = 32,  
    AStruct                     = 33,
}



#[allow(dead_code)]
#[derive(Debug)]
struct RootNode {
    unit: u16,
    attribute: u16,
    declared_length: u16,
    idx1 : u16,
    idx2 : u16,
    root_struct: Dop2Struct,
//    padding: DopPadding
}
impl RootNode 
{
    fn single (unit: u16, attribute: u16, root_struct: Dop2Struct)-> RootNode
    {
        
        RootNode {unit : unit, attribute : attribute, declared_length: 0, idx1: 0, idx2: 0, root_struct: root_struct}
    }
    fn has_more_siblings(&self)->bool
    {
        return self.idx1 == self.idx2;
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaggedDopField {
    tag: Dop2PayloadsKind,
    field_index : u16,
    value: Dop2Payloads,
}

impl TaggedDopField
{
    fn from_payload (field_index: u16, value: Dop2Payloads) -> TaggedDopField
    {
        let tag = Dop2PayloadsKind::from(&value);
        return TaggedDopField {field_index: field_index, tag: tag, value: value};
      
    }

    fn get_length (&self)->u16
    {
        let size = std::mem::size_of_val(&self.tag) + std::mem::size_of_val(&self.field_index);
        size.try_into().unwrap()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DopArray <T : Dop2PayloadExpressible+ToDop2Bytes>
{
    count : u16,
    elements : Vec<T>
}

impl<T> Into<Vec<T>> for DopArray<T>
where
    T: Dop2PayloadExpressible + ToDop2Bytes,
{
    fn into(self) -> Vec<T> {
        self.elements
    }
}

impl<T : Dop2PayloadExpressible + ToDop2Bytes> Dop2PayloadExpressible for DopArray<T>
{
     fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> 
    {
        let count = parser.take_u16().unwrap();
        let mut elements : Vec<T> = Vec::new();
        for x in 0..count
        {
            let element = T::parse(parser);
            elements.insert(x.into(), *element.unwrap());
        }
        Ok(Box::new(DopArray{count, elements}))
    }
}
impl<T: Dop2PayloadExpressible + ToDop2Bytes> ToDop2Bytes for DopArray<T>{

    fn to_bytes(self, vec: &mut Vec<u8>) {
        let count : u16 = self.elements.len().try_into().unwrap();
        vec.extend(count.to_be_bytes());
        self.elements.into_iter().for_each(|field| field.to_bytes(vec));
    }
}

macro_rules! newtype_int {
    ($name:ident, $inner:ty) => {

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
        pub struct $name($inner);
        impl ToDop2Bytes for $name {
            fn to_bytes(self, vec: &mut Vec<u8>) {
                vec.extend(self.0.to_be_bytes());
            }
        }

        impl Dop2PayloadExpressible for $name {
            fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> {
                let n = std::mem::size_of::<$inner>();
                let bytes = parser.take(n);
                let mut value: $inner = 0;
                for (i, b) in bytes.unwrap().into_iter().enumerate()
                {
                    //println!("{}", b);
                    value |= ((b as $inner) << ((n - 1 - i) * 8));
                }

                Ok(Box::new(value.into()))
            }
        }


    };
}

newtype_int!(E8, u8);
newtype_int!(E16, u16);
newtype_int!(E32, u32);
newtype_int!(E64, u64);

#[derive(Clone, Debug, PartialEq, Eq, EnumKind)]

#[enum_kind(Dop2PayloadsKind, derive(TryFromPrimitive), repr(u8))]
/* 
Bool                        = 1,
U8                          = 2,
I8                          = 3,
E8                          = 4,
U16                         = 5,
I16                         = 6,
E16                         = 7,
U32                         = 8,
I32                         = 9,
E32                         = 10,
U64                         = 11,
I64                         = 12,
E64                         = 13,
F32                         = 14,
F64                         = 15,
Struct                      = 16,
ArrayBool                   = 17,
ArrayU8                    = 18,
ArrayI8                     = 19,
ArrayE8                     = 20,
ArrayU16                    = 21,
ArrayI16                    = 22,
ArrayE16                    = 23,
ArrayU32                    = 24,
ArrayI32                    = 25,
ArrayE32                    = 26,
ArrayU64                    = 27,
ArrayI64                    = 28,
ArrayE64                    = 29,
ArrayF32                    = 30,
ArrayF64                    = 31,
MString                     = 32,  
AStruct                     = 33,
}*/
pub enum Dop2Payloads {
    Trash,
    Boolean(bool), // 1
    U8(u8),        // 2
    I8(i8),
    E8(E8),
    U16(u16),
    I16(i16),
    E16(E16),
    U32(u32),
    I32(i32),
    E32(E32),
    U64(u64),
    I64(i64),

    E64(E64),
    F32(i32),
    F64(i32),


    MStruct (Dop2Struct),
    ArrayBool (DopArray<bool>),
    ArrayU8 (DopArray<u8>),
    ArrayI8 (DopArray<i8>),
    ArrayE8 (DopArray<E8>),

    ArrayU16 (DopArray<u16>),
    ArrayI16 (DopArray<i16>),
    ArrayE16 (DopArray<E16>),

    ArrayU32 (DopArray<u32>),
    ArrayI32 (DopArray<i32>),
    ArrayE32 (DopArray<u32>),

    ArrayU64 (DopArray<u64>),
    ArrayI64 (DopArray<i32>),
    ArrayE64(DopArray<u64>),

    ArrayF32 (DopArray<u8>), // todo
    ArrayF64 (DopArray<i8>), // todo

    MString(String),
    AStruct (DopArray<Dop2Struct>),
}


// Generic* and Annotated* types moved to payloader::helper::types
pub use payloader::helper::types::*;

#[derive(Debug)]
struct DopPadding {
    bytes_of_padding : u8
}
impl DopPadding
{
    const PADDING_BYTE : u8 = 0x20;
    const PADDING_ALIGNMENT : u16 = 0x10;

    fn minimum_padding (builder: &Vec<u8>)->DopPadding
    {
        let current = builder.len() as u16;
        //println!("{}", current);
        return DopPadding {bytes_of_padding: ((DopPadding::PADDING_ALIGNMENT - (current % DopPadding::PADDING_ALIGNMENT)) % DopPadding::PADDING_ALIGNMENT) as u8 };
    }

    fn parse(parser: &mut Dop2Parser) -> Result<DopPadding, String>
    {
    let mut bytes_of_padding = 0u8;
    while !parser.is_empty() {
        let byte = parser.take_u8().map_err(|e| e.to_string())?;
        if byte == DopPadding::PADDING_BYTE {
            bytes_of_padding += 1;
        } else {
            // If we read a non-0x20 byte, backtrack and stop
            return Err(format!("Non-padding byte 0x{:02X} read", byte))
        }
    }

   
    Ok(DopPadding { bytes_of_padding })
    }
}

impl ToDop2Bytes for DopPadding
{

     fn to_bytes (self, vec: &mut Vec<u8>)
    {
        vec.extend(std::iter::repeat(DopPadding::PADDING_BYTE).take(self.bytes_of_padding.into()));
    }
}
trait Dop2PayloadExpressible {
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> ;
}

trait ToDop2Bytes 
{
    fn to_bytes (self, vec: &mut Vec<u8>);
}
impl Dop2PayloadExpressible for bool
{
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> 
    {
        let payload_byte = parser.take_u8().unwrap();
        if payload_byte >= 0x02
        {
            return Err("Invalid payload".to_string())
        }
        Ok(Box::new(payload_byte==0x01))
    }
}

impl ToDop2Bytes for bool
{
    fn to_bytes (self, vec: &mut Vec<u8>)
    {
        if self { vec.push(0x01); } else {vec.push(0x00); }
    }
}

impl Dop2PayloadExpressible for String
{
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> 
    {
        let length = parser.take_u16().unwrap();
        let mut result = String::new();
        let string_bytes = parser.take(length.into());
        for b in string_bytes.unwrap().into_iter() {
            // ASCII byte → char conversion
            result.push(b as char);

        }
        Ok(Box::new(result))
    }
}



impl_from_bytes!(u8);
impl_from_bytes!(u16);
impl_from_bytes!(u32);
impl_from_bytes!(u64);

impl_from_bytes!(i8);
impl_from_bytes!(i16);
impl_from_bytes!(i32);
impl_from_bytes!(i64);



impl_to_bytes!(u8);
impl_to_bytes!(u16);
impl_to_bytes!(u32);
impl_to_bytes!(u64);

impl_to_bytes!(i8);
impl_to_bytes!(i16);
impl_to_bytes!(i32);
impl_to_bytes!(i64);


impl ToDop2Bytes for String{
     fn to_bytes(self, vec: &mut Vec<u8>) {
               let ascii = self.into_bytes();
                vec.extend(ascii);
            }
}

impl TaggedDopField {
    fn to_bytes (self, vec: &mut Vec<u8>)
    {
        vec.extend(self.field_index.to_be_bytes());
        vec.push(self.tag as u8);
        
        match self.value {
            Dop2Payloads::Boolean(b)=> b.to_bytes(vec),
            Dop2Payloads::U8(payload)        => payload.to_bytes(vec),
            Dop2Payloads::U16(payload)       => payload.to_bytes(vec),
            Dop2Payloads::U32(payload)       => payload.to_bytes(vec),
            Dop2Payloads::U64(payload)       => payload.to_bytes(vec),
            Dop2Payloads::I8(payload)       => payload.to_bytes(vec),
            Dop2Payloads::I16(payload)       => payload.to_bytes(vec),
            Dop2Payloads::I32(payload)       => payload.to_bytes(vec),
            Dop2Payloads::I64(payload)       => payload.to_bytes(vec),
            Dop2Payloads::E8(payload)        => payload.to_bytes(vec),
            Dop2Payloads::E16(payload)       => payload.to_bytes(vec),
            Dop2Payloads::E32(payload)       => payload.to_bytes(vec),
            Dop2Payloads::E64(payload)       => payload.to_bytes(vec),
            Dop2Payloads::MString(payload)   => payload.to_bytes(vec),
            Dop2Payloads::ArrayU8(payload)  => payload.to_bytes(vec),
            Dop2Payloads::ArrayU16(payload)  => payload.to_bytes(vec),
            Dop2Payloads::ArrayI8(payload)  => payload.to_bytes(vec),
            Dop2Payloads::ArrayI16(payload)  => payload.to_bytes(vec),
            Dop2Payloads::ArrayI32(payload)  => payload.to_bytes(vec),
            Dop2Payloads::ArrayE8(payload)   => payload.to_bytes(vec),
            Dop2Payloads::ArrayE16(payload)  => payload.to_bytes(vec),
            Dop2Payloads::ArrayE32(payload)  => payload.to_bytes(vec),
            Dop2Payloads::ArrayE64(payload)  => payload.to_bytes(vec),
            Dop2Payloads::ArrayU32(payload)  => payload.to_bytes(vec),
            Dop2Payloads::ArrayU64(payload)  => payload.to_bytes(vec),
            Dop2Payloads::MStruct(payload)   => payload.to_bytes(vec),
            Dop2Payloads::AStruct(payload)   => payload.to_bytes(vec),
            Dop2Payloads::Trash => todo!(),
            Dop2Payloads::ArrayBool(_) => todo!(),
            Dop2Payloads::ArrayI64(_) => todo!(),
            Dop2Payloads::ArrayF32(_) => todo!(),
            Dop2Payloads::ArrayF64(_) => todo!(),
            Dop2Payloads::F32(_) => todo!(),
            Dop2Payloads::F64(_) => todo!(),
        }
        //self.value.to_bytes();
    }
    fn parse(parser: &mut Dop2Parser) -> Result<TaggedDopField, String> {
        let field_index = parser.take_u16().unwrap();
        let tag_byte = parser.take_u8()?;
        let tag = Dop2PayloadsKind::try_from_primitive(tag_byte)
            .map_err(|_| format!("Invalid Dop2Type value: 0x{:02X}", tag_byte))?;
        let value = match tag {
            Dop2PayloadsKind::Boolean => Dop2Payloads::Boolean(*bool::parse(parser).unwrap()),
            Dop2PayloadsKind::E8        => Dop2Payloads::E8(*E8::parse(parser).unwrap()),
            Dop2PayloadsKind::U8        => Dop2Payloads::U8(*u8::parse(parser).unwrap()),
            Dop2PayloadsKind::U16       => Dop2Payloads::U16(*u16::parse(parser).unwrap()),
            Dop2PayloadsKind::U64       => Dop2Payloads::U64(*u64::parse(parser).unwrap()),
            Dop2PayloadsKind::I8       => Dop2Payloads::I8(*i8::parse(parser).unwrap()),
            Dop2PayloadsKind::I16       => Dop2Payloads::I16(*i16::parse(parser).unwrap()),
            Dop2PayloadsKind::E16       => Dop2Payloads::E16(*E16::parse(parser).unwrap()),
            Dop2PayloadsKind::U32       => Dop2Payloads::U32(*u32::parse(parser).unwrap()),
            Dop2PayloadsKind::I32       => Dop2Payloads::I32(*i32::parse(parser).unwrap()),
            Dop2PayloadsKind::E32       => Dop2Payloads::E32(*E32::parse(parser).unwrap()),
            Dop2PayloadsKind::I64       => Dop2Payloads::I64(*i64::parse(parser).unwrap()),
            Dop2PayloadsKind::E64       => Dop2Payloads::E64(*E64::parse(parser).unwrap()),
            Dop2PayloadsKind::MString   => Dop2Payloads::MString(*String::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayU8  => Dop2Payloads::ArrayU8(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayI8  => Dop2Payloads::ArrayI8(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayI16  => Dop2Payloads::ArrayI16(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayI32  => Dop2Payloads::ArrayI32(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayU32  => Dop2Payloads::ArrayU32(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayU16   => Dop2Payloads::ArrayU16(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayE8   => Dop2Payloads::ArrayE8(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayE16  => Dop2Payloads::ArrayE16(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayE32  => Dop2Payloads::ArrayE32(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayE64  => Dop2Payloads::ArrayE64(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayU64  => Dop2Payloads::ArrayU64(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::MStruct    => Dop2Payloads::MStruct(*Dop2Struct::parse(parser).unwrap()),
            Dop2PayloadsKind::AStruct   => Dop2Payloads::AStruct(*DopArray::parse(parser).unwrap()),

            garbage => 
            {
                println!("unknown type {:?}", garbage);
                todo!()
            }
        };
        
        Ok(TaggedDopField { tag, field_index, value })
    }
}


#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dop2Struct{
    declared_fields: u16,
    fields: Vec<TaggedDopField>,
}
impl Dop2Struct {
    fn get_field (&self, id: u16)->Option<TaggedDopField>
    {
        self.fields.iter().find(|x| x.field_index==id).cloned() // TODO: check for duplicate indices and remove the clone
    }
    fn get_payload (&self, id: u16)->Option<Dop2Payloads>
    {
        self.get_field(id).map(|x| x.value.clone())
    }
    fn from_fields (fields: Vec<TaggedDopField>) -> Self
    {
        let m = fields.iter();
        let index = m.max_by_key(|x|x.field_index).map(|x|x.field_index).unwrap_or(0);

        Dop2Struct {declared_fields: index, fields}
    }
}
impl Dop2Struct 
{
    fn get_length (&self) -> u16
    {
        //2 + // 2-byte field_count header
        2+ self.fields.iter().map(|x|x.get_length()).sum::<u16>()
    }
}
impl ToDop2Bytes for Dop2Struct
{
    fn to_bytes(self, vec: &mut Vec<u8>)
    {
        let field_count : u16 = self.fields.len().try_into().unwrap();
        vec.extend(field_count.to_be_bytes());
        self.fields.into_iter().for_each(|field| field.to_bytes(vec));
    }
}
impl Dop2PayloadExpressible for Dop2Struct
{
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> {
        let declared_fields = parser.take_u16()?;
        let mut fields = Vec::new();
        //println!("Parsing fields");
        for _x in 1..declared_fields+1 {
           // println!("Parsing field {} of {}", x, declared_fields);
            let tagged_field = TaggedDopField::parse(parser)?;
            fields.push(tagged_field);
        }
        Ok(Box::new(Dop2Struct {declared_fields, fields}))
    }
}

impl RootNode {
    fn parse(parser: &mut Dop2Parser) -> Result<RootNode, String> {
        let declared_length = parser.take_u16().unwrap(); // only for validation, not needed for further parsing

        let unit = parser.take_u16()?;
        let attribute = parser.take_u16()?; 
        
        let idx1 = parser.take_u16().unwrap();
        let idx2 = parser.take_u16().unwrap();

        let root_struct = *Dop2Struct::parse(parser).unwrap();

        let _padding = DopPadding::parse(parser).unwrap();
        assert!(parser.is_empty()); // no trailing garbage

        /*if (declared_length != root_struct.get_length())// declared length equal to amounts of bytes parsed
{
    println!("{} is declared length, but calculated length is {}", declared_length, root_struct.get_length());
    return Err("size mismatch".to_string());
}*/


        Ok(RootNode { unit, attribute, declared_length, idx1, idx2, root_struct })
    }

    fn to_bytes (self, builder: &mut Vec<u8>)

    {
        builder.extend(self.unit.to_be_bytes());
        builder.extend(self.attribute.to_be_bytes());

        builder.extend(self.idx1.to_be_bytes());
        builder.extend(self.idx2.to_be_bytes());

        self.root_struct.to_bytes(builder);
        let length : u16 = builder.len().try_into().unwrap();
        builder.splice(0..0, (length).to_be_bytes()); // TODO: Fix this

        let padding = DopPadding::minimum_padding(builder);
       // println!("{:?} bytes of padding",padding.bytes_of_padding);
        padding.to_bytes(builder);
    }
}


struct Dop2Parser {
    payload : Vec<u8>
}
impl Dop2Parser {
    fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
    fn take(&mut self, n: usize) -> Result<Vec<u8>, &'static str> {
        if self.payload.len() < n {
            return Err("Not enough bytes in payload");
        }
        let bytes = self.payload.drain(..n).collect::<Vec<u8>>();
        //println!("{:#?}", bytes);
        Ok(bytes)
    }
    fn take_u16(&mut self) -> Result<u16, &'static str> {
        let bytes = self.take(2)?;
        Ok(((bytes[0] as u16) << 8) | bytes[1] as u16)
    }
    fn take_u8(&mut self) -> Result<u8, &'static str> {
        let bytes = self.take(1)?;
        Ok(bytes[0])
    }

    fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// The hex string to parse
    hex_string: Option<String>,
    
    // Unit parameter (optional)
   // #[arg(short, long)]
    //unit: Option<u16>,
    
    // Attribute parameter (optional)
   // #[arg(short, long)]
   // attribute: Option<u16>,
}

struct ParseHex 
{
    hex_string: String,
}

mod payloader;
#[macro_use]
pub mod macros;


use crate::payloader::comm_module::request::request::{XkmRequestId, XkmRequest};
use strum::IntoEnumIterator;
use std::str::FromStr;

fn main() {
    let args = Args::parse();

    let command_verbs_xkm = payloader::comm_module::request::request::XkmRequestId::iter().map(|x| x.to_string());
    let command_verbs_program = ProgramIdOven::iter().map(|x| x.to_string());
   // let command_verbs_user_request = UserRequestOven::iter().map(|x| x.to_string());
    //let mut it : Vec<String> = command_verbs_xkm.chain(command_verbs_program)
    //.chain(command_verbs_user_request)
  //  .collect();
   // let sorted = it.sort();
    
   let command = args.hex_string.as_deref().unwrap_or("");


    if let Ok(xkm)=XkmRequestId::from_str(&command)
    {

        eprintln!("Sending XKM command {:?}", xkm);
        let request = XkmRequest{request_id: xkm};
        let payload = request.to_dop2_struct_auto().unwrap();

        let root = RootNode::single(UnitIds::CommunicationsModule.into(), XkmRequest::ATTRIBUTE_IDS.first().unwrap().clone(), payload);
       
        let mut data : Vec<u8> = vec!();
        root.to_bytes(&mut data);
       // payload.to_bytes(&mut data);
        let hexdump = hex::encode(data);
        println!("{}", hexdump);
        
    }
    else if let Ok(program_id)=ProgramIdOven::from_str(&command)
    {
        eprintln!("Sending PS command {:?}", program_id);
        let request : payloader::device::generic::program_selection::select::PsSelect = payloader::device::generic::program_selection::select::PsSelect { program_id, selection_parameter: 0, selection_type: SelectionType::InitialDefault };
        let payload = request.to_dop2_struct_auto().unwrap();

        let root = RootNode::single(UnitIds::MainDevice.into(), payloader::device::generic::program_selection::select::PsSelect::ATTRIBUTE_IDS.first().unwrap().clone(), payload);
       
        let mut data : Vec<u8> = vec!();
        root.to_bytes(&mut data);
       // payload.to_bytes(&mut data);
        let hexdump = hex::encode(data);
        println!("{}", hexdump);

    }
    else if let Ok(user_request_id)=UserRequestOven::from_str(&command)
    {
        eprintln!("Sending UserRequest command {:?}", user_request_id);
        let request = payloader::device::generic::request::UserRequest {request_id: user_request_id};
    }
    else {
        let hex_str = match &args.hex_string {
            Some(s) => s,
            None => {
                println!("Available commands are:\n");
                println!("*** Program Selection: {:?}\n", command_verbs_program.collect::<Vec<_>>());
                println!("*** Communications Module: {:?}\n", command_verbs_xkm.collect::<Vec<_>>());
                eprintln!("Error: no hex string provided");
                std::process::exit(1);
            }
        };

    let bytes = match hex::decode(hex_str) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("Available commands are:");
            println!("*** Program Selection: {:?}\n", command_verbs_program.collect::<Vec<String>>());
            println!("*** Communications Module: {:?}\n", command_verbs_xkm.collect::<Vec<String>>());
            eprintln!("Error decoding hex string: {}", e);
            std::process::exit(1);
        }
    };
    let mut parser = Dop2Parser::new(bytes);
    let root_node = RootNode::parse(&mut parser).unwrap();
    println!("{root_node:#?}");
    

/*

     */
    if (payloader::device::generic::context::DeviceContext::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::device::generic::context::DeviceContext::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloader::device::oven::program_info::ProgramInfoOven::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::device::oven::program_info::ProgramInfoOven::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloader::device::oven::program_step_info::ProgramStepInfoOven::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::device::oven::program_step_info::ProgramStepInfoOven::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloader::device::generic::ident::ident::DeviceIdent::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::device::generic::ident::ident::DeviceIdent::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
    else if (payloader::comm_module::state::datetime::DateTimeInfo::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::comm_module::state::datetime::DateTimeInfo::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloader::filesystem::file_list::FileList::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::filesystem::file_list::FileList::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloader::filesystem::file_info::FileInfo::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::filesystem::file_info::FileInfo::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if payloader::filesystem::transfer::FileTransfer::ATTRIBUTE_IDS.contains(&root_node.attribute)
{
    let decoded = payloader::filesystem::transfer::FileTransfer::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloader::filesystem::rsa_key::RsaKey::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::filesystem::rsa_key::RsaKey::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}

    else if (payloader::device::generic::failure::FailureList::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::device::generic::failure::FailureList::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloader::device::generic::request::UserRequest::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::device::generic::request::UserRequest::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (XkmRequest::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = XkmRequest::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloader::comm_module::config::ip::XkmConfigIp::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::comm_module::config::ip::XkmConfigIp::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloader::comm_module::config::ssid::XkmConfigSsidList::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloader::comm_module::config::ssid::XkmConfigSsidList::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (DeviceCombiState::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = DeviceCombiState::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
    }
    else if (payloader::device::generic::settings::SfValueList::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::generic::settings::SfValueList::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloader::device::generic::program_selection::context::PSContext::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::generic::program_selection::context::PSContext::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloader::device::generic::state::cs_context::CSContext::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::generic::state::cs_context::CSContext::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }

    else if (payloader::device::generic::failure::Failure::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::generic::failure::Failure::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }

    else if (payloader::device::generic::state::hours::CSHoursOfOperation::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::generic::state::hours::CSHoursOfOperation::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloader::device::generic::ident::feature_list::FeatureList::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::generic::ident::feature_list::FeatureList::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloader::device::generic::notifications::DeviceNotifications::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::generic::notifications::DeviceNotifications::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloader::device::washer::process::Process::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::washer::process::Process::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloader::device::washer::actuator::ActuatorData::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::washer::actuator::ActuatorData::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloader::device::washer::sensor::Sensor::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloader::device::washer::sensor::Sensor::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }

    }       
    
}

mod payloads {
    use super::*;
#[repr(u16)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, EnumIter, EnumString, strum_macros::Display, IntoPrimitive)]
pub enum UnitIds {
    ProgrammingMaster = 1, // appears to be the unit responsible for front panel, notifications, programming
    MainDevice = 2,
    UnknownThree = 3, // seen in oven
   // SecondDevice = 5, // seen in oven code but never in real device
    UnknownEight = 8, // seen in oven
    UnknownNine = 9, // seen in oven
    UnknownTwelve = 12, // seen in oven
    CommunicationsModule = 14,
    Filesystem = 15,
}









pub trait Dop2ParseTreeExpressible : Sized
{
    fn from_parse_tree(payload: Dop2Payloads) -> Result<Self, String>;
}

/*
impl TryFrom<Dop2Struct> for DeviceCombiState 
{
    type Error = String;

    fn try_from(value: Dop2Struct) -> Result<Self, Self::Error> {
        return DeviceCombiState::from_parse_tree (Dop2Payloads::MStruct(value))
    }
}

impl TryFrom<Dop2Struct> for PSAttributesCCA 
{
    type Error = String;

    fn try_from(value: Dop2Struct) -> Result<Self, Self::Error> {
        return PSAttributesCCA::from_parse_tree (Dop2Payloads::MStruct(value))
    }
}*/



//impl_tryfrom_dop2struct!(UserRequestOven);
//impl_tryfrom_dop2struct!(ApplianceState);
//impl_tryfrom_dop2struct!(SelectionType);


impl_tryfrom_dop2struct!(AnnotatedBool);
impl_tryfrom_dop2struct!(AnnotatedU8);
impl_tryfrom_dop2struct!(AnnotatedI16);
impl_tryfrom_dop2struct!(AnnotatedI32);


impl_tryfrom_dop2struct!(GenericU8);

impl_tryfrom_dop2struct!(AnnotatedU16);
impl_tryfrom_dop2struct!(GenericU16);
//impl_tryfrom_dop2struct!(AnnotatedU32);
impl_tryfrom_dop2struct!(AnnotatedU64);
impl_tryfrom_dop2struct!(AnnotatedTimeStamp);






}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    // Static test data structure with individual fields
    struct TestPayloads {
        oven_14_130: &'static str,
        oven_2_1586: &'static str,
        oven_9_19: &'static str,
        oven_ident: &'static str,
        oven_2_114: &'static str,
        oven_1_391: &'static str,
        oven_1_209: &'static str,
        oven_2_1585: &'static str,
        oven_1_1599: &'static str,
    }
    
    static TEST_PAYLOADS: TestPayloads = TestPayloads {
        // actual oven payloads
        oven_14_130:"000e000e008200010001000100010400", // one E8, no padding
        oven_2_1586: "0016000206320000000000030001040400020405000304012020202020202020", // devicecombistate, 3 E8, padding
        oven_9_19: "00230009001300000001000500010500ab0002050001000305fb00000405fb0000050500002020202020202020202020", // U16s with padding
    //001c000e007a00010001000200010b0000000068e814fd000209000000002020 //Unsigned64
        oven_ident: "004e000e061d0001000100080002040000030400000412000530392e31340005051a390006120008001d63fffeaf152f0007040000081200080000000000000000000914000a00000000000000000000",
        oven_2_114: "009f0002007200000000000200010226000217004603f903f303fa03ee03f1001e0021001b001c001d001a000a000b000c00150018001903e800030005000400160011000e0012042e042d04590464045a046b046904520453046504560457046c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000202020202020202020202020202020",
        oven_1_391: "02c1000101870000000000090001100003000104000002040000030400000710001a0001070000000207000000031000030001020000020500000003040000041000030001020000020500000003040000051000030001020000020500000003040000061000030001020000020500000003040000071000030001020000020500000003040000081000030001020000020200000304000009100003000102000002020000030400000a0400000b0200000c100003000102000002020000030400000d100003000102000002020000030400000e10000300010200000202000003040000100400001210000300010200000205000000030400001308000000000014070000001510000300010200000205000000030400001610000300010200000202000003040000171000030001020000020500000003040000181000030001020000020200000304000019100003000102000002020000030400001a1000030001020000020b000000000000000000030400001c070000001d1000030001020000020b000000000000000000030400000810001e000208000000000003080000000000040800000000000508000000000006080000000000070400000804000009100003000104000002040000030400000b0400000c050000000d0800000000000e0800000000000f0200001004000011040000120100001301000014010000150100001608000000000017050000001805000000190200001a0100001b0100001c0100001d050000001e050000001f100003000101000002010000030100002010000400010100000201000003010000040100000917000e00000000000000000000000000000000000000000000000000000000000a0400000b0100000c070000000d0100001110000700010100000208000000000003080000000000040800000000000512000c00000000000000000000000000060500000007080000000020202020202020202020202020", // struct with U8s
        oven_1_209: "0050000100d1000000000002000121000200020001010000020b000000000000000000020001010000020b000000000000000000022100020002000104000002090000015c000200010401000209000000002020202020202020202020202020", // Struct[]
        oven_2_1585: "02c1000106310000000000090001100003000104040002040600030404000710001a00010727100002070000000310000300010200000205ffff00030400000410000300010208000205000000030405000510000300010200000205ffff00030405000610000300010208000205086300030403000710000300010200000205ffff0003040300081000030001020800020201000304000009100003000102000002020000030400000a0432000b0200000c100003000102000002020000030400000d100003000102000002020000030400000e100003000102000002020000030400001004000012100003000102000002050000000304000013080000000000140700000015100003000102080002053e8000030403001610000300010200000202000003040100171000030001020000020500000003040300181000030001020000020200000304000019100003000102000002020000030407001a1000030001020800020b0000000068e814f40003041f001c07000d001d1000030001020000020b000000000000000000030400000810001e000208000000000003080000000000040800000000000508000000000006080000000000070400000804000009100003000104000002040000030400000b0400000c05ffff000d0800000000000e0800000000000f02ff001004000011040000120100001301000014010000150100001608000001630017050000001805000000190200001a0100001b0100001c0100001d050000001e050000001f100003000101000002010000030100002010000400010100000201000003010000040100000917000e0013000e000200380070000000000000000000000000000000000000000a0401000b0101000c070000000d0100001110000700010100000208ffffffff00030800000000000408ffffffff000512000c0000000000000000000000000006050000000708ffffffff20202020202020202020202020",
        oven_1_1599: "026c0001063f0000000000020001210008000400010500010002050060000308000000010004180004518036610007800000000000000000000004000105006100020500960003080000000200041800040000000000000000000000000000000000040001053a980002053ab7000308000001000004180004000000010000000000000000000000000004000105010300020501420003080000001000041800045e40d8110000000000000000000000000004000105014300020501620003080000002000041800040000301f00000000000000000000000000040001053e800002053edf0003080000020000041800042bdfdf0e0000003f0000000000000000000400010500000002050000000308000000000004180004000000000000000000000000000000000004000105000000020500000003080000000000041800040000000000000000000000000000000000022100010004000105016300020507e0000308000000400004180040f99c76ae11f8c681c103071b0001643cefffe308513ffcbb000007fe0000000003807c00000000000ff0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002020" // programGroupsComplete
       };
     static TEST_BANK : [&str; 7] = [TEST_PAYLOADS.oven_14_130, TEST_PAYLOADS.oven_2_1586, TEST_PAYLOADS.oven_9_19, TEST_PAYLOADS.oven_ident, TEST_PAYLOADS.oven_2_114, TEST_PAYLOADS.oven_1_391, TEST_PAYLOADS.oven_1_209];
 

    #[test]
    fn test_root_node_parse_insufficient_data() {
        // Test with insufficient data (only 2 bytes, need at least 4)
        let test_data = vec![0x12, 0x34];
        
        let mut parser = Dop2Parser::new(test_data);
        let result = RootNode::parse(&mut parser);
        assert!(result.is_err());
    }

    #[test]
    fn round_trips ()
    {
        let test_bank = [TEST_PAYLOADS.oven_1_209];
        for x in test_bank
        {
            test_round_trip(x);
        }
    }

    #[test]
    fn test_device_combo_state() {
        // Test that we can use it with our parser
        let mut parser = Dop2Parser::new(hex::decode(TEST_PAYLOADS.oven_2_1586).unwrap());
        let result = RootNode::parse(&mut parser);
        assert!(result.is_ok());
        let _root_node = result.unwrap();
        //assert_eq!(root_node.unit, 2);
        //assert_eq!(root_node.attribute, 1586);
       // assert_eq!(root_node.declared_fields, 3);
      //  assert_eq!(root_node.fields[0].value, 0x0082);
    }

    fn test_round_trip (payload: &str)
    {
        let mut parser = Dop2Parser::new(hex::decode(payload).unwrap());
        let result = RootNode::parse(&mut parser);
        assert!(result.is_ok());
        let root_node = result.unwrap();
        let mut data :  Vec<u8> = Vec::new();
        let _bytes = root_node.to_bytes(&mut data);
        let hex_string = hex::encode(&data);
        assert_eq!(hex_string, payload);
    }
    #[test]
    fn test_static_payloads() {
        // Test that we can use it with our parser
        let mut parser = Dop2Parser::new(hex::decode(TEST_PAYLOADS.oven_14_130).unwrap());
        let result = RootNode::parse(&mut parser);
        assert!(result.is_ok());

        let root_node = result.unwrap();
        let mut data :  Vec<u8> = Vec::new();
        //root_node.padding=None;
        let _bytes = root_node.to_bytes(&mut data);
        let hex_string = hex::encode(&data);
        assert_eq!(hex_string, TEST_PAYLOADS.oven_14_130);
        //assert_eq!(root_node.unit, 14);
        //assert_eq!(root_node.attribute, 130);
      //  assert_eq!(root_node.declared_fields, 1);
      //  assert_eq!(root_node.fields[0].value, 0x0082);
    }

    
}
