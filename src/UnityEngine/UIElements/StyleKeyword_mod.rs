#[cfg(feature = "UnityEngine+UIElements+StyleKeyword")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleKeyword {
    Auto = 2i32,
    Initial = 4i32,
    None = 3i32,
    Null = 1i32,
    Undefined = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleKeyword")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleKeyword =>
    "UnityEngine.UIElements"."StyleKeyword"
);
