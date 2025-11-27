#![feature(derive_from)]
use chrono::{DateTime, NaiveDateTime, Utc};

use clap::Parser;
use num_enum::{TryFromPrimitive, IntoPrimitive};
use derive_more::From;

use dop2marshal::AssocTypes;
use payloads::{UnitIds, XkmRequest};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dop2TimestampUtc(DateTime<Utc>);

impl TryFrom<u64> for Dop2TimestampUtc {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > i64::MAX as u64 {
            return Err("timestamp out of range");
        }

        let naive = NaiveDateTime::from_timestamp(value as i64, 0);
        Ok(Dop2TimestampUtc(DateTime::<Utc>::from_utc(naive, Utc)))
    }

    
}
impl From<Dop2TimestampUtc> for u64 
{
    fn from(value: Dop2TimestampUtc) -> Self {
        return value.0.timestamp() as u64;
    }
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
struct TaggedDopField {
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
struct DopArray <T : Dop2PayloadExpressible+ToDop2Bytes>
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
enum Dop2Payloads {
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

/*
macro_rules! MakeAnnotatedValueType {
($concrete_type) => {
paste! {
struct [<Garbage $concrete_type>] {

    #[dop2field(1, Dop2Payloads::U8)]
    requestMask : u8,
    #[dop2field(2, T)]
    value : $concrete_type, 
    #[dop2field(3, Dop2Payloads::E16)]
    interpretation: E16
}
}
}
}*/
/*
macro_rules! MakeAnnotatedValueType {
($payload:path, $concrete_type:ty) => {
paste! {
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
struct [<Garbage $concrete_type>] {
#[dop2field(1, Dop2Payloads::U8)]
requestMask : u8,

#[dop2field(2, $payload )]
value: $concrete_type,
#[dop2field(3, Dop2Payloads::E16)]
interpretation: E16

}
}
};
}
*/
/*
macro_rules! MakeAnnotatedValueType {
    ($name:ident, $payload:path, $concrete_type:ty) => {
//            paste!{
        #[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
        struct $name {
            #[dop2field(1, Dop2Payloads::U8)]
            requestMask: u8,
            #[dop2field(2, $payload )]
            value: $concrete_type,
            #[dop2field(3, Dop2Payloads::E16)]
            interpretation: E16,
//}
        }
    };
}

MakeAnnotatedValueType!(GarbageU16, Dop2Payloads::U16, u16);
*/

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, EnumIter, EnumString, strum_macros::Display, IntoPrimitive)]
pub enum ValueInterpretation {
    None = 0,
    Percentage = 1,
    TemperatureC1 = 2,
    TemperatureC100 = 3,
    TemperatureF100 = 4,
    DurationSec = 5,
    DurationMin = 6,
    Step = 7,
    WeightGram = 8,
    Numerical = 9,
    Date = 10,
    MicrowavePowerSteps = 11,
    RfPower = 12,
    RfEnergy = 13,
    RfMode = 14,
    Browning = 15,
    DegreeOfCooking = 16,
    TimeFormat = 17,
    TimePresentation = 18,
    Language = 19,
    BurstOfSteam = 20,
    DisplayScheme = 21,
    DisplayInStandby = 22,
    Lighting = 23,
    TemperatureUnit = 24,
    WeightUnit = 25,
    StartScreen = 26,
    WaterHardness = 27,
    TestState = 28,
    TimeUtc = 29,
    VoltageAndFrequency = 30,
    StartPoint = 31,
    EndPoint = 32,
    ParameterShape = 33,
    StartPointOrRightNow = 34,
    WaterLevelMmws = 35,
    WaterInletWay = 36,
    DrumSpeedRpm = 37,
    OnOff = 38,
    DoorSwitch = 39,
    DoorLockSwitch = 40,
    WpsSwitch = 41,
    TwindosContainer1Switch = 42,
    TwindosContainer2Switch = 43,
    EcoWaterLiter = 44,
    EcoEnergyKwh = 45,
    EcoEnergyWatt = 46,
    StartPointOnlyRightNow = 47,
    TemperatureF1 = 48,
    DrumSpeed10Rpm = 49,
    OperationMode = 50,
    Name = 51,
    TimeBackground = 52,
    DisplayBrightness = 53,
    DisplayContrast = 54,
    VolumeSignalTonesLevel = 55,
    VolumeKeyTone = 56,
    MotoePosition = 57,
    DoorExtOpeningEnabled = 58,
    WawPosition = 59,
    WawDirection = 60,
    CookingShelfs3 = 80,
    CookingShelfs4 = 81,
    CookingShelfs5 = 82,
    CookingShelfs6 = 83,
    PerformanceMode = 90,
    Altitude = 91,
    ProfileChange = 92,
    MicrowavePower = 96,
    Variant = 97,
    SensorGroup = 98,
    TimerDayOfWeekAssignment = 99,
    TimeUtc0 = 100,
    ActiveUser = 101,
    TemperatureText = 102,
    WeightTenthOfGram = 103,
    TimeDisplay = 104,
    DeviceHeight = 105,
    DeviceWidth = 106,
    AutoDosCartridgeType = 107,
    RinseAidCapacityMl = 108,
    Knock2Open = 109,
    CountryVariant = 110,
    NetworkingCountry = 111,
    CountryLanguage = 112,
    Lbs = 120,
    EnergyWh = 121,
    CoolAirBlowersFollowUp = 122,
    CookingProgramId = 123,
    Extend = 124,
    ApproximationLightOption = 125,
    BurstsOfSteamType = 126,
    Lbs100 = 127,
    DurationSecOrUndefined = 128,
    WaterHardnessDh = 129,
    FlowMlMin = 130,
    TimeIn100Ms = 131,
    Concentration = 132,
    WaterSource = 133,
    TempCalibration = 134,
    FoodProbeSelection = 135,
    FoodProbeSerialNumber = 136,
    LiquidQuantityLiters = 137,
    LiquidQuantity100Milliliters = 138,
    LiquidQuantityMilliliters = 139,
    Day = 140,
    Program = 141,
    SpinSpeed10RpmText = 142,
    Percent10 = 143,
    Temperature10C = 144,
    LiquidQuantity500Milliliters = 145,
    Percent100 = 146,
    Temperature10F = 147,
    Quantity10Liters = 148,
    MicroSiemens10PerCm = 149,
    MicroSiemensPerCm = 150,
}

macro_rules! MakeAnnotatedValueType {
    ($name:ident, $variant:ident, $concrete_type:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
        struct $name {
            #[dop2field(1, Dop2Payloads::U8)]
            requestMask: u8,
            #[dop2field(2, Dop2Payloads::$variant)]
            value: $concrete_type,
            #[dop2field(3, Dop2Payloads::E8)]
            interpretation: ValueInterpretation,
        }
        //impl_tryfrom_dop2struct!($name); // TODO: Fix this
    };

}

macro_rules! MakeGenericValueType {
    ($name:ident, $variant:ident, $concrete_type:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
        struct $name {
            #[dop2field(1, Dop2Payloads::U8)]
            requestMask: u8,
            #[dop2field(2,  Dop2Payloads::$variant)]
            min: $concrete_type,
            #[dop2field(3,  Dop2Payloads::$variant)]
            max: $concrete_type,
            #[dop2field(4,  Dop2Payloads::$variant)]
            current: $concrete_type,
            #[dop2field(5,  Dop2Payloads::$variant)]
            stepSize: $concrete_type,
        }
        //impl_tryfrom_dop2struct!($name); // TODO: Fix this
    };

}

MakeAnnotatedValueType!(AnnotatedU8, U8, u8);
MakeAnnotatedValueType!(AnnotatedU16, U16, u16);
MakeAnnotatedValueType!(AnnotatedU64, U64, u64);
MakeAnnotatedValueType!(AnnotatedBool, Boolean, bool);
MakeAnnotatedValueType!(AnnotatedTimeStamp, U64, Dop2TimestampUtc);

MakeGenericValueType!(GenericU8, U8, u8);
MakeGenericValueType!(GenericU16, U16, u16);

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
            Dop2Payloads::ArrayBool(dop_array) => todo!(),
            Dop2Payloads::ArrayI64(dop_array) => todo!(),
            Dop2Payloads::ArrayF32(dop_array) => todo!(),
            Dop2Payloads::ArrayF64(dop_array) => todo!(),
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
struct Dop2Struct{
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




use crate::payloads::{XkmRequestId, UserRequestOven, ProgramIdOven, PsSelect};
use strum::IntoEnumIterator;
use std::str::FromStr;

fn main() {
    let args = Args::parse();

    let commandVerbsXkm = XkmRequestId::iter().map(|x| x.to_string());
    let commandVerbsProgram = ProgramIdOven::iter().map(|x| x.to_string());
   // let commandVerbsUserRequest = UserRequestOven::iter().map(|x| x.to_string());
    //let mut it : Vec<String> = commandVerbsXkm.chain(commandVerbsProgram)
    //.chain(commandVerbsUserRequest)
  //  .collect();
   // let sorted = it.sort();
    
   let command = args.hex_string.as_deref().unwrap_or("");


    if let Ok(xkm)=XkmRequestId::from_str(&command)
    {

        eprintln!("Sending XKM command {:?}", xkm);
        let request : XkmRequest = payloads::XkmRequest{request_id: xkm};
        let payload = request.to_dop2_struct().unwrap();

        let root = RootNode::single(UnitIds::CommunicationsModule.into(), XkmRequest::ATTRIBUTE_IDS.first().unwrap().clone(), payload);
       
        let mut data : Vec<u8> = vec!();
        root.to_bytes(&mut data);
       // payload.to_bytes(&mut data);
        let hexdump = hex::encode(data);
        println!("{}", hexdump);
        
    }
    else if let Ok(programId)=ProgramIdOven::from_str(&command)
    {
        eprintln!("Sending PS command {:?}", programId);
        let request : payloads::PsSelect = payloads::PsSelect { program_id: programId, selection_parameter: 0, selection_type: payloads::SelectionType::InitialDefault };
        let payload = request.to_dop2_struct_auto().unwrap();

        let root = RootNode::single(UnitIds::MainDevice.into(), PsSelect::ATTRIBUTE_IDS.first().unwrap().clone(), payload);
       
        let mut data : Vec<u8> = vec!();
        root.to_bytes(&mut data);
       // payload.to_bytes(&mut data);
        let hexdump = hex::encode(data);
        println!("{}", hexdump);

    }
    else if let Ok(userRequestId)=UserRequestOven::from_str(&command)
    {
        eprintln!("Sending UserRequest command {:?}", userRequestId);
        let request = payloads::UserRequest {request_id: userRequestId};
    }
    else {
        let hex_str = match &args.hex_string {
            Some(s) => s,
            None => {
                println!("Available commands are:\n");
                println!("*** Program Selection: {:?}\n", commandVerbsProgram.collect::<Vec<_>>());
                println!("*** Communications Module: {:?}\n", commandVerbsXkm.collect::<Vec<_>>());
                eprintln!("Error: no hex string provided");
                std::process::exit(1);
            }
        };

    let bytes = match hex::decode(hex_str) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("Available commands are:");
            println!("*** Program Selection: {:?}\n", commandVerbsProgram.collect::<Vec<String>>());
            println!("*** Communications Module: {:?}\n", commandVerbsXkm.collect::<Vec<String>>());
            eprintln!("Error decoding hex string: {}", e);
            std::process::exit(1);
        }
    };
    let mut parser = Dop2Parser::new(bytes);
    let root_node = RootNode::parse(&mut parser).unwrap();
    println!("{root_node:#?}");
    

/*
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
        },
        _ => { println!("no decoding for attribute");
    }
}
     */
    if (payloads::DeviceContext::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::DeviceContext::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloads::ProgramInfoOven::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::ProgramInfoOven::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloads::ProgramStepInfoOven::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::ProgramStepInfoOven::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloads::DeviceIdent::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::DeviceIdent::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
    else if (payloads::DateTimeInfo::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::DateTimeInfo::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloads::FileList::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::FileList::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloads::FileInfo::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::FileInfo::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloads::RsaKey::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::RsaKey::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}

    else if (payloads::FailureList::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::FailureList::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloads::UserRequest::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::UserRequest::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloads::XkmRequest::ATTRIBUTE_IDS.contains(&root_node.attribute))
{
    let decoded = payloads::XkmRequest::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
}
else if (payloads::DeviceCombiState::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloads::DeviceCombiState::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
    println!("{decoded:#?}");
    }
    else if (payloads::SfValueList::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloads::SfValueList::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloads::PSContext::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloads::PSContext::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloads::CSContext::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloads::CSContext::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }

    else if (payloads::Failure::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloads::Failure::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }

    else if (payloads::CSHoursOfOperation::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloads::CSHoursOfOperation::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloads::FeatureList::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloads::FeatureList::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
        println!("{decoded:#?}");
    }
    else if (payloads::DeviceNotifications::ATTRIBUTE_IDS.contains(&root_node.attribute))
    {
        let decoded = payloads::DeviceNotifications::from_parse_tree(Dop2Payloads::MStruct(root_node.root_struct));
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

    #[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, EnumIter, EnumString, strum_macros::Display, IntoPrimitive)]
pub enum UserRequestOven {
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
    KeepWarm = 21,
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
    WifiOff=112,
}

#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive, EnumIter, EnumString, strum_macros::Display, IntoPrimitive)]
pub enum ProgramIdOven {
    NoProgram = 0,
    DefrostBottom = 1,
    AutoHotAir = 2,
    AutoTopBottomHeat = 3,
    EcoHotAir1 = 4,
    EcoHotAir2 = 5,
    EcoHotAir = 6,
    RoastAutomatic = 7,
    UniversalCook = 8,
    Grill = 9,
    GrillLarge = 10,
    GrillSmall = 11,
    Descale = 12,
    HotAirPlus = 13,
    IntenseBaking = 14,
    ComboCookGrill = 15,
    ComboCookHotAirPlus = 16,
    ComboCookTopBottom = 17,
    CakeSpecial = 18,
    Microwave = 19,
    MicrowaveRoastAutomatic = 20,
    MicrowaveGrill = 21,
    MicrowaveHotAirPlus = 22,
    MicrowaveFanGrill = 23,
    TopBottomHeat = 24,
    TopHeat = 25,
    Pyrolysis = 26,
    RapidHeat = 27,
    Rinse = 28,
    FanGrill = 29,
    EcoHotAir3 = 30,
    BottomHeat = 31,
    KeepWarm = 32,
    DefrostSteam = 33,
    DefrostMicrowave = 34,
    ClimateRoastAutomatic = 35,
    ConvectionBake = 36,
    ConvectionRoast = 37,
    MicrowaveConvectionBake = 38,
    MicrowaveConvectionRoast = 39,
    ClimateHotAirPlus = 40,
    HeatBottom = 41,
    HeatSteam = 42,
    HeatMicrowave = 43,
    CookFish = 44,
    CookMeat = 45,
    CookVegetables = 46,
    ClimateConvectionBake = 47,
    ClimateRoast = 48,
    ClimateHotAirPlus2 = 49,
    ClimateIntenseBaking = 50,
    ClimateTopBottomHeat = 51,
    ClimateConvectionRoast = 52,
    Popcorn = 53,
    QuickMicrowave = 54,
    RoastAutomaticRf = 55,
    ConvectionBakeRf = 56,
    GrillRf = 57,
    HotAirPlusRf = 58,
    IntenseBakingRf = 59,
    TopBottomHeatRf = 60,
    TopHeatRf = 61,
    FanGrillRf = 62,
    BottomHeatRf = 63,
    ConvectionRoastRf = 64,
    SteamHeat = 65,
    SteamHold = 66,
    SteamAltitudeAdjust = 67,
    DescaleSoak = 68,
    DescaleRinse = 69,
    SteamResidualWater = 70,
    Rotisserie = 71,
    SousVide = 72,
    SteamDry = 73,
    ClimateIntenseBaking2 = 74,
    EcoUniversalCook = 75,
    ClimateTopBottomHeat2 = 76,
    ComboCookMicrowave = 77,
    RotisserieLarge = 78,
    RotisserieSmall = 79,
    RotisserieFan = 80,
    AutoRoastAutomatic = 81,
    AutoIntenseBaking = 82,
    AutoBottomHeat = 83,
    AutoTopHeat = 84,
    SabbathTopBottom = 85,
    SabbathBottomHeat = 86,
    AutoSteamCook = 87,
    HydrocleanBottomHeat = 88,
    SoloRf = 89,
    HydrocleanTopBottomHeat = 90,
    HydrocleanUniversalCook = 91,
    DryHotAirPlusWithSteam = 92,
    DrySteamOnly = 93,
    DryGrillWithSteam = 94,
}



#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive, EnumIter, EnumString, strum_macros::Display, IntoPrimitive)]
pub enum SelectionType
{
	InitialAsConfigured = 0,
	InitialDefault = 1,
	Parametrized = 2,
	Deselect = 3,
	InitialFull = 4,
	InitialAsConfiguredViaSyndication = 10,
	InitialDefaultViaSyndication = 11,
	ParametrizedTemperature = 12,
	Last = 13
}


#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]

pub enum SfId {
    None = 0,
    TimeDisplay = 3,
    TimePresentation = 4,
    TimeFormat = 5,
    StartScreen = 10,
    DisplayBrightness = 11,
    DisplayColorScheme = 12,
    VolumeSignalTones = 14,
    VolumeKeyTone = 17,
    WelcomeMelodyVolume = 18,
    Lighting = 21,
    TemperatureUnit = 22,
    WeightUnit = 23,
    SafetyKeyLock = 24,
    StartupLock = 25,
    FurnitureFrontRecognition = 26,
    SensorLightOnApproach = 27,
    SensorDisplayOnApproach = 28,
    SensorLowerToneOnApproach = 29,
    RemoteControl = 30,
    SupervisionFunction = 31,
    SupervisionDisplayInStandby = 32,
    RemoteUpdate = 33,
    VoiceControl = 34,

    BaseLanguageCountry = 1000,
    QuickMicrowaveDuration = 1001,
    QuickMicrowavePower = 1002,
    PopcornDuration = 1003,
    KeepWarmMicrowave = 1004,
    KeepWarmDishwasher = 1005,
    KeepWarmOven = 1006,
    AutomaticFlushing = 1007,
    SteamReduction = 1008,
    PyrolysisRequest = 1009,
    PanningScreen = 1010,
    CoolAirFollowUp = 1011,
    ProposedTemperatures = 1012,
    ProposedMicrowavePower = 1013,
    NetVoltageAndFrequency = 1014,
    ProbeSensorGroup = 1015,
    TemperatureCalibration = 1016,
    CameraActivation = 1017,
    FastCooling = 1018,
    ApplianceVariantId = 1019,
    CurrentRate = 1020,
    WaterHardness = 1021,
    CarbonateHardness = 1022,
    Altitude = 1023,
    FreshWater = 1024,
    PerformanceMode = 1025,
    SpoutAdjustment = 1026,
    ProfileChange = 1027,
    ExpertMode = 1028,
    LightingOnTurnedOn = 1029,
    LightingOnTurnedOff = 1030,
    ApplianceLightSwitchOffDelay = 1031,
    SensorLightActivatesWhenOff = 1032,
    SensorLightActivatesWhenOn = 1033,
    DemoMode = 1034,

    AutonomousCleaningActive = 1041,
    AutonomousMaintenance = 1042,
    BeanSorting = 1043,
    ProximitySensorLight = 1044,
    TeaTimerActivation = 1045,
    Timer1OnTime = 1046,
    Timer1OffTime = 1047,
    Timer1OffDelay = 1048,
    Timer1WeekdayAssignment = 1049,
    Timer1OnActive = 1050,
    Timer1OffActive = 1051,
    Timer2OnTime = 1052,
    Timer2OffTime = 1053,
    Timer2OffDelay = 1054,
    Timer2WeekdayAssignment = 1055,
    Timer2OnActive = 1056,
    Timer2OffActive = 1057,
    MaintenanceTimer1OnTime = 1058,
    MaintenanceTimer1WeekdayAssignment = 1059,
    MaintenanceTimer1OnActive = 1060,
    MaintenanceTimer2OnTime = 1061,
    MaintenanceTimer2WeekdayAssignment = 1062,
    MaintenanceTimer2OnActive = 1063,
    ActiveUser = 1064,
    FreshWaterControlEnabled = 1065,
    BeanPortioningEnabled = 1066,
    SteamExtractionEnabled = 1067,
    WaterSofteningEnabled = 1068,
    BoosterFunction = 1069,
    QuickTouchDisplay = 1070,
    CupHeater = 1071,
    MultiZoneFoodProbeAdd = 1072,
    MultiZoneFoodProbeSelect = 1073,
    AutomaticPanelMovement = 1074,
    SmartFoodId = 1075,
    AltitudeAdjustment = 1076,

    // Example of proposed cooking temperatures
    ProposedDefrostOven = 1101,
    ProposedHotAirAuto = 1102,
    ProposedTopBottomHeatAuto = 1103,
    ProposedSteamCook1 = 1104,
    ProposedSteamCook2 = 1105,
    ProposedEcoHotAir = 1106,
    ProposedRoastingAutomatic = 1107,
    ProposedUniversalCooking = 1108,
    ProposedGrill = 1109,
    ProposedLargeGrill = 1110,
    ProposedSmallGrill = 1111,
    ProposedHotAirPlus = 1113,
    ProposedIntensiveBaking = 1114,
    ProposedCombinationGrill = 1115,
    ProposedCombinationSteamOven = 1116,
    ProposedCombinationOven = 1117,
    ProposedSpecialCake = 1118,
    ProposedTopBottomHeat = 1124,
    ProposedTopHeat = 1125,
    ProposedQuickHeat = 1127,
    ProposedConvectionGrill = 1129,
    ProposedBottomHeat = 1131,
    ProposedKeepWarm = 1132,
    ProposedDefrostDishwasher = 1133,
    ProposedConvectionBake = 1136,
    ProposedHeatOven = 1141,
    ProposedHeatDishwasher = 1142,
    ProposedFishCooking = 1144,
    ProposedMeatCooking = 1145,
    ProposedVegetableCooking = 1146,
    ProposedClimateRoastingAutomatic = 1148,
    ProposedClimateHotAirPlus = 1149,
    ProposedClimateIntensiveBaking = 1150,
    ProposedClimateTopBottomHeat = 1151,
    ProposedRotisserie = 1171,
    ProposedSousVideCooking = 1172,
    ProposedSurroundBake = 1173,
    ProposedEcoUniversalCooking = 1175,
    ProposedClimateCooking = 1176,
    ProposedCombinationMicrowaveCooking = 1177,
    ProposedLargeRotisserie = 1178,
    ProposedSmallRotisserie = 1179,
    ProposedConvectionRotisserie = 1180,
    ProposedConvectionRoast = 1181,
    ProposedMicrowavePower2 = 1219,
    ProposedMicrowavePowerAutomatic = 1220,
    ProposedMicrowavePowerGrill = 1221,
    ProposedMicrowavePowerHotAirPlus = 1222,
    ProposedMicrowavePowerConvectionGrill = 1223,
    ProposedCombinationMicrowavePower = 1277,
    ReservedInvalid = 32767,
}


#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum ShowMeHowId {
    None = 0,
    ReservedInvalid = 32767,

    CaOvDescaling1 = 51,
    CaOvDescaling2 = 52,
    CaOvDescaling3 = 53,
    CaOvDrawInWater = 54,
    CaOvUseWirelessFoodprobe = 55,
    CaOvUseWiredFoodprobe = 56,
    CaOvRotisserie = 57,
    CaOvAfterPyrolyticCleaning = 58,

    CaSovcUseWirelessFoodprobe = 59,
    CaSovcUseWiredFoodprobe = 60,
    CaSovcFreshWaterFill = 61,
    CaSovcEmptyCondensateTank = 62,
    CaSovcFlushWaterTankAndFill = 63,
    CaSovcDescaling = 64,
    CaSovcFlushFreshWater = 65,
    CaSovcPurgeFreshWaterFill = 66,
    CaSovcPurgeEmptyCondensateTank = 67,

    CaSovmFreshWaterFill = 68,
    CaSovmFlushFreshWaterFill = 69,
    CaSovmDescaling = 70,
    CaSovmFlushFreshWater = 71,
    CaOvmUseWiredFoodprobe = 72,

    CoFillWatertankWithWaterAndDescalingAgent = 73,
    CoPlaceMaintenanceContainerUnderSpout = 74,
    CoEmptyDripTrayWasteContainerCleanContactsAndPlaceBack = 75,
    CoRinseFillInsertWaterContainer = 76,
    CoRemoveWaterContainerBrewUnitRinseBrewUnit = 77,
    CoInsertBrewUnitWithTablet = 78,
    CoFillWatertankWithWaterAndCleaningAgent = 79,
    CoRinseInsertWaterContainer = 80,
    CoUnwrapDescalingCartidgeAndFitAsDescribed = 81,
    CoUnwrapCleaningCartidgeAndFitAsDescribed = 82,
    CoUnwrapCleaningAndDescalingCartidgeAndFitAsDescribed = 83,
    CoFitMilkValveConnectMilkPipework = 84,
    CoNewDescalingCartidgeIsFlooded = 85,

    CaSovcUseWiredFoodprobe2 = 86,
    CaOvRotisserieR36 = 87,
    CaOvRotisserieR48 = 88,
    CaOvReplugWirelessFoodprobe = 89,
    CaSovcReplugWirelessFoodprobe = 90,
    CoInsertAdapter = 91,
    CoRemoveAndCleanMilkValve = 92,
    CaSovcRemoveAccessoriesAndShelfRunners = 93,
    CaSovcDropBroilingElementDownAndRemoveCoarseSoiling = 94,
    CaSovcInsertFilterInTheFloorAndPourCleaningAgent = 95,
    CaSovcRaiseBroilingElementAndRefitShelfRunnersAndAccessories = 96,
    CaOvUseWirelessFoodprobeNa30 = 97,
    CaOvUseWirelessFoodprobeR30R36 = 98,
    CaOvUseWirelessFoodprobeR48 = 99,
}
/* 
#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
pub enum UserRequestId {
    None = 0,
    StartProgram = 1,
    StopProgram = 2,
    PauseProgram = 3,
    StartSuperFreezing = 4,
    StopSuperFreezing = 5,
    StartSuperCooling = 6,
    StopSuperCooling = 7,
    StartDelayProgram = 8,
    DoorOpen = 11,
    DoorClose = 12,
    LightOn = 13,
    LightOff = 14,
    FactorySettings = 15,
    SwitchOnPanel = 16,
    Next = 17,
    Back = 18,
    SwitchOffPanel = 19,
    ResetPincode = 20,
    KeepAlive = 21,
    Step = 22,
    StartRemoteUpdateInstall = 23,
    CheckSupervisorCode = 24,
    ChangeSupervisorCode = 25,
    ResetSupervisorCode = 26,
    SupervisorCodeLocked = 27,
    StopProgramEndSignal = 28,
    CheckDoorCode = 29,
    ChangeDoorCode = 30,
    TracePersistenceActivate = 51,
    TracePersistenceDeactivate = 52,
    ProgramStop = 54,
    ProgramAbort = 55,
    ProgramFinalize = 56,
    ProgramClose = 57,
    BurstOfSteamStart = 59,
    BurstOfSteamStop = 60,
    ProgramSave = 61,
    IncrementQuickMwDuration = 62,
    FasciaPanelOpen = 65,
    FasciaPanelClose = 66,
    Backward = 67,
    HoldingBreak = 68,
    HoldingStart = 69,
    QuickMwStart = 70,
    PopcornStart = 71,
    ContinueCookingStart = 72,
    RfStart = 74,
    RfStop = 75,
    CsActuatorOff = 76,
    CsActuatorOn = 77,
    AbortPhase1 = 78,
    AbortPhase2 = 79,
    AbortPhase3 = 80,
    AbortPhase4 = 81,
    DoubleDispense = 82,
    SaveDispensePhase1 = 83,
    SaveDispensePhase2 = 84,
    SaveDispensePhase3 = 85,
    SaveDispensePhase4 = 86,
    ParameterSave = 87,
    ShowFooterInfo = 88,
    StartDrying = 89,
    AbortPhase5 = 90,
    AbortPhase6 = 91,
    AbortPhase7 = 92,
    AbortPhase8 = 93,
    MobileKeyPressed = 94,
    ProgramStepInfo = 95,
    ProgramInstructionInfo = 96,
    BackwardTwoSteps = 97,
    JumpToFoodIq = 98,
    PushToTalkStart = 99,
    PushToTalkBreak = 100,
    EnterSoftAccessPoint = 104,
    ProgressStart = 105,
    ProgressStop = 106,
    ProgressOk = 107,
    ShopWindowSequenceAllowed = 108,
    SwitchApplianceOnDemoMode = 109,
    StartOwnProgramModification = 110,
    EnableWiFi = 111,
    DisableWiFi = 112,
    StartLocal = 113,
    ResetInitialGrinding = 114,
    GlobalMax = 50,
    ReservedInvalid = 32767,
}
*/

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

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum DeviceType {
    None,
    Washer,
    Dryer,
    WasherSemiPro,
    DryerSemiPro,
    WasherPro,
    DryerPro,
    Dishwasher,
    DishwasherSemiPro,
    DishwasherPro,
    Cooker,
    Microwave,
    Oven,
    OvenMicrowaveCombo,
}

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, EnumIter, EnumString, strum_macros::Display)]
pub enum ProtocolType {
    Unknown = 0,
    Uart = 1,
    MeterBusDop1 = 2,
    MeterBusDop2 = 3,
    HdrDop2=4,
    HdrMaci,
    SpiMaci,
    SdioMaci,
    UartMaci,
    DbusDop2=200,
    TodDop2=201,
    UsbDop2=202
}

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



#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq, IntoPrimitive)]
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
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq, IntoPrimitive)]
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
#[derive(Debug, Clone, TryFromPrimitive, PartialEq, Eq, IntoPrimitive)]
enum ProcessState
{
    Unknown,
    NoProgram,
    ProgramSelected,
    ProgramStart,
    ProgramRunning
}
/*
macro_rules! impl_tryfrom_wrapper {
    ($enum:ty, $wrapper:ty) => 
{
        impl TryFrom<$wrapper> for $enum {
            type Error = ();

            fn try_from(value: $wrapper) -> Result<Self, <$enum as TryFrom<$wrapper>>::Error> {
                <$enum>::try_from(value.0).map_err(|_| ())
            }
        }
        impl TryFrom<DopArray<$wrapper>> for Vec<$enum>
	{
            type Error = ();
	fn try_from (value: DopArray<$wrapper>) -> Result <Self, <$enum as TryFrom<$wrapper>>::Error>
         {
         Err("garbage2")
}

}
}*/
macro_rules! impl_into_wrapper {
    ($enum:ty, $wrapper:ty) => {
        impl Into<$wrapper> for $enum {
            fn into(self) -> $wrapper {
                <$wrapper>::from(self)
            }
        }

        impl Into<DopArray<$wrapper>> for Vec<$enum> {
            fn into(self) -> DopArray<$wrapper> {
                DopArray {
                    count: self.len() as u16,
                    elements: self.into_iter().map(|e| e.into()).collect(),
                }
            }
        }
    };
}


macro_rules! impl_tryfrom_wrapper { // example ProcessState, E8
($enum:ty, $wrapper:ident) => {

impl TryFrom<Vec<$enum>> for DopArray<$wrapper> {
    type Error = String;
    fn try_from (value: Vec<$enum>)-> Result<Self, String>
    {
        let elements : Vec<$wrapper> = value
        .into_iter()
        .map(|elem| <$wrapper>::try_from(elem).unwrap().into())
        .collect();
        Ok(DopArray{count: elements.len() as u16, elements})
    }
}
impl TryFrom<$wrapper> for $enum {
type Error = ();

        fn try_from(value: $wrapper) -> Result<Self, <$enum as TryFrom<$wrapper>>::Error> {
            <$enum>::try_from(value.0).map_err(|_| ()) }
        }
    

    impl TryFrom<DopArray<$wrapper>> for Vec<$enum> {
        type Error = ();

        fn try_from(value: DopArray<$wrapper>) -> Result<Self, Self::Error> {
            value
                .elements
                .into_iter()
                .map(|elem| <$enum>::try_from(elem))
                .collect()
        }
    }
impl From<$enum> for $wrapper
{
    fn from (value: $enum) -> $wrapper
    {
        $wrapper (value.into()) // TODO: Fix this
    }
}
}    
}


impl_tryfrom_wrapper!(ProcessState, E8);

impl_tryfrom_wrapper!(OperationState, E8);

impl_tryfrom_wrapper!(ProgramIdOven, E16);

impl_tryfrom_wrapper!(SelectionType, E8);

impl_tryfrom_wrapper!(ApplianceState, E8);

impl_tryfrom_wrapper!(NotificationAckOption, E8);

//impl_into_wrapper!(XkmRequestId, E8);

//impl_tryfrom_wrapper!(UserRequestId, E16);
impl_tryfrom_wrapper!(UserRequestOven, E16);

impl_tryfrom_wrapper!(SfId, E16);

impl_tryfrom_wrapper!(XkmRequestId, E8);

impl_tryfrom_wrapper!(ValueInterpretation, E8);

impl_tryfrom_wrapper!(ShowMeHowId, E16);

impl_tryfrom_wrapper!(DeviceType, E8);
impl_tryfrom_wrapper!(ProtocolType, E8);
//impl_tryfrom_wrapper!(Dop2TimestampUtc, u64);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct UserRequest {
        #[dop2field(1, Dop2Payloads::E16)]
        pub request_id : UserRequestOven,
        /*#[dop2field(2, Dop2Payloads::E8)]
        operation_state : OperationState,
        #[dop2field(3, Dop2Payloads::E8)]
        process_state : ProcessState*/
}
impl UserRequest
{
    pub const ATTRIBUTE_IDS : &[u16] = &[1583];
}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceCombiState {
        #[dop2field(1, Dop2Payloads::E8)]
        appliance_state : ApplianceState,
        #[dop2field(2, Dop2Payloads::E8)]
        operation_state : OperationState,
        #[dop2field(3, Dop2Payloads::E8)]
        process_state : ProcessState
}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SupportedApplications {
        #[dop2field(1, Dop2Payloads::Boolean)]
        mieleAtHome : bool,
        #[dop2field(2, Dop2Payloads::Boolean)]
        remoteVision : bool,
         #[dop2field(3, Dop2Payloads::Boolean)]
        superVision : bool,
        #[dop2field(4, Dop2Payloads::Boolean)]
        smartGrid : bool,
        #[dop2field(5, Dop2Payloads::Boolean)]
        mobileControl : bool,
         #[dop2field(6, Dop2Payloads::Boolean)]
        unknown1 : bool,
        #[dop2field(7, Dop2Payloads::Boolean)]
        unknown2 : bool,
        #[dop2field(8, Dop2Payloads::Boolean)]
        voiceControl : bool,
        #[dop2field(9, Dop2Payloads::Boolean)]
        unknown3 : bool,
        #[dop2field(10, Dop2Payloads::Boolean)]
        featureList : bool,
        #[dop2field(11, Dop2Payloads::Boolean)]
        washToDry : bool,
}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceIdent {
        #[dop2field(1, Dop2Payloads::E8)]
        device_type : DeviceType,
        #[dop2field(2, Dop2Payloads::E8)]
        protocol_type : ProtocolType,
        #[dop2field(5, Dop2Payloads::MStruct)]
        supported_apps : SupportedApplications,
      //  #[dop2field(6, Dop2Payloads::U8)] //missing in washer
       // unit_count : u8,
        #[dop2field(9, Dop2Payloads::E16)] //should be an enum in washer
        rf_variant : E16,
}

impl DeviceIdent
{
    pub const ATTRIBUTE_IDS : &[u16] = &[144];
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

macro_rules! impl_tryfrom_dop2struct {
($target:ty) => {
impl TryFrom<Dop2Struct> for $target {
type Error = String;

        fn try_from(value: Dop2Struct) -> Result<Self, String> {
            <$target>::from_parse_tree(Dop2Payloads::MStruct(value))
        }
    }


impl TryFrom<DopArray<Dop2Struct>> for Vec<$target>
{
    type Error = String;

    fn try_from(value: DopArray<Dop2Struct>) -> Result<Vec<$target>, String> {
             Ok(value.elements.into_iter().map(|x| TryInto::<$target>::try_into(x).unwrap()).collect())
        }
    
}
}
}

//impl_tryfrom_dop2struct!(UserRequestOven);
//impl_tryfrom_dop2struct!(ApplianceState);
//impl_tryfrom_dop2struct!(SelectionType);
impl_tryfrom_dop2struct!(SupportedApplications);
impl_tryfrom_dop2struct!(RsaKey);
impl_tryfrom_dop2struct!(ErrorInfo);
impl_tryfrom_dop2struct!(NotificationInfo);
impl_tryfrom_dop2struct!(QueryInfo);
impl_tryfrom_dop2struct!(MessageInfo);
impl_tryfrom_dop2struct!(UserRequest);
impl_tryfrom_dop2struct!(DeviceIdent);
impl_tryfrom_dop2struct!(DeviceCombiState);
impl_tryfrom_dop2struct!(DeviceNotifications);
impl_tryfrom_dop2struct!(PsSelect);
impl_tryfrom_dop2struct!(XkmRequest);
impl_tryfrom_dop2struct!(PSAttributesCCA);
impl_tryfrom_dop2struct!(DeviceAttributesCCA);

impl_tryfrom_dop2struct!(ProgramInfoOven);
impl_tryfrom_dop2struct!(ProgramStepInfoOven);

impl_tryfrom_dop2struct!(PSContextParametersOven);

impl_tryfrom_dop2struct!(CSContextParametersOven);
impl_tryfrom_dop2struct!(CSContextParametersCoffeeMaker);

impl_tryfrom_dop2struct!(AnnotatedBool);
impl_tryfrom_dop2struct!(AnnotatedU8);


impl_tryfrom_dop2struct!(GenericU8);

impl_tryfrom_dop2struct!(AnnotatedU16);
impl_tryfrom_dop2struct!(GenericU16);
//impl_tryfrom_dop2struct!(AnnotatedU32);
impl_tryfrom_dop2struct!(AnnotatedU64);
impl_tryfrom_dop2struct!(AnnotatedTimeStamp);
impl_tryfrom_dop2struct!(FileList);

impl_tryfrom_dop2struct!(FailureListItem);
impl_tryfrom_dop2struct!(FailureList);
impl_tryfrom_dop2struct!(FeatureListOven);

impl_tryfrom_dop2struct!(Failure);

impl_tryfrom_dop2struct!(CSHoursOfOperation);

impl DeviceCombiState
{
    pub const ATTRIBUTE_IDS : &[u16] = &[1586];
/*
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
*/
}
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceContext // can be for main device or second device -- same struct 
{
        #[dop2field(1, Dop2Payloads::MStruct )]
        state : DeviceCombiState,
        #[dop2field(7, Dop2Payloads::MStruct )]
        prog : PSAttributesCCA,
        #[dop2field(8, Dop2Payloads::MStruct )]
        deviceAttributes : DeviceAttributesCCA,
        #[dop2field(9, Dop2Payloads::ArrayE16 )]
        supportedUserRequests : Vec<UserRequestOven>,
        #[dop2field(11, Dop2Payloads::Boolean)]
        mobileStartActive : bool,
       #[dop2field(12, Dop2Payloads::E16 )]
        showMeHowId : ShowMeHowId,
        #[dop2field(13, Dop2Payloads::Boolean )]
        requestTimeSync : bool,

}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceNotifications // can be for main device or second device -- same struct 
{
    //    #[dop2field(1, Dop2Payloads::AStruct )]
      //  info : Vec<NotificationInfo>, // this field appears to be OPTIONAL on a washer. present on an oven.

        #[dop2field(2, Dop2Payloads::AStruct )]
        messages : Vec<MessageInfo>,

       #[dop2field(3, Dop2Payloads::AStruct )]
        errors : Vec<ErrorInfo>,

       // #[dop2field(4, Dop2Payloads::AStruct )] // also optional on washer
       // queries : Vec<QueryInfo>,

}

impl DeviceNotifications
{
    pub const ATTRIBUTE_IDS : &[u16] = &[131, 392];
}


#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct NotificationInfo // can be for main device or second device -- same struct 
{
         #[dop2field(1, Dop2Payloads::E16 )]
        id : E16,
        #[dop2field(3, Dop2Payloads::ArrayE8 )]
        ackOptions : Vec<NotificationAckOption>,

}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct RsaKey 
{
    #[dop2field(1, Dop2Payloads::ArrayU8 )]
    key : DopArray<u8>,
}

impl RsaKey
{
    pub const ATTRIBUTE_IDS : &[u16] = &[287];
}
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct MessageInfo // can be for main device or second device -- same struct 
{
    #[dop2field(1, Dop2Payloads::E16 )]
    id : E16,
    #[dop2field(3, Dop2Payloads::ArrayE8 )]
    ackOptions : Vec<NotificationAckOption>,
}
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ErrorInfo // can be for main device or second device -- same struct 
{
    #[dop2field(1, Dop2Payloads::U32 )]
    id : u32,

    #[dop2field(2, Dop2Payloads::ArrayE8 )]
    ackOptions : Vec<NotificationAckOption>,

}
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct QueryInfo // can be for main device or second device -- same struct 
{
    #[dop2field(1, Dop2Payloads::E16 )]
    id : E16,

}
/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
enum FailureCode
{

}*/

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FailureListItem
{
        #[dop2field(1, Dop2Payloads::U32 )]
        failureCode : u32,
        #[dop2field(2, Dop2Payloads::Boolean )]
        presentNow : bool,
}
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FailureList
{
        #[dop2field(1, Dop2Payloads::AStruct )]
        items : Vec<FailureListItem>,

}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FileList
{
        #[dop2field(1, Dop2Payloads::MString )]
        filename : String,

        #[dop2field(2, Dop2Payloads::ArrayU8 )]
        sha256 : DopArray<u8>,


        #[dop2field(3, Dop2Payloads::MString )]
        description : String,

        #[dop2field(4, Dop2Payloads::E8 )]
        fileAccessMode : E8,

        #[dop2field(5, Dop2Payloads::U32 )]
        size : u32,
}

impl FileList
{
    pub const ATTRIBUTE_IDS : &[u16] = &[333];
}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FileInfo
{
        #[dop2field(1, Dop2Payloads::MString )]
        filename : String,

        #[dop2field(2, Dop2Payloads::ArrayU8 )]
        sha256 : DopArray<u8>,

        #[dop2field(3, Dop2Payloads::U32 )]
        currentSize : u32,

        #[dop2field(4, Dop2Payloads::U32 )]
        maxSize : u32,
}

impl FileInfo
{
    pub const ATTRIBUTE_IDS : &[u16] = &[1588];
}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct Failure
{
        #[dop2field(1, Dop2Payloads::U32 )]
        failureCode : u32,

        #[dop2field(2, Dop2Payloads::Boolean )]
        active : bool,

        #[dop2field(3, Dop2Payloads::U16 )]
        occurrenceFrequency : u16,

        #[dop2field(4, Dop2Payloads::U64 )]
        occurrenceTime : Dop2TimestampUtc,

        #[dop2field(5, Dop2Payloads::U32 )]
        operationSeconds : u32,

        #[dop2field(6, Dop2Payloads::U16 )]
        progId : u16,

        #[dop2field(7, Dop2Payloads::U16 )]
        blockNumber : u16,

}
impl Failure
{
    pub const ATTRIBUTE_IDS : &[u16] = &[117];
}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FeatureListOven
{
        #[dop2field(1, Dop2Payloads::U16 )]
        deviceId : u16,

}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct FeatureList
{
        #[dop2field(1, Dop2Payloads::E8 )]
        deviceId : E8,

        
        #[dop2field(2, Dop2Payloads::E8 )]
        deviceClass : E8,


        #[dop2field(3, Dop2Payloads::U16 )]
        deviceSubClass : u16,

     /*   #[dop2field(4, Dop2Payloads::E8 )]
        deviceGeneration : E8, */

        #[dop2field(5, Dop2Payloads::Boolean )]
        hasSearch : bool,

        #[dop2field(6, Dop2Payloads::Boolean )]
        hasCamera : bool,

        #[dop2field(7, Dop2Payloads::E8 )]
        deviceIdSubType : E8,

        #[dop2field(131, Dop2Payloads::MStruct )]
        featureListOven : FeatureListOven,




}
impl FeatureList
{
    pub const ATTRIBUTE_IDS : &[u16] = &[348];
}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CSHoursOfOperation
{
        #[dop2field(1, Dop2Payloads::U32 )]
        hoursOfOperation : u32,

        
        #[dop2field(2, Dop2Payloads::U32 )]
        hoursOfOperationBeforeReplacement : u32,


        #[dop2field(3, Dop2Payloads::U32 )]
        hoursOfOperationSinceLastMaintenance : u32,

        #[dop2field(4, Dop2Payloads::U32 )]
        hoursOfOperationMode1 : u32,

        #[dop2field(5, Dop2Payloads::U32 )]
        hoursOfOperationMode2 : u32,

}
impl CSHoursOfOperation
{
    pub const ATTRIBUTE_IDS : &[u16] = &[119];
}

impl FailureList
{
    pub const ATTRIBUTE_IDS : &[u16] = &[148];
   
}
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct SfValueList
{
    #[dop2field(1, Dop2Payloads::U8 )]
    valid_count : u8,
    #[dop2field(2, Dop2Payloads::ArrayE16 )]
    valid : Vec<SfId>,
}

impl SfValueList
{
    pub const ATTRIBUTE_IDS : &[u16] = &[114];
    
}

impl DeviceContext
{
    pub const ATTRIBUTE_IDS : &[u16] = &[391, 1585];
    
}


#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceAttributesCCA {
//    #[dop2field(1, Dop2Payloads::U32 )]
//    milkCleaningCntr : u32,
    #[dop2field(11, Dop2Payloads::E8 )]
    doorLock : E8,
}

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct CSContext
{
    #[dop2field(1, Dop2Payloads::E16)]
    pub(crate) program_id : ProgramIdOven,

    #[dop2field(3, Dop2Payloads::MStruct )]
    contextOven : CSContextParametersOven,

    #[dop2field(4, Dop2Payloads::MStruct )]
    contextcoffeMaker: CSContextParametersCoffeeMaker
}
impl CSContext
    {
        pub const ATTRIBUTE_IDS : &[u16] = &[154]; // always in unit 1?
    }

    #[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
    pub struct CSContextParametersOven
    {
        #[dop2field(1, Dop2Payloads::MStruct )]
        open : AnnotatedBool,
         #[dop2field(2, Dop2Payloads::MStruct )]
        lock : AnnotatedBool,
        #[dop2field(3, Dop2Payloads::MStruct )]
        on : AnnotatedBool,
        #[dop2field(4, Dop2Payloads::MStruct )]
        level : GenericU8,
    }    

    #[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
    pub struct CSContextParametersCoffeeMaker
    {
        #[dop2field(5, Dop2Payloads::MStruct )]
        ceramicValve : GenericU8,
        
         #[dop2field(6, Dop2Payloads::MStruct )]
        brewingUnit : GenericU8,
        #[dop2field(7, Dop2Payloads::MStruct )]
        pump : GenericU8,
#[dop2field(8, Dop2Payloads::MStruct )]
        spout : GenericU8, 

        #[dop2field(12, Dop2Payloads::MStruct )]
        fan : GenericU8, 
    }    
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct PSContext 
{
    #[dop2field(4, Dop2Payloads::MStruct )]
    contextOven : PSContextParametersOven,
    #[dop2field(7, Dop2Payloads::MStruct )]
    attributesOven : PSAttributesCCA,
}

impl PSContext
    {
        pub const ATTRIBUTE_IDS : &[u16] = &[1574]; // always in unit 1?
    }


    #[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ProgramStepInfoOven 
{
    #[dop2field(1, Dop2Payloads::U8 )]
    stepNumber : u8,
    #[dop2field(2, Dop2Payloads::E8 )]
    stepType : E8,
    #[dop2field(3, Dop2Payloads::U16 )]
    operationMode : u16,
    #[dop2field(4, Dop2Payloads::U16 )]
    temperatureSetpoint : u8,
    #[dop2field(5, Dop2Payloads::U8 )]
    setGrillLevel : u8,
    #[dop2field(6, Dop2Payloads::U8 )]
    mwPower : u8,
    #[dop2field(7, Dop2Payloads::U32 )]
    duration : u32,
    #[dop2field(8, Dop2Payloads::U16 )]
    moistsetCoreTemperature : u16, // TODO: add remaining fields
}

// TODO: implement ProgramList 2, 212

impl ProgramStepInfoOven
    {
        pub const ATTRIBUTE_IDS : &[u16] = &[214]; // always in unit 2?
    }
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
    
    #[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct PSContextParametersOven
{
   #[dop2field(1, Dop2Payloads::MStruct )]
    grill_level : GenericU8,
    #[dop2field(2, Dop2Payloads::MStruct )]
    moisture : GenericU8,
    #[dop2field(5, Dop2Payloads::MStruct )]
    level : GenericU8,
    #[dop2field(6, Dop2Payloads::MStruct )]
    temperature : GenericU16,
    //TODO
}
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct PSAttributesCCA {
    #[dop2field(1, Dop2Payloads::E16 )]
    progPhase : E16,

    #[dop2field(2, Dop2Payloads::E16 )]
    progSubPhase : E16,
        #[dop2field(3, Dop2Payloads::MStruct )]
        progress : AnnotatedU16,

        #[dop2field(6, Dop2Payloads::MStruct )]
        displayTemperature : AnnotatedU16,


        #[dop2field(7, Dop2Payloads::MStruct )]
        displayCoreTemperature : AnnotatedU16,

        #[dop2field(21, Dop2Payloads::MStruct )]
        temperatureSetpoint : AnnotatedU16,

        #[dop2field(22, Dop2Payloads::MStruct )]
        moistureSetpoint : AnnotatedU8,

        #[dop2field(24, Dop2Payloads::MStruct )]
        powerSetpoint : AnnotatedU8,


        #[dop2field(26, Dop2Payloads::MStruct )]
        startTime : AnnotatedTimeStamp,

        #[dop2field(29, Dop2Payloads::MStruct )]
        nextActionTime : AnnotatedTimeStamp,

}


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

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DateTimeInfo {
       #[dop2field(1, Dop2Payloads::U64)]
        utc_time : Dop2TimestampUtc, // TODO: bring this back
       #[dop2field(2, Dop2Payloads::I32)]
        utc_offset : i32
}

impl DateTimeInfo 
{
    pub const ATTRIBUTE_IDS : &[u16] = &[122];
}




#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct PsSelect {
       #[dop2field(1, Dop2Payloads::E16)]
        pub(crate) program_id : ProgramIdOven,

        #[dop2field(2, Dop2Payloads::U16)]
        pub(crate) selection_parameter : u16,

        #[dop2field(3, Dop2Payloads::E8)]
        pub(crate) selection_type : SelectionType,
        
}
    impl PsSelect
    {
        pub const ATTRIBUTE_IDS : &[u16] = &[1577];
/*        pub fn to_dop2_struct (&self) -> Result<Dop2Struct, String>
        {
            let mut fields: Vec<TaggedDopField> = vec!();
            let request_id_payload : Dop2Payloads = Dop2Payloads::E16(self.program_id.clone().into());
            let request_id_field : TaggedDopField = TaggedDopField{ field_index: 1, tag: Dop2PayloadsKind::from(request_id_payload.clone()), value: request_id_payload};
            let selection_parameter_payload : Dop2Payloads = Dop2Payloads::U16(self.selection_parameter.clone().into());
            let selection_parameter_field : TaggedDopField = TaggedDopField{ field_index: 2, tag: Dop2PayloadsKind::from(selection_parameter_payload.clone()), value: selection_parameter_payload};
            let selection_type_payload :  Dop2Payloads = Dop2Payloads::E8(self.selection_type.clone().into());
            let selection_type_field : TaggedDopField = TaggedDopField{ field_index: 3, tag: Dop2PayloadsKind::from(selection_type_payload.clone()), value: selection_type_payload};
           
            fields.push(request_id_field);
            fields.push(selection_parameter_field);
            fields.push(selection_type_field);
            Ok(Dop2Struct::from_fields (fields))
        } */

/*        pub fn from_parse_tree (payload: Dop2Payloads) -> Result<Self, String>
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
    */
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
