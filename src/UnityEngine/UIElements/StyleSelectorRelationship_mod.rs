#[cfg(feature = "UnityEngine+UIElements+StyleSelectorRelationship")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleSelectorRelationship {
    Child = 1i32,
    Descendent = 2i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSelectorRelationship")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSelectorRelationship => "UnityEngine.UIElements"
    ."StyleSelectorRelationship"
);
