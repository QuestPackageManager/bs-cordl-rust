#[cfg(feature = "UnityEngine+UIElements+DisplayStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisplayStyle {
    #[default]
    Flex = 0i32,
    None = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+DisplayStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DisplayStyle =>
    "UnityEngine.UIElements"."DisplayStyle"
);
