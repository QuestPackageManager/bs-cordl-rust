#[cfg(feature = "UnityEngine+Events+PersistentListenerMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistentListenerMode {
    Bool = 6i32,
    EventDefined = 0i32,
    Float = 4i32,
    Int = 3i32,
    Object = 2i32,
    String = 5i32,
    Void = 1i32,
}
#[cfg(feature = "UnityEngine+Events+PersistentListenerMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::PersistentListenerMode =>
    "UnityEngine.Events"."PersistentListenerMode"
);
