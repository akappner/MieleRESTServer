
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
#[command(about = "Decode and encode DOP2 payloads.\n\n\
    DECODE: pass a hex string, or pipe raw binary data via stdin.\n\
    ENCODE: pass a command name (e.g. an XKM request or program ID) to produce a hex-encoded DOP2 payload.")]
struct Args {
    /// Hex string to decode, or command name to encode. If omitted, reads raw binary from stdin.
    hex_string: Option<String>,
}

mod payloader;
#[macro_use]
pub mod macros;


use crate::payloader::comm_module::request::request::{XkmRequestId, XkmRequest};
use strum::IntoEnumIterator;
use std::io::Read;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let command_verbs_xkm = payloader::comm_module::request::request::XkmRequestId::iter().map(|x| x.to_string());
    let command_verbs_program = ProgramIdOven::iter().map(|x| x.to_string());

    let command = args.hex_string.as_deref().unwrap_or("");

    if let Ok(xkm) = XkmRequestId::from_str(&command)
    {
        eprintln!("Sending XKM command {:?}", xkm);
        let request = XkmRequest{request_id: xkm};
        let payload = request.to_dop2_struct_auto().map_err(|e| format!("Failed to encode XKM request: {e}"))?;

        let attr = XkmRequest::ATTRIBUTE_IDS.first().ok_or("No attribute ID defined for XkmRequest")?.clone();
        let root = RootNode::single(UnitIds::CommunicationsModule.into(), attr, payload);

        let mut data : Vec<u8> = vec!();
        root.to_bytes(&mut data);
        println!("{}", hex::encode(data));
    }
    else if let Ok(program_id) = ProgramIdOven::from_str(&command)
    {
        eprintln!("Sending PS command {:?}", program_id);
        let request = payloader::device::generic::program_selection::select::PsSelect { program_id, selection_parameter: 0, selection_type: SelectionType::InitialDefault };
        let payload = request.to_dop2_struct_auto().map_err(|e| format!("Failed to encode PS request: {e}"))?;

        let attr = payloader::device::generic::program_selection::select::PsSelect::ATTRIBUTE_IDS.first().ok_or("No attribute ID defined for PsSelect")?.clone();
        let root = RootNode::single(UnitIds::MainDevice.into(), attr, payload);

        let mut data : Vec<u8> = vec!();
        root.to_bytes(&mut data);
        println!("{}", hex::encode(data));
    }
    else if let Ok(user_request_id) = UserRequestOven::from_str(&command)
    {
        eprintln!("Sending UserRequest command {:?}", user_request_id);
        let _request = payloader::device::generic::request::UserRequest {
            request_id: user_request_id,
            parameter0: None,
            parameter1: None,
        };
    }
    else {
        let bytes = match &args.hex_string {
            Some(hex_str) => {
                match hex::decode(hex_str) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        println!("Available commands are:");
                        println!("*** Program Selection: {:?}\n", command_verbs_program.collect::<Vec<String>>());
                        println!("*** Communications Module: {:?}\n", command_verbs_xkm.collect::<Vec<String>>());
                        return Err(format!("Error decoding hex string: {e}").into());
                    }
                }
            }
            None => {
                use std::io::IsTerminal;
                if std::io::stdin().is_terminal() {
                    println!("Available commands are:\n");
                    println!("*** Program Selection: {:?}\n", command_verbs_program.collect::<Vec<_>>());
                    println!("*** Communications Module: {:?}\n", command_verbs_xkm.collect::<Vec<_>>());
                    return Err("No hex string provided and stdin is a terminal".into());
                }
                let mut buf = Vec::new();
                std::io::stdin().read_to_end(&mut buf)?;
                buf
            }
        };

        let mut parser = Dop2Parser::new(bytes);
        let root_node = RootNode::parse(&mut parser).map_err(|e| format!("Failed to parse DOP2 payload: {e}"))?;
        println!("{root_node:#?}");

        let registry = attribute_registry::AttributeRegistry::new();
        if let Err(e) = registry.handle(root_node.attribute, root_node.root_struct) {
            eprintln!("Warning: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
