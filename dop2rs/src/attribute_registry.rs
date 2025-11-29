// Registry Pattern implementation to replace long if/else chains
// This uses a HashMap-based registry to map attribute IDs to handler functions

use std::collections::HashMap;
use crate::payloader::root::Dop2Struct;

type AttributeHandler = fn(Dop2Struct) -> Result<(), String>;

pub struct AttributeRegistry {
    handlers: HashMap<u16, AttributeHandler>,
}

impl AttributeRegistry {
    pub fn new() -> Self {
        let mut registry = AttributeRegistry {
            handlers: HashMap::new(),
        };
        registry.register_all();
        registry
    }

    pub fn handle(&self, attribute_id: u16, struct_data: Dop2Struct) -> Result<(), String> {
        if let Some(handler) = self.handlers.get(&attribute_id) {
            handler(struct_data)
        } else {
            Err(format!("No handler registered for attribute ID: {}", attribute_id))
        }
    }

    // Public method to register a handler for a type
    pub fn register_handler<T>(&mut self) 
    where
        T: crate::payloader::root::Dop2ParseTreeExpressible + std::fmt::Debug + crate::payloader::attribute_ids::HasAttributeIds,
    {
        for &attr_id in T::ATTRIBUTE_IDS {
            self.handlers.insert(attr_id, |struct_data| {
                use crate::payloader::root::{Dop2Payloads, Dop2ParseTreeExpressible};
                let decoded = T::from_parse_tree(Dop2Payloads::MStruct(struct_data))?;
                println!("{decoded:#?}");
                Ok(())
            });
        }
    }

    fn register_all(&mut self) {
        crate::payloader::attribute_ids::register_all_handlers(self);
    }
}
