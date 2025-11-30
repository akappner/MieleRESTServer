
use clap::Parser;

use payloader::unit_ids::UnitIds;
use payloader::device::generic::program_selection::enums::{ProgramIdOven, SelectionType};
use payloader::device::generic::request::UserRequestOven;
use payloader::root::RootNode;


mod crypto;
mod device_api;
mod attribute_registry;
pub use payloader::helper::types::*;

// Re-export Dop types for macro usage
pub use payloader::root::{Dop2Payloads, Dop2PayloadsKind, Dop2Struct, TaggedDopField, Dop2ParseTreeExpressible};
pub use payloader::parser::{DopArray, ToDop2Bytes, Dop2PayloadExpressible, Dop2Parser};

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
        let _request = payloader::device::generic::request::UserRequest {
            request_id: user_request_id,
            parameter0: None,
            parameter1: None,
        };
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
    
    // Use Registry Pattern to handle attribute decoding
    let registry = attribute_registry::AttributeRegistry::new();
    if let Err(e) = registry.handle(root_node.attribute, root_node.root_struct) {
        eprintln!("Warning: {}", e);
    }

    }       
    
}

#[cfg(test)]
mod tests;
