#[cfg(feature = "UnityEngine+UIElements+Wrap")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wrap {
    NoWrap = 0i32,
    Wrap = 1i32,
    WrapReverse = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+Wrap")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Wrap =>
    "UnityEngine.UIElements"."Wrap"
);
