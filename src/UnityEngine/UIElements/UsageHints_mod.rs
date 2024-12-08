#[cfg(feature = "UnityEngine+UIElements+UsageHints")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsageHints {
    DynamicColor = 8i32,
    DynamicTransform = 1i32,
    GroupTransform = 2i32,
    MaskContainer = 4i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+UsageHints")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UsageHints =>
    "UnityEngine.UIElements"."UsageHints"
);
