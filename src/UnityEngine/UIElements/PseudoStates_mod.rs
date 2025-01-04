#[cfg(feature = "UnityEngine+UIElements+PseudoStates")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PseudoStates {
    #[default]
    Active = 1i32,
    Checked = 8i32,
    Disabled = 32i32,
    Focus = 64i32,
    Hover = 2i32,
    Root = 128i32,
}
#[cfg(feature = "UnityEngine+UIElements+PseudoStates")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PseudoStates =>
    "UnityEngine.UIElements"."PseudoStates"
);
