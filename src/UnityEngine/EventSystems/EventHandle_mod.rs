#[cfg(feature = "UnityEngine+EventSystems+EventHandle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventHandle {
    #[default]
    Unused = 0i32,
    Used = 1i32,
}
#[cfg(feature = "UnityEngine+EventSystems+EventHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::EventHandle =>
    "UnityEngine.EventSystems"."EventHandle"
);
