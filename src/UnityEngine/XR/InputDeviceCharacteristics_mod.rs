#[cfg(feature = "UnityEngine+XR+InputDeviceCharacteristics")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDeviceCharacteristics {
    Camera = 268960770u32,
    Controller = 2172682304u32,
    EyeTracking = 2151686160u32,
    HandTracking = 1075843080u32,
    HeadMounted = 134480385u32,
    HeldInHand = 537921540u32,
    Left = 8519809u32,
    None = 67240192u32,
    Right = 8650882u32,
    Simulated6DOF = 16777348u32,
    TrackedDevice = 2155888672u32,
    TrackingReference = 8487040u32,
}
#[cfg(feature = "UnityEngine+XR+InputDeviceCharacteristics")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::InputDeviceCharacteristics =>
    "UnityEngine.XR"."InputDeviceCharacteristics"
);
