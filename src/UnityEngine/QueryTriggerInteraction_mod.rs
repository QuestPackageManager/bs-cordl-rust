#[cfg(feature = "UnityEngine+QueryTriggerInteraction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QueryTriggerInteraction {
    #[default]
    Collide = 2i32,
    Ignore = 1i32,
    UseGlobal = 0i32,
}
#[cfg(feature = "UnityEngine+QueryTriggerInteraction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::QueryTriggerInteraction =>
    "UnityEngine"."QueryTriggerInteraction"
);
