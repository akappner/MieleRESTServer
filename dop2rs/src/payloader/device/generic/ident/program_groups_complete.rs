use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;

/// GLOBAL_ProgramGroupsComplete (attribute 1599)
///
/// This payload consists of two arrays of structs:
/// * Field 1: array of smaller range descriptors
/// * Field 2: array with a large bitmap covering all programs
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ProgramGroupsComplete {
    /// Range descriptors (start/end program ids with small bitmaps)
    #[dop2field(1, Dop2Payloads::AStruct)]
    pub low: Vec<ProgramGroupRange>,

    /// Complete bitmap descriptors (usually a single, large bitmap)
    #[dop2field(2, Dop2Payloads::AStruct)]
    pub high: Vec<ProgramGroupRange>,
}

impl_tryfrom_dop2struct!(ProgramGroupsComplete);

/// Describes a contiguous range of program ids and a small bitmap payload.
#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct ProgramGroupRange {
    /// First program id (inclusive)
    #[dop2field(1, Dop2Payloads::U16)]
    pub first_program_id: u16,

    /// Last program id (inclusive)
    #[dop2field(2, Dop2Payloads::U16)]
    pub last_program_id: u16,

    /// Flags or count information for the range
    #[dop2field(3, Dop2Payloads::U32)]
    pub flags: u32,

    /// Bitmap or auxiliary data for the range
    #[dop2field(4, Dop2Payloads::ArrayU32)]
    pub payload: DopArray<u32>,
}

impl_tryfrom_dop2struct!(ProgramGroupRange);

