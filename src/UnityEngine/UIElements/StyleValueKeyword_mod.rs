#[cfg(feature = "UnityEngine+UIElements+StyleValueKeyword")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StyleValueKeyword {
    #[default]
    Auto = 2i32,
    False = 5i32,
    Inherit = 0i32,
    Initial = 1i32,
    None = 6i32,
    True = 4i32,
    Unset = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueKeyword")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleValueKeyword =>
    "UnityEngine.UIElements"."StyleValueKeyword"
);
