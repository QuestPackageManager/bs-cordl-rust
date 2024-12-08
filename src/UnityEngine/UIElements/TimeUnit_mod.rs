#[cfg(feature = "UnityEngine+UIElements+TimeUnit")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Millisecond = 1i32,
    Second = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+TimeUnit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TimeUnit =>
    "UnityEngine.UIElements"."TimeUnit"
);
