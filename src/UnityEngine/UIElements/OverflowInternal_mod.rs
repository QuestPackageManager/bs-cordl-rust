#[cfg(feature = "UnityEngine+UIElements+OverflowInternal")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowInternal {
    Hidden = 1i32,
    Scroll = 2i32,
    Visible = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+OverflowInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::OverflowInternal =>
    "UnityEngine.UIElements"."OverflowInternal"
);
