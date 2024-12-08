#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasFilters")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicAtlasFilters {
    ColorSpace = 8i32,
    FilterMode = 16i32,
    Format = 4i32,
    None = 0i32,
    Readability = 1i32,
    Size = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasFilters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DynamicAtlasFilters =>
    "UnityEngine.UIElements"."DynamicAtlasFilters"
);
