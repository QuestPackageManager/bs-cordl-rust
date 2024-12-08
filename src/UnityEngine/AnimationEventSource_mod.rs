#[cfg(feature = "UnityEngine+AnimationEventSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationEventSource {
    Animator = 2i32,
    Legacy = 1i32,
    NoSource = 0i32,
}
#[cfg(feature = "UnityEngine+AnimationEventSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimationEventSource =>
    "UnityEngine"."AnimationEventSource"
);
