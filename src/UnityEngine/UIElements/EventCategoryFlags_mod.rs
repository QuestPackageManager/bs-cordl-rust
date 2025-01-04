#[cfg(feature = "UnityEngine+UIElements+EventCategoryFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventCategoryFlags {
    #[default]
    All = -1i32,
    None = 0i32,
    TargetOnly = 2768i32,
    TriggeredByOS = 81974i32,
}
#[cfg(feature = "UnityEngine+UIElements+EventCategoryFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventCategoryFlags =>
    "UnityEngine.UIElements"."EventCategoryFlags"
);
