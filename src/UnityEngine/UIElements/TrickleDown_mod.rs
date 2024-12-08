#[cfg(feature = "UnityEngine+UIElements+TrickleDown")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrickleDown {
    NoTrickleDown = 0i32,
    TrickleDown = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+TrickleDown")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TrickleDown =>
    "UnityEngine.UIElements"."TrickleDown"
);
