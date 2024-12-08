#[cfg(feature = "UnityEngine+ArticulationDriveType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticulationDriveType {
    Acceleration = 1i32,
    Force = 0i32,
    Target = 2i32,
    Velocity = 3i32,
}
#[cfg(feature = "UnityEngine+ArticulationDriveType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ArticulationDriveType =>
    "UnityEngine"."ArticulationDriveType"
);
