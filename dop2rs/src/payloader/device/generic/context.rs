use crate::payloader::prelude::*;
use crate::Dop2ParseTreeExpressible;
use crate::payloader::device::generic::state::combined::DeviceCombiState;
use super::request::UserRequestOven;
use super::program_selection::context::PSAttributesCCA;
use super::attributes::DeviceAttributesCCA;

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum ShowMeHowId {
    None = 0,
    ReservedInvalid = 32767,

    CaOvDescaling1 = 51,
    CaOvDescaling2 = 52,
    CaOvDescaling3 = 53,
    CaOvDrawInWater = 54,
    CaOvUseWirelessFoodprobe = 55,
    CaOvUseWiredFoodprobe = 56,
    CaOvRotisserie = 57,
    CaOvAfterPyrolyticCleaning = 58,

    CaSovcUseWirelessFoodprobe = 59,
    CaSovcUseWiredFoodprobe = 60,
    CaSovcFreshWaterFill = 61,
    CaSovcEmptyCondensateTank = 62,
    CaSovcFlushWaterTankAndFill = 63,
    CaSovcDescaling = 64,
    CaSovcFlushFreshWater = 65,
    CaSovcPurgeFreshWaterFill = 66,
    CaSovcPurgeEmptyCondensateTank = 67,

    CaSovmFreshWaterFill = 68,
    CaSovmFlushFreshWaterFill = 69,
    CaSovmDescaling = 70,
    CaSovmFlushFreshWater = 71,
    CaOvmUseWiredFoodprobe = 72,

    CoFillWatertankWithWaterAndDescalingAgent = 73,
    CoPlaceMaintenanceContainerUnderSpout = 74,
    CoEmptyDripTrayWasteContainerCleanContactsAndPlaceBack = 75,
    CoRinseFillInsertWaterContainer = 76,
    CoRemoveWaterContainerBrewUnitRinseBrewUnit = 77,
    CoInsertBrewUnitWithTablet = 78,
    CoFillWatertankWithWaterAndCleaningAgent = 79,
    CoRinseInsertWaterContainer = 80,
    CoUnwrapDescalingCartidgeAndFitAsDescribed = 81,
    CoUnwrapCleaningCartidgeAndFitAsDescribed = 82,
    CoUnwrapCleaningAndDescalingCartidgeAndFitAsDescribed = 83,
    CoFitMilkValveConnectMilkPipework = 84,
    CoNewDescalingCartidgeIsFlooded = 85,

    CaSovcUseWiredFoodprobe2 = 86,
    CaOvRotisserieR36 = 87,
    CaOvRotisserieR48 = 88,
    CaOvReplugWirelessFoodprobe = 89,
    CaSovcReplugWirelessFoodprobe = 90,
    CoInsertAdapter = 91,
    CoRemoveAndCleanMilkValve = 92,
    CaSovcRemoveAccessoriesAndShelfRunners = 93,
    CaSovcDropBroilingElementDownAndRemoveCoarseSoiling = 94,
    CaSovcInsertFilterInTheFloorAndPourCleaningAgent = 95,
    CaSovcRaiseBroilingElementAndRefitShelfRunnersAndAccessories = 96,
    CaOvUseWirelessFoodprobeNa30 = 97,
    CaOvUseWirelessFoodprobeR30R36 = 98,
    CaOvUseWirelessFoodprobeR48 = 99,
}

crate::impl_tryfrom_wrapper!(ShowMeHowId, E16);

#[derive(Debug, Clone, PartialEq, Eq, AssocTypes)]
pub struct DeviceContext {
    #[dop2field(1, Dop2Payloads::MStruct)]
    state: DeviceCombiState,
    #[dop2field(7, Dop2Payloads::MStruct)]
    prog: PSAttributesCCA,
    #[dop2field(8, Dop2Payloads::MStruct)]
    device_attributes: DeviceAttributesCCA,
    #[dop2field(9, Dop2Payloads::ArrayE16)]
    supported_user_requests: Vec<UserRequestOven>,
    #[dop2field(11, Dop2Payloads::Boolean)]
    mobile_start_active: bool,
    #[dop2field(12, Dop2Payloads::E16)]
    show_me_how_id: ShowMeHowId,
    #[dop2field(13, Dop2Payloads::Boolean)]
    request_time_sync: bool,
}

impl DeviceContext {
    pub const ATTRIBUTE_IDS: &[u16] = &[391, 1585];
}

impl_tryfrom_dop2struct!(DeviceContext);

