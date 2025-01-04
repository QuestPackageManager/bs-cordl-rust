#[cfg(feature = "UnityEngine+VRTextureUsage")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VRTextureUsage {
    #[default]
    DeviceSpecific = 3i32,
    None = 0i32,
    OneEye = 1i32,
    TwoEyes = 2i32,
}
#[cfg(feature = "UnityEngine+VRTextureUsage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::VRTextureUsage => "UnityEngine"
    ."VRTextureUsage"
);
