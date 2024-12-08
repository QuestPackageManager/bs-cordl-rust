#[cfg(feature = "UnityEngine+UIElements+Position")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Absolute = 1i32,
    Relative = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+Position")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Position =>
    "UnityEngine.UIElements"."Position"
);
