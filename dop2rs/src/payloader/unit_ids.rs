use num_enum::{TryFromPrimitive, IntoPrimitive};
use strum_macros::{EnumIter, EnumString, Display};

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, EnumIter, EnumString, Display, IntoPrimitive)]
pub enum UnitIds {
    ProgrammingMaster = 1, // appears to be the unit responsible for front panel, notifications, programming
    MainDevice = 2,
    UnknownThree = 3, // seen in oven
   // SecondDevice = 5, // seen in oven code but never in real device
    UnknownEight = 8, // seen in oven
    UnknownNine = 9, // seen in oven
    UnknownTwelve = 12, // seen in oven
    CommunicationsModule = 14,
    Update = 15,
}

