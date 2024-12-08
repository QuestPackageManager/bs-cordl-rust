#[cfg(feature = "UnityEngine+AnimatorRecorderMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatorRecorderMode {
    Offline = 0i32,
    Playback = 1i32,
    Record = 2i32,
}
#[cfg(feature = "UnityEngine+AnimatorRecorderMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimatorRecorderMode =>
    "UnityEngine"."AnimatorRecorderMode"
);
