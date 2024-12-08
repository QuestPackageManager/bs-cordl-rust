#[cfg(feature = "UnityEngine+RigidbodyConstraints")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RigidbodyConstraints {
    FreezeAll = 126i32,
    FreezePosition = 14i32,
    FreezePositionX = 2i32,
    FreezePositionY = 4i32,
    FreezePositionZ = 8i32,
    FreezeRotation = 112i32,
    FreezeRotationX = 16i32,
    FreezeRotationY = 32i32,
    FreezeRotationZ = 64i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+RigidbodyConstraints")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RigidbodyConstraints =>
    "UnityEngine"."RigidbodyConstraints"
);
