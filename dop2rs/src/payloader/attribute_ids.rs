// Macro to register ATTRIBUTE_IDS for all root node types
// This centralizes the definition of attribute IDs for all payloader types
// It also generates handler registration code

// Trait for types that have ATTRIBUTE_IDS
pub trait HasAttributeIds {
    const ATTRIBUTE_IDS: &'static [u16];
}

#[macro_export]
macro_rules! register_root_nodes {
    // Define ATTRIBUTE_IDS for types and generate handler registration function
    ($($type:ty => $ids:expr),* $(,)?) => {
        // Generate ATTRIBUTE_IDS definitions
        $(
            impl $type {
                pub const ATTRIBUTE_IDS: &[u16] = $ids;
            }
            impl HasAttributeIds for $type {
                const ATTRIBUTE_IDS: &'static [u16] = $ids;
            }
        )*
        
        // Generate a function that registers all handlers
        pub fn register_all_handlers(registry: &mut crate::attribute_registry::AttributeRegistry) {
            $(
                registry.register_handler::<$type>();
            )*
        }
    };
}

// Register all root nodes with their attribute IDs
register_root_nodes! {
    // Device context types
    crate::payloader::device::generic::context::DeviceContext => &[391, 1585],
    
    // Oven types
    crate::payloader::device::oven::program_info::ProgramInfoOven => &[213],
    crate::payloader::device::oven::program_step_info::ProgramStepInfoOven => &[214],
    crate::payloader::device::generic::ident::program_groups_complete::ProgramGroupsComplete => &[1599],
    
    // Washer types
    crate::payloader::device::washer::process::Process => &[6195],
    crate::payloader::device::washer::actuator::ActuatorData => &[6192],
    crate::payloader::device::washer::sensor::Sensor => &[6193],
    
    // Device generic types
    crate::payloader::device::generic::failure::FailureList => &[148],
    crate::payloader::device::generic::failure::Failure => &[117],
    crate::payloader::device::generic::request::UserRequest => &[1583],
    crate::payloader::device::generic::ident::ident::DeviceIdent => &[144],
    crate::payloader::device::generic::ident::feature_list::FeatureList => &[348],
    crate::payloader::device::generic::settings::SfValueList => &[114],
    crate::payloader::device::generic::notifications::DeviceNotifications => &[131, 392],
    crate::payloader::device::generic::program_selection::context::PSContext => &[1574],
    crate::payloader::device::generic::program_selection::select::PsSelect => &[1577],
    crate::payloader::device::generic::cets::CookingEndTimeSynchronizationStatus => &[412],
    
    // Device state types
    crate::payloader::device::generic::state::combined::DeviceCombiState => &[1586],
    crate::payloader::device::generic::state::cs_context::CSContext => &[154],
    crate::payloader::device::generic::state::cs_context::CSBarcode => &[174],
    crate::payloader::device::generic::state::hours::CSHoursOfOperation => &[119],
    
    // Communication module types
    crate::payloader::comm_module::state::datetime::DateTimeInfo => &[122],
    crate::payloader::comm_module::state::state::XkmStateInfo => &[1568],
    crate::payloader::comm_module::request::request::XkmRequest => &[130],
    crate::payloader::comm_module::update::control::UpdateControl => &[170],
    crate::payloader::comm_module::config::ip::XkmConfigIp => &[1573],
    crate::payloader::comm_module::config::ssid::XkmConfigSsidList => &[110],
    crate::payloader::comm_module::ident::ident::XkmIdent => &[1565],
    crate::payloader::comm_module::ident::ident_label::XkmIdentLabel => &[1566],
    crate::payloader::comm_module::supervision::config::SuperVisionListConfig => &[1570],
    crate::payloader::comm_module::supervision::config::SuperVisionListItem => &[1571],
    
    // Filesystem types
    crate::payloader::filesystem::file_list::FileList => &[333],
    crate::payloader::filesystem::file_info::FileInfo => &[1588],
    crate::payloader::filesystem::file_write::FileWrite => &[1590],
    crate::payloader::filesystem::transfer::FileTransfer => &[336],
    crate::payloader::filesystem::rsa_key::RsaKey => &[287],
    
    // Meta types
    crate::payloader::meta::object_ids::SysObjectId => &[19],
    crate::payloader::meta::object_ids::SoftwareIds => &[17],
}

