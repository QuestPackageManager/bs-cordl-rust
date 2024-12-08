#[cfg(feature = "OVR+OpenVR+EGamepadTextInputMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EGamepadTextInputMode {
    k_EGamepadTextInputModeNormal = 0i32,
    k_EGamepadTextInputModePassword = 1i32,
    k_EGamepadTextInputModeSubmit = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EGamepadTextInputMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EGamepadTextInputMode =>
    "OVR.OpenVR"."EGamepadTextInputMode"
);
