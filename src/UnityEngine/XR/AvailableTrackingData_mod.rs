#[cfg(feature = "UnityEngine+XR+AvailableTrackingData")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvailableTrackingData {
    #[default]
    AccelerationAvailable = 16i32,
    AngularAccelerationAvailable = 32i32,
    AngularVelocityAvailable = 8i32,
    None = 0i32,
    PositionAvailable = 1i32,
    RotationAvailable = 2i32,
    VelocityAvailable = 4i32,
}
#[cfg(feature = "UnityEngine+XR+AvailableTrackingData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::AvailableTrackingData =>
    "UnityEngine.XR"."AvailableTrackingData"
);
