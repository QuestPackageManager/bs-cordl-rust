#[cfg(feature = "UnityEngine+ProBuilder+EntityType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Collider = 3i32,
    Detail = 0i32,
    Mover = 4i32,
    Occluder = 1i32,
    Trigger = 2i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+EntityType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::EntityType =>
    "UnityEngine.ProBuilder"."EntityType"
);
