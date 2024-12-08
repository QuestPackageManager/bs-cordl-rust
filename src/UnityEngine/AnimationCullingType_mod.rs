#[cfg(feature = "UnityEngine+AnimationCullingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationCullingType {
    AlwaysAnimate = 0i32,
    BasedOnClipBounds = 2i32,
    BasedOnRenderers = 1i32,
    BasedOnUserBounds = 3i32,
}
#[cfg(feature = "UnityEngine+AnimationCullingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimationCullingType =>
    "UnityEngine"."AnimationCullingType"
);
