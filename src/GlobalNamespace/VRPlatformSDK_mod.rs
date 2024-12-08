#[cfg(feature = "VRPlatformSDK")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VRPlatformSDK {
    Oculus = 1i32,
    OpenXR = 0i32,
    Unknown = 2i32,
}
#[cfg(feature = "VRPlatformSDK")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for VRPlatformSDK => ""."VRPlatformSDK"
);
