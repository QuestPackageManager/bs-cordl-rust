#[cfg(feature = "UnityEngine+ProBuilder+ColliderType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColliderType {
    #[default]
    BoxCollider = 1i32,
    MeshCollider = 2i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+ColliderType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ColliderType =>
    "UnityEngine.ProBuilder"."ColliderType"
);
