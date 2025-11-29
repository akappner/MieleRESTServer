use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

/// Decoder for attribute 1566 (DOP2XKMIdentLabel)
///
/// Mirrors the Python structure in `MieleDop2Structures.py`:
/// - serialNumber
/// - fabricationNumber
/// - technicalType
/// - materialNumber
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct XkmIdentLabel {
    // Serial number as a fixed-length ASCII byte array
    #[dop2field(1, Dop2Payloads::ArrayU8)]
    serial_number: DopArray<u8>,

    // Fabrication number as a fixed-length ASCII byte array
    #[dop2field(2, Dop2Payloads::ArrayU8)]
    fabrication_number: DopArray<u8>,

    // Technical type as a fixed-length ASCII byte array
    #[dop2field(3, Dop2Payloads::ArrayU8)]
    technical_type: DopArray<u8>,

    // Material number as a fixed-length ASCII byte array
    #[dop2field(4, Dop2Payloads::ArrayU8)]
    material_number: DopArray<u8>,
}

impl_tryfrom_dop2struct!(XkmIdentLabel);


