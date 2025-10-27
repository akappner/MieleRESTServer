#![feature(derive_from)]

use clap::Parser;
use num_enum::TryFromPrimitive;
use derive_more::From;

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
#[derive(Clone, Debug)]
struct TaggedDopField {
    tag: Dop2Type,
    field_index : u16,
    value: Dop2Payloads,
}


#[allow(dead_code)]
#[derive(Clone, Debug)]
struct DopArray <T : Dop2PayloadExpressible+ToDop2Bytes>
{
    count : u16,
    elements : Vec<T>
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
        struct $name($inner);

//        impl From<$inner> for $name {
 //           fn from(value: $inner) -> Self { $name(value) }
  //      }

 //       impl From<$name> for $inner {
  //          fn from(value: $name) -> Self { value.0 }
   //     }
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

#[derive(Clone, Debug)]
enum Dop2Payloads {
    Boolean(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    E8(E8),
    E16(E16),
    E32(E32),
    E64(E64),

    MString(String),
    //StringU8 (String),

    ArrayU8 (DopArray<u8>),
    ArrayU16 (DopArray<u16>),
    ArrayU32 (DopArray<u32>),
    ArrayU64 (DopArray<u64>),

    ArrayI8 (DopArray<i8>),
    ArrayI16 (DopArray<i16>),
    ArrayI32 (DopArray<i32>),
//    ArrayI64 (DopArray<u64>),

    ArrayE8 (DopArray<u8>),
    ArrayE16 (DopArray<u16>),
    ArrayE32 (DopArray<u32>),
    ArrayE64(DopArray<u64>),

    MStruct (Dop2Struct),
    AStruct (DopArray<Dop2Struct>)
}


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
        println!("{}", current);
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

macro_rules! impl_from_bytes {
    ($t:ty) => {
        impl Dop2PayloadExpressible for $t {
            fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> {
                let n = std::mem::size_of::<$t>();
                let bytes = parser.take(n);
                let mut value: $t = 0;
                for (i, b) in bytes.unwrap().into_iter().enumerate()
                {
                    //println!("{}", b);
                    value |= ((b as $t) << ((n - 1 - i) * 8));
                }

                Ok(Box::new(value as $t))
            }
        }
    };
}

impl_from_bytes!(u8);
impl_from_bytes!(u16);
impl_from_bytes!(u32);
impl_from_bytes!(u64);

impl_from_bytes!(i8);
impl_from_bytes!(i16);
impl_from_bytes!(i32);
impl_from_bytes!(i64);

macro_rules! impl_to_bytes {
    ($t:ty) => {
        impl ToDop2Bytes for $t {
            fn to_bytes(self, vec: &mut Vec<u8>) {
                vec.extend(self.to_be_bytes());
            }
        }
    };
}

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
        println!("Pushed tag {:#x}",self.tag as u8);
//        let gag : Box<dyn ToDop2Bytes> = Box::new (self.value);
        
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
         //   _ => todo!("not yet implemented")
        }
        //self.value.to_bytes();
    }
    fn parse(parser: &mut Dop2Parser) -> Result<TaggedDopField, String> {
        let field_index = parser.take_u16().unwrap();
        let tag_byte = parser.take_u8()?;
        let tag = Dop2Type::try_from_primitive(tag_byte)
            .map_err(|_| format!("Invalid Dop2Type value: 0x{:02X}", tag_byte))?;
        let value = match tag {
            Dop2Type::Bool => Dop2Payloads::Boolean(*bool::parse(parser).unwrap()),
            Dop2Type::E8        => Dop2Payloads::E8(*E8::parse(parser).unwrap()),
            Dop2Type::U8        => Dop2Payloads::U8(*u8::parse(parser).unwrap()),
            Dop2Type::U16       => Dop2Payloads::U16(*u16::parse(parser).unwrap()),
            Dop2Type::U64       => Dop2Payloads::U64(*u64::parse(parser).unwrap()),
            Dop2Type::I8       => Dop2Payloads::I8(*i8::parse(parser).unwrap()),
            Dop2Type::I16       => Dop2Payloads::I16(*i16::parse(parser).unwrap()),
            Dop2Type::E16       => Dop2Payloads::E16(*E16::parse(parser).unwrap()),
            Dop2Type::U32       => Dop2Payloads::U32(*u32::parse(parser).unwrap()),
            Dop2Type::I32       => Dop2Payloads::I32(*i32::parse(parser).unwrap()),
            Dop2Type::E32       => Dop2Payloads::E32(*E32::parse(parser).unwrap()),
            Dop2Type::I64       => Dop2Payloads::I64(*i64::parse(parser).unwrap()),
            Dop2Type::E64       => Dop2Payloads::E64(*E64::parse(parser).unwrap()),
            Dop2Type::MString   => Dop2Payloads::MString(*String::parse(parser).unwrap()),
            Dop2Type::ArrayU8  => Dop2Payloads::ArrayU8(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayI8  => Dop2Payloads::ArrayI8(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayI16  => Dop2Payloads::ArrayI16(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayI32  => Dop2Payloads::ArrayI32(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayU32  => Dop2Payloads::ArrayU32(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayU16   => Dop2Payloads::ArrayU16(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayE8   => Dop2Payloads::ArrayE8(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayE16  => Dop2Payloads::ArrayE16(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayE32  => Dop2Payloads::ArrayE32(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayE64  => Dop2Payloads::ArrayE64(*DopArray::parse(parser).unwrap()),
            Dop2Type::ArrayU64  => Dop2Payloads::ArrayU64(*DopArray::parse(parser).unwrap()),
            Dop2Type::Struct    => Dop2Payloads::MStruct(*Dop2Struct::parse(parser).unwrap()),
            Dop2Type::AStruct   => Dop2Payloads::AStruct(*DopArray::parse(parser).unwrap()),

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
#[derive(Clone, Debug)]
struct Dop2Struct{
    declared_fields: u16,
    fields: Vec<TaggedDopField>,
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
        assert!(parser.is_empty());

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
        builder.splice(0..0, length.to_be_bytes());

        let padding = DopPadding::minimum_padding(builder);
        println!("{:?} bytes",padding.bytes_of_padding);
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
    hex_string: String,
    
    // Unit parameter (optional)
   // #[arg(short, long)]
    //unit: Option<u16>,
    
    // Attribute parameter (optional)
   // #[arg(short, long)]
   // attribute: Option<u16>,
}

fn main() {
    let args = Args::parse();
    
    let bytes = match hex::decode(&args.hex_string) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error decoding hex string: {}", e);
            std::process::exit(1);
        }
    };
    let mut parser = Dop2Parser::new(bytes);
    let root_node = RootNode::parse(&mut parser).unwrap();
    println!("{root_node:#?}");

    match root_node.attribute
    {
        payloads::XkmRequest::ATTRIBUTE => 
        { 
             let decoded = payloads::XkmRequest::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
             println!("{decoded:#?}");
         },

        payloads::PsSelect::ATTRIBUTE => 
        { 
             let decoded = payloads::PsSelect::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
             println!("{decoded:#?}");
         },
        payloads::DeviceCombiState::ATTRIBUTE => 
        { 
             let decoded = payloads::DeviceCombiState::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
             println!("{decoded:#?}");
         }

        _ => { println!("no decoding for attribute"); }
    }
}

mod payloads {
    use super::*;
#[repr(u8)]
enum UnitIds {
    UnknownOne = 1,
    MainDevice = 2,
    UnknownThree = 3, // seen in oven
    UnknownEight = 8, // seen in oven
    UnknownNine = 9, // seen in oven
    UnknownTwelve = 12, // seen in oven
    CommunicationsModule = 14,
    Filesystem = 15,
}

    #[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
enum UserRequestOven {
    Nop = 0,
    Start = 1,
    Stop = 2,
    Pause = 3,
    StartDelay = 8,
    DoorOpen = 11,
    DoorClose = 12,
    LightOn = 13,
    LightOff = 14,
    FactorySettingReset = 15,
    SwitchOn = 16,
    Next = 17,
    Back = 18,
    SwitchOff = 19,
    ResetPinCode = 20,
    KeepAlive = 21,
    Step = 22,
    StartRemoteUpdateInstall = 23,
    ProgramStop = 54,
    ProgramAbort = 55,
    ProgramFinalize = 56,
    ProgramSave = 61,
    MotorizedFrontPanelOpen = 65,
    MotorizedFrontPanelClose = 66,
    HoldingBreak = 68,
    HoldingStart = 69,
}

#[repr(u16)]
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq, From)]
enum ProgramIdOven {
    NoProgram = 0,
    OvenHotAirPlus = 13,
    OvenIntenseBaking = 14,
}

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq)]
enum XkmRequestId {
    None = 0,
    Reset = 1,
    FactoryReset = 2,
    OpenSoftAccessPointEndUser = 3,
    OpenSoftAccessPointCustomerService = 45,
    Shutdown = 46,
    MieleSmartConnect = 47,
}

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq)]
enum ApplianceState
{
    Unknown,
    Off,
    Synchronizing,
    Initializing,
    Normal,
    Demonstration,
    Service,
    Error,
    CheckAppliance,
    Standby,
    Supervisory,
    ShowWindow
}
#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq)]
enum OperationState
{
    Unknown,
    EndOfLine,
    Service,
    Settings,
    InitialSettings,
    SelectProgram,
    RunDelay,
}
#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq)]
enum ProcessState
{
    Unknown,
    NoProgram,
    ProgramSelected,
    ProgramStart,
    ProgramRunning
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceCombiState {
        appliance_state : ApplianceState,
        operation_state : OperationState,
        process_state : ProcessState
}


impl DeviceCombiState
{
        pub const ATTRIBUTE : u16 = 1586;

        pub fn from_parse_tree (payload: Dop2Payloads) -> Result<Self, String>
        {
            if let Dop2Payloads::MStruct(x)=payload // if payload cannot be deserialized as struct, fail
            {
                if let Dop2Payloads::E8(appliance_state) = x.fields[0].value &&
                   let Dop2Payloads::E8(operation_state) = x.fields[1].value &&
                  let Dop2Payloads::E8(process_state) = x.fields[2].value 
                {
                    return Ok(DeviceCombiState{appliance_state: appliance_state.0.try_into().unwrap(),
                             operation_state: operation_state.0.try_into().unwrap(), 
                             process_state: process_state.0.try_into().unwrap()} )
                }
                else 
                { 
                    println!("{:?}", x);
                    return Err("Entity mismatch while deserializing field".to_string())
                }
            }
            Err("Entity mismatch".to_string())
        }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XkmRequest {
        request_id : XkmRequestId
}
    impl XkmRequest
    {
        pub const ATTRIBUTE : u16 = 130; // typically unit 14
    
        pub fn from_parse_tree (payload: Dop2Payloads) -> Result<Self, String>
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

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PsSelect {
        program_id : ProgramIdOven
}
    impl PsSelect
    {
        pub const ATTRIBUTE : u16 = 1577;

        pub fn from_parse_tree (payload: Dop2Payloads) -> Result<Self, String>
        {
            if let Dop2Payloads::MStruct(x)=payload // if payload cannot be deserialized as struct, fail
            {
                if let Dop2Payloads::E16(program_id) = x.fields[0].value
                {
                    let intermediate : E16 = program_id.try_into().unwrap();
                    return Ok(PsSelect{program_id: intermediate.0.try_into().unwrap() })
                }
                else 
                {
                    return Err("Entity mismatch while deserializing field".to_string())
                }
            }
            Err("Entity mismatch".to_string())
        }
    }
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
        oven_1_209: "0050000100d1000000000002000121000200020001010000020b000000000000000000020001010000020b000000000000000000022100020002000104000002090000015c000200010401000209000000002020202020202020202020202020" // Struct[]

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
