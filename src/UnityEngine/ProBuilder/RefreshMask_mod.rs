#[cfg(feature = "UnityEngine+ProBuilder+RefreshMask")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshMask {
    All = 31i32,
    Bounds = 22i32,
    Collisions = 16i32,
    Colors = 2i32,
    Normals = 4i32,
    Tangents = 8i32,
    UV = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+RefreshMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::RefreshMask =>
    "UnityEngine.ProBuilder"."RefreshMask"
);
