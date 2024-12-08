#[cfg(feature = "UnityEngine+AnimationPlayMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlayMode {
    Mix = 2i32,
    Queue = 1i32,
    Stop = 0i32,
}
#[cfg(feature = "UnityEngine+AnimationPlayMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimationPlayMode => "UnityEngine"
    ."AnimationPlayMode"
);
