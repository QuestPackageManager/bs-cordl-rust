#[cfg(feature = "OVR+OpenVR+EGamepadTextInputLineMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EGamepadTextInputLineMode {
    k_EGamepadTextInputLineModeMultipleLines = 1i32,
    k_EGamepadTextInputLineModeSingleLine = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EGamepadTextInputLineMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EGamepadTextInputLineMode =>
    "OVR.OpenVR"."EGamepadTextInputLineMode"
);
