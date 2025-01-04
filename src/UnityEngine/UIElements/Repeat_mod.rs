#[cfg(feature = "UnityEngine+UIElements+Repeat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Repeat {
    #[default]
    NoRepeat = 0i32,
    Repeat = 3i32,
    Round = 2i32,
    Space = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+Repeat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Repeat =>
    "UnityEngine.UIElements"."Repeat"
);
