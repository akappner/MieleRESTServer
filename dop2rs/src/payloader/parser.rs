// Parser and core parsing infrastructure for DOP2 protocol

/// Parser for DOP2 protocol byte streams
pub struct Dop2Parser {
    payload: Vec<u8>,
}

impl Dop2Parser {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }

    pub fn take(&mut self, n: usize) -> Result<Vec<u8>, &'static str> {
        if self.payload.len() < n {
            return Err("Not enough bytes in payload");
        }
        let bytes = self.payload.drain(..n).collect::<Vec<u8>>();
        Ok(bytes)
    }

    pub fn take_u16(&mut self) -> Result<u16, &'static str> {
        let bytes = self.take(2)?;
        Ok(((bytes[0] as u16) << 8) | bytes[1] as u16)
    }

    pub fn take_u8(&mut self) -> Result<u8, &'static str> {
        let bytes = self.take(1)?;
        Ok(bytes[0])
    }

    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

/// Trait for types that can be parsed from a DOP2 byte stream
pub trait Dop2PayloadExpressible {
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String>;
}

/// Trait for types that can be serialized to DOP2 bytes
pub trait ToDop2Bytes {
    fn to_bytes(self, vec: &mut Vec<u8>);
}

/// Array type for DOP2 protocol
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DopArray<T: Dop2PayloadExpressible + ToDop2Bytes> {
    pub count: u16,
    pub elements: Vec<T>,
}

impl DopArray<u8> {
    /// Converts the byte array to a hexadecimal string representation
    pub fn to_hex_str(&self) -> String {
        hex::encode(&self.elements)
    }
}


impl<T> Into<Vec<T>> for DopArray<T>
where
    T: Dop2PayloadExpressible + ToDop2Bytes,
{
    fn into(self) -> Vec<T> {
        self.elements
    }
}

impl<T: Dop2PayloadExpressible + ToDop2Bytes> Dop2PayloadExpressible for DopArray<T> {
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> {
        let count = parser.take_u16()?;
        let mut elements: Vec<T> = Vec::new();
        for x in 0..count {
            let element = T::parse(parser);
            elements.insert(x.into(), *element.unwrap());
        }
        Ok(Box::new(DopArray { count, elements }))
    }
}

impl<T: Dop2PayloadExpressible + ToDop2Bytes> ToDop2Bytes for DopArray<T> {
    fn to_bytes(self, vec: &mut Vec<u8>) {
        let count: u16 = self.elements.len().try_into().unwrap();
        vec.extend(count.to_be_bytes());
        self.elements.into_iter().for_each(|field| field.to_bytes(vec));
    }
}

// Implementations for primitive types

impl Dop2PayloadExpressible for bool {
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> {
        let payload_byte = parser.take_u8()?;
        if payload_byte >= 0x02 {
            return Err("Invalid payload".to_string());
        }
        Ok(Box::new(payload_byte == 0x01))
    }
}

impl ToDop2Bytes for bool {
    fn to_bytes(self, vec: &mut Vec<u8>) {
        if self {
            vec.push(0x01);
        } else {
            vec.push(0x00);
        }
    }
}


impl Dop2PayloadExpressible for String {
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> {
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

impl ToDop2Bytes for String {
    fn to_bytes(self, vec: &mut Vec<u8>) {
        let ascii = self.into_bytes();
        vec.extend(ascii);
    }
}

