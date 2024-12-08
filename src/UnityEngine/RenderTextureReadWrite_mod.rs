#[cfg(feature = "UnityEngine+RenderTextureReadWrite")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderTextureReadWrite {
    Default = 0i32,
    Linear = 1i32,
    sRGB = 2i32,
}
#[cfg(feature = "UnityEngine+RenderTextureReadWrite")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderTextureReadWrite =>
    "UnityEngine"."RenderTextureReadWrite"
);
