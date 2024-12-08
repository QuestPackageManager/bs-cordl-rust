#[cfg(feature = "UnityEngine+UIElements+Overflow")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Hidden = 1i32,
    Visible = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+Overflow")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Overflow =>
    "UnityEngine.UIElements"."Overflow"
);
