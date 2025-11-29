// Registry Pattern implementation to replace long if/else chains
// This uses a HashMap-based registry to map attribute IDs to handler functions

use std::collections::HashMap;
use crate::payloader::root::{Dop2Payloads, Dop2Struct, Dop2ParseTreeExpressible};

type AttributeHandler = fn(Dop2Struct) -> Result<(), String>;

pub struct AttributeRegistry {
    handlers: HashMap<u16, AttributeHandler>,
}

// Macro to register a type with its attribute IDs
macro_rules! register_handler {
    ($registry:expr, $type:ty) => {
        for &attr_id in <$type>::ATTRIBUTE_IDS {
            $registry.handlers.insert(attr_id, |struct_data| {
                let decoded = <$type>::from_parse_tree(Dop2Payloads::MStruct(struct_data))?;
                println!("{decoded:#?}");
                Ok(())
            });
        }
    };
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

    fn register_all(&mut self) {
        // Device context types
        register_handler!(
            self,
            crate::payloader::device::generic::context::DeviceContext
        );
        
        // Oven types
        register_handler!(
            self,
            crate::payloader::device::oven::program_info::ProgramInfoOven
        );
        register_handler!(
            self,
            crate::payloader::device::oven::program_step_info::ProgramStepInfoOven
        );
        
        // Device identification
        register_handler!(
            self,
            crate::payloader::device::generic::ident::ident::DeviceIdent
        );
        
        // Communication module
        register_handler!(
            self,
            crate::payloader::comm_module::state::datetime::DateTimeInfo
        );
        
        // Filesystem types
        register_handler!(
            self,
            crate::payloader::filesystem::file_list::FileList
        );
        register_handler!(
            self,
            crate::payloader::filesystem::file_info::FileInfo
        );
        register_handler!(
            self,
            crate::payloader::filesystem::file_write::FileWrite
        );
        register_handler!(
            self,
            crate::payloader::filesystem::transfer::FileTransfer
        );
        register_handler!(
            self,
            crate::payloader::filesystem::rsa_key::RsaKey
        );
        
        // Device generic types
        register_handler!(
            self,
            crate::payloader::device::generic::failure::FailureList
        );
        register_handler!(
            self,
            crate::payloader::device::generic::request::UserRequest
        );
        
        // Communication module requests
        register_handler!(
            self,
            crate::payloader::comm_module::request::request::XkmRequest
        );
        register_handler!(
            self,
            crate::payloader::comm_module::config::ip::XkmConfigIp
        );
        register_handler!(
            self,
            crate::payloader::comm_module::config::ssid::XkmConfigSsidList
        );
        
        // Device state types
        register_handler!(
            self,
            crate::payloader::device::generic::state::combined::DeviceCombiState
        );
        register_handler!(
            self,
            crate::payloader::device::generic::settings::SfValueList
        );
        register_handler!(
            self,
            crate::payloader::device::generic::program_selection::context::PSContext
        );
        register_handler!(
            self,
            crate::payloader::device::generic::state::cs_context::CSContext
        );
        register_handler!(
            self,
            crate::payloader::device::generic::failure::Failure
        );
        register_handler!(
            self,
            crate::payloader::device::generic::state::hours::CSHoursOfOperation
        );
        register_handler!(
            self,
            crate::payloader::device::generic::ident::feature_list::FeatureList
        );
        register_handler!(
            self,
            crate::payloader::device::generic::notifications::DeviceNotifications
        );
        
        // Washer types
        register_handler!(
            self,
            crate::payloader::device::washer::process::Process
        );
        register_handler!(
            self,
            crate::payloader::device::washer::actuator::ActuatorData
        );
        register_handler!(
            self,
            crate::payloader::device::washer::sensor::Sensor
        );
    }
}
