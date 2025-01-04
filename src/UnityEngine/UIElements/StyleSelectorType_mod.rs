#[cfg(feature = "UnityEngine+UIElements+StyleSelectorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StyleSelectorType {
    #[default]
    Class = 3i32,
    _cordl_ID = 6i32,
    Predicate = 7i32,
    PseudoClass = 4i32,
    RecursivePseudoClass = 5i32,
    Type = 2i32,
    Unknown = 0i32,
    Wildcard = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSelectorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleSelectorType =>
    "UnityEngine.UIElements"."StyleSelectorType"
);
