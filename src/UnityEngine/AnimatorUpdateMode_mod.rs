#[cfg(feature = "UnityEngine+AnimatorUpdateMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimatorUpdateMode {
    #[default]
    AnimatePhysics = 1i32,
    Normal = 0i32,
    UnscaledTime = 2i32,
}
#[cfg(feature = "UnityEngine+AnimatorUpdateMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimatorUpdateMode => "UnityEngine"
    ."AnimatorUpdateMode"
);
