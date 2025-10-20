use clap::Parser;
use num_enum::TryFromPrimitive;

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
    StringU8                    = 18,
    ArrayE8                     = 20,
    ArrayU16                    = 21,
    ArrayI16                    = 22,
    ArrayE16                    = 23,
    ArrayI32                    = 25,
    ArrayU64                    = 27,
    MString                     = 32,  
    AStruct                     = 33,
}


#[derive(Debug)]
struct RootNode {
    unit: u16,
    attribute: u16,
    declared_length: u16,
    declared_fields: u16,
    fields: Vec<TaggedDopField>,
    padding: DopPadding
}
#[derive(Debug)]
struct TaggedDopField {
    tag: Dop2Type,
    fieldIndex : u16,
    value: Dop2Payloads,
}
#[derive(Debug)]
struct DopArray <T : Dop2PayloadExpressible>
{
    count : u16,
    elements : Vec<T>
}

impl<T : Dop2PayloadExpressible> Dop2PayloadExpressible for DopArray<T>
{
     fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> 
    {
        let count = parser.take_u16().unwrap();
        let mut elements : Vec<T> = Vec::new();
        for x in 0..count
        {
//            println!("Array element {} of {}, datatype {}", x, count);
            let element = T::parse(parser);
            elements.insert(x.into(), *element.unwrap());
        }
        Ok(Box::new(DopArray{count, elements}))
    }
}
#[derive(Debug)]
enum Dop2Payloads {
    boolean(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(u8),
    I16(u16),
    I32(i32),
    I64(u64),

    E8(u8),
    E16(u16),
    E32(u32),
    E64(u64),

    MString(String),
    StringU8 (String),

    ArrayU8 (DopArray<u8>),
    ArrayU16 (DopArray<u16>),
    ArrayU32 (DopArray<u32>),
    ArrayU64 (DopArray<u64>),

//    ArrayI8 (DopArray<u8>),
    ArrayI16 (DopArray<u16>),
    ArrayI32 (DopArray<i32>),
//    ArrayI64 (DopArray<u64>),

    ArrayE8 (DopArray<u8>),
    ArrayE16 (DopArray<u16>),

    MStruct (Dop2Struct),
    AStruct (DopArray<Dop2Struct>)
//    ArrayE32(DopArray<u32>),
//    ArrayE64(DopArray<u64>),
}
        
#[derive(Debug)]
struct DopPadding {
    bytes_of_padding : u8
}
impl DopPadding
{
    fn parse(parser: &mut Dop2Parser) -> Result<DopPadding, String>
    {
    let mut bytes_of_padding = 0u8;
    while !parser.is_empty() {
        let byte = parser.take_u8().map_err(|e| e.to_string())?;
        if byte == 0x20 {
            bytes_of_padding += 1;
        } else {
            // If we read a non-0x20 byte, backtrack and stop
            return Err(format!("Non-padding byte 0x{:02X} read", byte))
        }
    }
    Ok(DopPadding { bytes_of_padding })
    }
}
trait Dop2PayloadExpressible {
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> ;
}
impl Dop2PayloadExpressible for bool
{
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> 
    {
        let payloadByte = parser.take_u8().unwrap();
        if (payloadByte >= 0x02)
        {
            return Err("Invalid payload".to_string())
        }
        Ok(Box::new(payloadByte==0x01))
    }
}

impl Dop2PayloadExpressible for String
{
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> 
    {
        let length = parser.take_u16().unwrap();
        let mut result = String::new();
        let stringBytes = parser.take(length.into());
        for b in stringBytes.unwrap().into_iter() {
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

impl_from_bytes!(i32);

impl TaggedDopField {
    fn parse(parser: &mut Dop2Parser) -> Result<TaggedDopField, String> {
        let fieldIndex = parser.take_u16().unwrap();
        let tag_byte = parser.take_u8()?;
        let tag = Dop2Type::try_from_primitive(tag_byte)
            .map_err(|_| format!("Invalid Dop2Type value: 0x{:02X}", tag_byte))?;
       // let value = ImmediateDopField {value: parser.take_u8()?};
        // INSERT_YOUR_CODE
        let value = match tag {
            Dop2Type::Bool => {
                Dop2Payloads::boolean(*bool::parse(parser).unwrap())
            },
            Dop2Type::E8 =>
            {
                Dop2Payloads::U8 (*u8::parse(parser).unwrap())
            },
            Dop2Type::U8 =>
            {
                Dop2Payloads::U8 (*u8::parse(parser).unwrap())
            },
            Dop2Type::U16 =>
            {
                Dop2Payloads::U16 (*u16::parse(parser).unwrap())
            },
            Dop2Type::U64 =>
            {
                Dop2Payloads::U64 (*u64::parse(parser).unwrap())
            }
            Dop2Type::I16 =>
            {
                Dop2Payloads::I16 (*u16::parse(parser).unwrap())
            }
            Dop2Type::E16 =>
            {
                Dop2Payloads::E16 (*u16::parse(parser).unwrap())
            }
            Dop2Type::U32 =>
            {
                Dop2Payloads::U32 (*u32::parse(parser).unwrap())
            }

            Dop2Type::I32 =>
            {
                Dop2Payloads::I32 (*i32::parse(parser).unwrap())
            }
            Dop2Type::I64 =>
            {
                Dop2Payloads::I64 (*u64::parse(parser).unwrap())
            }
            Dop2Type::MString =>
            {
                Dop2Payloads::MString (*String::parse(parser).unwrap())
            }
            Dop2Type::StringU8 =>
            {
                Dop2Payloads::StringU8 (*String::parse(parser).unwrap())
            }

            Dop2Type::ArrayI16 =>
            {
                Dop2Payloads::ArrayI16 (*DopArray::parse(parser).unwrap())
            }
            Dop2Type::ArrayI32 =>
            {
                Dop2Payloads::ArrayI32 (*DopArray::parse(parser).unwrap())
            }
                Dop2Type::ArrayE8 =>
            {
                Dop2Payloads::ArrayE8 (*DopArray::parse(parser).unwrap())
            }
            Dop2Type::ArrayE16 =>
            {
                Dop2Payloads::ArrayE16 (*DopArray::parse(parser).unwrap())
            }

            Dop2Type::ArrayU64 =>
            {
                Dop2Payloads::ArrayU64 (*DopArray::parse(parser).unwrap())
            }
            Dop2Type::Struct =>
            {
                Dop2Payloads::MStruct (*Dop2Struct::parse(parser).unwrap())
            }
              Dop2Type::AStruct =>
            {
                Dop2Payloads::AStruct (*DopArray::parse(parser).unwrap())
            }
            garbage => 
            {
                println!("unknown type {:?}", garbage);
                todo!()
            }
        };
        
        Ok(TaggedDopField { tag, fieldIndex, value })
    }
}

#[derive(Debug)]
struct Dop2Struct{
    declared_fields: u16,
    fields: Vec<TaggedDopField>,
}

impl Dop2PayloadExpressible for Dop2Struct
{
    fn parse(parser: &mut Dop2Parser) -> Result<Box<Self>, String> {
        let declared_fields = parser.take_u16()?;
        let mut fields = Vec::new();
        println!("Parsing fields");
        for x in 1..declared_fields+1 {
            println!("Parsing field {} of {}", x, declared_fields);
            let tagged_field = TaggedDopField::parse(parser)?;
            fields.push(tagged_field);
        }
        Ok(Box::new(Dop2Struct {declared_fields, fields}))
    }
}
impl RootNode {
    fn parse(parser: &mut Dop2Parser) -> Result<RootNode, String> {
     


        let declared_length = parser.take_u16().unwrap();
        let unit = parser.take_u16()?;
        let attribute = parser.take_u16()?; 
        
        let _ = parser.take(4); // skip the length field
        let declared_fields = parser.take_u16()?;
        let mut fields = Vec::new();
        println!("Parsing fields");
        for x in 1..declared_fields+1 {
            println!("Parsing field {} of {}", x, declared_fields);
            let tagged_field = TaggedDopField::parse(parser)?;
            fields.push(tagged_field);
        }
        let padding = DopPadding::parse(parser).unwrap();
        assert!(parser.is_empty());
        Ok(RootNode { unit, attribute, declared_fields, declared_length, fields, padding })
    }
}


struct Dop2Parser {
    payload : Vec<u8>
}
impl Dop2Parser {
    fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
    /// Strips off `n` bytes from the beginning of the payload and returns them as a vector.
    /// Returns an error if there are not enough bytes.
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
    /// Returns true if the parser has no remaining payload
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
    
    println!("Hex string: {}", args.hex_string);
   // println!("Unit: {:?}", args.unit);
    //println!("Attribute: {:?}", args.attribute);

    let mut parser = Dop2Parser::new(bytes);
    let root_node = RootNode::parse(&mut parser).unwrap();
    println!("{root_node:#?}");
}

#[cfg(test)]
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
    
    #[test]
    fn test_dop2_parser_take() {
        let mut parser = Dop2Parser::new(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        
        // Test taking 2 bytes
        let bytes = parser.take(2).unwrap();
        assert_eq!(bytes, vec![0xDE, 0xAD]);
        
        // Test taking remaining bytes
        let bytes = parser.take(2).unwrap();
        assert_eq!(bytes, vec![0xBE, 0xEF]);
        
        // Test taking more bytes than available
        let result = parser.take(1);
        assert!(result.is_err());
    }

    #[test]
    fn test_dop2_parser_take_u16() {
        let mut parser = Dop2Parser::new(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        
        // Test big-endian u16 parsing
        let value1 = parser.take_u16().unwrap();
        assert_eq!(value1, 0xDEAD);
        
        let value2 = parser.take_u16().unwrap();
        assert_eq!(value2, 0xBEEF);
        
        // Test taking more than available
        let result = parser.take_u16();
        assert!(result.is_err());
    }


    #[test]
    fn test_root_node_parse_insufficient_data() {
        // Test with insufficient data (only 2 bytes, need at least 4)
        let test_data = vec![0x12, 0x34];
        
        let mut parser = Dop2Parser::new(test_data);
        let result = RootNode::parse(&mut parser);
        assert!(result.is_err());
    }

    #[test]
    fn test_device_combo_state() {
        // Test that we can use it with our parser
        let mut parser = Dop2Parser::new(hex::decode(TEST_PAYLOADS.oven_2_1586).unwrap());
        let result = RootNode::parse(&mut parser);
        assert!(result.is_ok());
        let root_node = result.unwrap();
        assert_eq!(root_node.unit, 2);
        assert_eq!(root_node.attribute, 1586);
        assert_eq!(root_node.declared_fields, 3);
      //  assert_eq!(root_node.fields[0].value, 0x0082);
    }
    #[test]
    fn test_static_payloads() {
        // Test that we can use it with our parser
        let mut parser = Dop2Parser::new(hex::decode(TEST_PAYLOADS.oven_14_130).unwrap());
        let result = RootNode::parse(&mut parser);
        assert!(result.is_ok());
        let root_node = result.unwrap();
        assert_eq!(root_node.unit, 14);
        assert_eq!(root_node.attribute, 130);
        assert_eq!(root_node.declared_fields, 1);
      //  assert_eq!(root_node.fields[0].value, 0x0082);
    }
}
