#[cfg(feature = "UnityEngine+XR+TrackingSpaceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackingSpaceType {
    #[default]
    RoomScale = 1i32,
    Stationary = 0i32,
}
#[cfg(feature = "UnityEngine+XR+TrackingSpaceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::TrackingSpaceType =>
    "UnityEngine.XR"."TrackingSpaceType"
);
