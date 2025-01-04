#[cfg(feature = "OVR+OpenVR+EVRSettingsError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRSettingsError {
    #[default]
    IPCFailed = 1i32,
    JsonParseFailed = 4i32,
    None = 0i32,
    ReadFailed = 3i32,
    UnsetSettingHasNoDefault = 5i32,
    WriteFailed = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EVRSettingsError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRSettingsError => "OVR.OpenVR"
    ."EVRSettingsError"
);
