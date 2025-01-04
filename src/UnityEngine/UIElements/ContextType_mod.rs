#[cfg(feature = "UnityEngine+UIElements+ContextType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContextType {
    #[default]
    Editor = 1i32,
    Player = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+ContextType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ContextType =>
    "UnityEngine.UIElements"."ContextType"
);
