#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PoseDataFlags {
    #[default]
    NoData = 0i32,
    Position = 1i32,
    Rotation = 2i32,
}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpatialTracking::PoseDataFlags =>
    "UnityEngine.SpatialTracking"."PoseDataFlags"
);
