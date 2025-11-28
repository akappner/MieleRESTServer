// Root node and payload structures for DOP2 protocol

use crate::payloader::parser::{Dop2Parser, Dop2PayloadExpressible, ToDop2Bytes, DopArray};
use crate::payloader::helper::types::{E8, E16, E32, E64};
use enum_kinds::EnumKind;
use num_enum::TryFromPrimitive;

/// Root node structure for DOP2 protocol
#[allow(dead_code)]
#[derive(Debug)]
pub struct RootNode {
    pub unit: u16,
    pub attribute: u16,
    pub declared_length: u16,
    pub idx1: u16,
    pub idx2: u16,
    pub root_struct: Dop2Struct,
}

impl RootNode {
    pub fn single(unit: u16, attribute: u16, root_struct: Dop2Struct) -> RootNode {
        RootNode {
            unit,
            attribute,
            declared_length: 0,
            idx1: 0,
            idx2: 0,
            root_struct,
        }
    }

    pub fn has_more_siblings(&self) -> bool {
        return self.idx1 == self.idx2;
    }

    pub fn parse(parser: &mut Dop2Parser) -> Result<RootNode, String> {
        let declared_length = parser.take_u16().unwrap(); // only for validation, not needed for further parsing

        let unit = parser.take_u16()?;
        let attribute = parser.take_u16()?;

        let idx1 = parser.take_u16().unwrap();
        let idx2 = parser.take_u16().unwrap();

        let root_struct = *Dop2Struct::parse(parser).unwrap();

        let _padding = DopPadding::parse(parser).unwrap();
        assert!(parser.is_empty()); // no trailing garbage

        Ok(RootNode {
            unit,
            attribute,
            declared_length,
            idx1,
            idx2,
            root_struct,
        })
    }

    pub fn to_bytes(self, builder: &mut Vec<u8>) {
        builder.extend(self.unit.to_be_bytes());
        builder.extend(self.attribute.to_be_bytes());

        builder.extend(self.idx1.to_be_bytes());
        builder.extend(self.idx2.to_be_bytes());

        self.root_struct.to_bytes(builder);
        let length: u16 = builder.len().try_into().unwrap();
        builder.splice(0..0, (length).to_be_bytes()); // TODO: Fix this

        let padding = DopPadding::minimum_padding(builder);
        padding.to_bytes(builder);
    }
}

/// Tagged field structure for DOP2 protocol
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaggedDopField {
    pub tag: Dop2PayloadsKind,
    pub field_index: u16,
    pub value: Dop2Payloads,
}

impl TaggedDopField {
    pub fn from_payload(field_index: u16, value: Dop2Payloads) -> TaggedDopField {
        let tag = Dop2PayloadsKind::from(&value);
        return TaggedDopField {
            field_index,
            tag,
            value,
        };
    }

    pub fn get_length(&self) -> u16 {
        let size = std::mem::size_of_val(&self.tag) + std::mem::size_of_val(&self.field_index);
        size.try_into().unwrap()
    }

    pub fn to_bytes(self, vec: &mut Vec<u8>) {
        vec.extend(self.field_index.to_be_bytes());
        vec.push(self.tag as u8);

        match self.value {
            Dop2Payloads::Boolean(b) => b.to_bytes(vec),
            Dop2Payloads::U8(payload) => payload.to_bytes(vec),
            Dop2Payloads::U16(payload) => payload.to_bytes(vec),
            Dop2Payloads::U32(payload) => payload.to_bytes(vec),
            Dop2Payloads::U64(payload) => payload.to_bytes(vec),
            Dop2Payloads::I8(payload) => payload.to_bytes(vec),
            Dop2Payloads::I16(payload) => payload.to_bytes(vec),
            Dop2Payloads::I32(payload) => payload.to_bytes(vec),
            Dop2Payloads::I64(payload) => payload.to_bytes(vec),
            Dop2Payloads::E8(payload) => payload.to_bytes(vec),
            Dop2Payloads::E16(payload) => payload.to_bytes(vec),
            Dop2Payloads::E32(payload) => payload.to_bytes(vec),
            Dop2Payloads::E64(payload) => payload.to_bytes(vec),
            Dop2Payloads::MString(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayU8(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayU16(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayI8(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayI16(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayI32(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayE8(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayE16(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayE32(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayE64(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayU32(payload) => payload.to_bytes(vec),
            Dop2Payloads::ArrayU64(payload) => payload.to_bytes(vec),
            Dop2Payloads::MStruct(payload) => payload.to_bytes(vec),
            Dop2Payloads::AStruct(payload) => payload.to_bytes(vec),
            Dop2Payloads::Trash => todo!(),
            Dop2Payloads::ArrayBool(_) => todo!(),
            Dop2Payloads::ArrayI64(_) => todo!(),
            Dop2Payloads::ArrayF32(_) => todo!(),
            Dop2Payloads::ArrayF64(_) => todo!(),
            Dop2Payloads::F32(_) => todo!(),
            Dop2Payloads::F64(_) => todo!(),
        }
    }

    pub fn parse(parser: &mut Dop2Parser) -> Result<TaggedDopField, String> {
        let field_index = parser.take_u16().unwrap();
        let tag_byte = parser.take_u8()?;
        let tag = Dop2PayloadsKind::try_from_primitive(tag_byte)
            .map_err(|_| format!("Invalid Dop2Type value: 0x{:02X}", tag_byte))?;
        let value = match tag {
            Dop2PayloadsKind::Boolean => Dop2Payloads::Boolean(*bool::parse(parser).unwrap()),
            Dop2PayloadsKind::E8 => Dop2Payloads::E8(*E8::parse(parser).unwrap()),
            Dop2PayloadsKind::U8 => Dop2Payloads::U8(*u8::parse(parser).unwrap()),
            Dop2PayloadsKind::U16 => Dop2Payloads::U16(*u16::parse(parser).unwrap()),
            Dop2PayloadsKind::U64 => Dop2Payloads::U64(*u64::parse(parser).unwrap()),
            Dop2PayloadsKind::I8 => Dop2Payloads::I8(*i8::parse(parser).unwrap()),
            Dop2PayloadsKind::I16 => Dop2Payloads::I16(*i16::parse(parser).unwrap()),
            Dop2PayloadsKind::E16 => Dop2Payloads::E16(*E16::parse(parser).unwrap()),
            Dop2PayloadsKind::U32 => Dop2Payloads::U32(*u32::parse(parser).unwrap()),
            Dop2PayloadsKind::I32 => Dop2Payloads::I32(*i32::parse(parser).unwrap()),
            Dop2PayloadsKind::E32 => Dop2Payloads::E32(*E32::parse(parser).unwrap()),
            Dop2PayloadsKind::I64 => Dop2Payloads::I64(*i64::parse(parser).unwrap()),
            Dop2PayloadsKind::E64 => Dop2Payloads::E64(*E64::parse(parser).unwrap()),
            Dop2PayloadsKind::MString => Dop2Payloads::MString(*String::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayU8 => Dop2Payloads::ArrayU8(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayI8 => Dop2Payloads::ArrayI8(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayI16 => Dop2Payloads::ArrayI16(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayI32 => Dop2Payloads::ArrayI32(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayU32 => Dop2Payloads::ArrayU32(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayU16 => Dop2Payloads::ArrayU16(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayE8 => Dop2Payloads::ArrayE8(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayE16 => Dop2Payloads::ArrayE16(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayE32 => Dop2Payloads::ArrayE32(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayE64 => Dop2Payloads::ArrayE64(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::ArrayU64 => Dop2Payloads::ArrayU64(*DopArray::parse(parser).unwrap()),
            Dop2PayloadsKind::MStruct => Dop2Payloads::MStruct(*Dop2Struct::parse(parser).unwrap()),
            Dop2PayloadsKind::AStruct => Dop2Payloads::AStruct(*DopArray::parse(parser).unwrap()),

            garbage => {
                println!("unknown type {:?}", garbage);
                todo!()
            }
        };

        Ok(TaggedDopField { tag, field_index, value })
    }
}

/// DOP2 payloads enum
#[derive(Clone, Debug, PartialEq, Eq, EnumKind)]
#[enum_kind(Dop2PayloadsKind, derive(TryFromPrimitive), repr(u8))]
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
    MStruct(Dop2Struct),
    ArrayBool(DopArray<bool>),
    ArrayU8(DopArray<u8>),
    ArrayI8(DopArray<i8>),
    ArrayE8(DopArray<E8>),
    ArrayU16(DopArray<u16>),
    ArrayI16(DopArray<i16>),
    ArrayE16(DopArray<E16>),
    ArrayU32(DopArray<u32>),
    ArrayI32(DopArray<i32>),
    ArrayE32(DopArray<u32>),
    ArrayU64(DopArray<u64>),
    ArrayI64(DopArray<i32>),
    ArrayE64(DopArray<u64>),
    ArrayF32(DopArray<u8>), // todo
    ArrayF64(DopArray<i8>), // todo
    MString(String),
    AStruct(DopArray<Dop2Struct>),
}

/// DOP2 struct structure
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dop2Struct {
    pub declared_fields: u16,
    pub fields: Vec<TaggedDopField>,
}

impl Dop2Struct {
    pub fn get_field(&self, id: u16) -> Option<TaggedDopField> {
        self.fields
            .iter()
            .find(|x| x.field_index == id)
            .cloned() // TODO: check for duplicate indices and remove the clone
    }

    pub fn get_payload(&self, id: u16) -> Option<Dop2Payloads> {
        self.get_field(id).map(|x| x.value.clone())
    }

    pub fn from_fields(fields: Vec<TaggedDopField>) -> Self {
        let m = fields.iter();
        let index = m
            .max_by_key(|x| x.field_index)
            .map(|x| x.field_index)
            .unwrap_or(0);

        Dop2Struct {
            declared_fields: index,
            fields,
        }
    }

    pub fn get_length(&self) -> u16 {
        2 + self.fields.iter().map(|x| x.get_length()).sum::<u16>()
    }
}

impl ToDop2Bytes for Dop2Struct {
    fn to_bytes(self, vec: &mut Vec<u8>) {
        let field_count: u16 = self.fields.len().try_into().unwrap();
        vec.extend(field_count.to_be_bytes());
        self.fields.into_iter().for_each(|field| field.to_bytes(vec));
    }
}

impl Dop2PayloadExpressible for Dop2Struct {
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> {
        let declared_fields = parser.take_u16()?;
        let mut fields = Vec::new();
        for _x in 1..declared_fields + 1 {
            let tagged_field = TaggedDopField::parse(parser)?;
            fields.push(tagged_field);
        }
        Ok(Box::new(Dop2Struct {
            declared_fields,
            fields,
        }))
    }
}

/// Padding structure for DOP2 protocol
#[derive(Debug)]
pub struct DopPadding {
    pub bytes_of_padding: u8,
}

impl DopPadding {
    const PADDING_BYTE: u8 = 0x20;
    const PADDING_ALIGNMENT: u16 = 0x10;

    pub fn minimum_padding(builder: &Vec<u8>) -> DopPadding {
        let current = builder.len() as u16;
        return DopPadding {
            bytes_of_padding: ((DopPadding::PADDING_ALIGNMENT
                - (current % DopPadding::PADDING_ALIGNMENT))
                % DopPadding::PADDING_ALIGNMENT) as u8,
        };
    }

    pub fn parse(parser: &mut Dop2Parser) -> Result<DopPadding, String> {
        let mut bytes_of_padding = 0u8;
        while !parser.is_empty() {
            let byte = parser.take_u8().map_err(|e| e.to_string())?;
            if byte == DopPadding::PADDING_BYTE {
                bytes_of_padding += 1;
            } else {
                // If we read a non-0x20 byte, backtrack and stop
                return Err(format!("Non-padding byte 0x{:02X} read", byte));
            }
        }

        Ok(DopPadding { bytes_of_padding })
    }
}

impl ToDop2Bytes for DopPadding {
    fn to_bytes(self, vec: &mut Vec<u8>) {
        vec.extend(
            std::iter::repeat(DopPadding::PADDING_BYTE).take(self.bytes_of_padding.into()),
        );
    }
}

/// Trait for types that can be created from a DOP2 parse tree
pub trait Dop2ParseTreeExpressible: Sized {
    fn from_parse_tree(payload: Dop2Payloads) -> Result<Self, String>;
}

