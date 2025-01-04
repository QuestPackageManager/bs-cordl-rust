#[cfg(feature = "UnityEngine+UIElements+WhiteSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WhiteSpace {
    #[default]
    NoWrap = 1i32,
    Normal = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+WhiteSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::WhiteSpace =>
    "UnityEngine.UIElements"."WhiteSpace"
);
