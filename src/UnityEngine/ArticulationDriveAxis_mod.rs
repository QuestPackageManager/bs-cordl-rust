#[cfg(feature = "UnityEngine+ArticulationDriveAxis")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticulationDriveAxis {
    X = 0i32,
    Y = 1i32,
    Z = 2i32,
}
#[cfg(feature = "UnityEngine+ArticulationDriveAxis")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ArticulationDriveAxis =>
    "UnityEngine"."ArticulationDriveAxis"
);
