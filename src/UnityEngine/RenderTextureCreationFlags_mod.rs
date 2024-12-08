#[cfg(feature = "UnityEngine+RenderTextureCreationFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderTextureCreationFlags {
    AllowVerticalFlip = 128i32,
    AutoGenerateMips = 2i32,
    BindMS = 2048i32,
    CreatedFromScript = 32i32,
    DynamicallyScalable = 1024i32,
    EnableRandomWrite = 16i32,
    EyeTexture = 8i32,
    MipMap = 1i32,
    NoResolvedColorSurface = 256i32,
    SRGB = 4i32,
}
#[cfg(feature = "UnityEngine+RenderTextureCreationFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderTextureCreationFlags =>
    "UnityEngine"."RenderTextureCreationFlags"
);
