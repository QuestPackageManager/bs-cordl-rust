#[cfg(feature = "UnityEngine+ProBuilder+RectSelectMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectSelectMode {
    Complete = 1i32,
    Partial = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+RectSelectMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::RectSelectMode =>
    "UnityEngine.ProBuilder"."RectSelectMode"
);
