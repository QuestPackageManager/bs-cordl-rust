#[cfg(feature = "UnityEngine+UIElements+ScrollerVisibility")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollerVisibility {
    AlwaysVisible = 1i32,
    Auto = 0i32,
    Hidden = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+ScrollerVisibility")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ScrollerVisibility =>
    "UnityEngine.UIElements"."ScrollerVisibility"
);
