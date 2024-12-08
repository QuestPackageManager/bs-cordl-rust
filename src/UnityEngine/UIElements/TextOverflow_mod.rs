#[cfg(feature = "UnityEngine+UIElements+TextOverflow")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextOverflow {
    Clip = 0i32,
    Ellipsis = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+TextOverflow")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TextOverflow =>
    "UnityEngine.UIElements"."TextOverflow"
);
