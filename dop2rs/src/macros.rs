
#[macro_export]
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
    #[macro_export]
    macro_rules! impl_to_bytes {
        ($t:ty) => {
            impl ToDop2Bytes for $t {
                fn to_bytes(self, vec: &mut Vec<u8>) {
                    vec.extend(self.to_be_bytes());
                }
            }
        };
    }

    #[macro_export]
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

    #[macro_export]
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
    
#[macro_export]
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


    #[macro_export]
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


    #[macro_export]
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
    