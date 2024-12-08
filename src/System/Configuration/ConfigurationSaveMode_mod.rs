#[cfg(feature = "System+Configuration+ConfigurationSaveMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigurationSaveMode {
    Full = 2i32,
    Minimal = 1i32,
    Modified = 0i32,
}
#[cfg(feature = "System+Configuration+ConfigurationSaveMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Configuration::ConfigurationSaveMode =>
    "System.Configuration"."ConfigurationSaveMode"
);
