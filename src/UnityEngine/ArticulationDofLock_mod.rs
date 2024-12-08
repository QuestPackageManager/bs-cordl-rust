#[cfg(feature = "UnityEngine+ArticulationDofLock")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticulationDofLock {
    FreeMotion = 2i32,
    LimitedMotion = 1i32,
    LockedMotion = 0i32,
}
#[cfg(feature = "UnityEngine+ArticulationDofLock")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ArticulationDofLock =>
    "UnityEngine"."ArticulationDofLock"
);
