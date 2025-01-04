#[cfg(feature = "UnityEngine+PenEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PenEventType {
    #[default]
    NoContact = 0i32,
    PenDown = 1i32,
    PenUp = 2i32,
}
#[cfg(feature = "UnityEngine+PenEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PenEventType => "UnityEngine"
    ."PenEventType"
);
