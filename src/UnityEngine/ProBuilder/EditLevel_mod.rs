#[cfg(feature = "UnityEngine+ProBuilder+EditLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditLevel {
    Geometry = 1i32,
    Plugin = 3i32,
    Texture = 2i32,
    Top = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+EditLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::EditLevel =>
    "UnityEngine.ProBuilder"."EditLevel"
);
