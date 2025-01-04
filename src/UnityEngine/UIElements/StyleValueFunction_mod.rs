#[cfg(feature = "UnityEngine+UIElements+StyleValueFunction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StyleValueFunction {
    #[default]
    Env = 2i32,
    LinearGradient = 3i32,
    Unknown = 0i32,
    Var = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueFunction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleValueFunction =>
    "UnityEngine.UIElements"."StyleValueFunction"
);
