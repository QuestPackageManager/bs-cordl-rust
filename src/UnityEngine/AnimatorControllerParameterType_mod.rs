#[cfg(feature = "UnityEngine+AnimatorControllerParameterType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatorControllerParameterType {
    Bool = 4i32,
    Float = 1i32,
    Int = 3i32,
    Trigger = 9i32,
}
#[cfg(feature = "UnityEngine+AnimatorControllerParameterType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimatorControllerParameterType =>
    "UnityEngine"."AnimatorControllerParameterType"
);
