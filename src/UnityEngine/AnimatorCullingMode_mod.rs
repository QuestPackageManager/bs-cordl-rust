#[cfg(feature = "UnityEngine+AnimatorCullingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatorCullingMode {
    AlwaysAnimate = 0i32,
    CullCompletely = 2i32,
    CullUpdateTransforms = 1i32,
}
#[cfg(feature = "UnityEngine+AnimatorCullingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimatorCullingMode =>
    "UnityEngine"."AnimatorCullingMode"
);
