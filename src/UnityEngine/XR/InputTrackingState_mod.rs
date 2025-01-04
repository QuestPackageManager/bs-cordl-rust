#[cfg(feature = "UnityEngine+XR+InputTrackingState")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputTrackingState {
    #[default]
    Acceleration = 4136976u32,
    All = 2147483711u32,
    AngularAcceleration = 16160u32,
    AngularVelocity = 1059065864u32,
    None = 67240192u32,
    Position = 134480385u32,
    Rotation = 268960770u32,
    Velocity = 537921540u32,
}
#[cfg(feature = "UnityEngine+XR+InputTrackingState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::InputTrackingState =>
    "UnityEngine.XR"."InputTrackingState"
);
