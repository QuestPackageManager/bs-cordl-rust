#[cfg(feature = "UnityEngine+UIElements+EventInterestOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventInterestOptions {
    AllEventTypes = -1i32,
    Inherit = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventInterestOptions =>
    "UnityEngine.UIElements"."EventInterestOptions"
);
