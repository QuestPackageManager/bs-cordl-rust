#[cfg(feature = "UnityEngine+TransparencySortMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransparencySortMode {
    #[default]
    CustomAxis = 3i32,
    Default = 0i32,
    Orthographic = 2i32,
    Perspective = 1i32,
}
#[cfg(feature = "UnityEngine+TransparencySortMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TransparencySortMode =>
    "UnityEngine"."TransparencySortMode"
);
