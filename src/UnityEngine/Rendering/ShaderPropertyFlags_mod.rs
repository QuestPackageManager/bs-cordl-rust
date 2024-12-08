#[cfg(feature = "UnityEngine+Rendering+ShaderPropertyFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderPropertyFlags {
    Gamma = 32i32,
    HDR = 16i32,
    HideInInspector = 1i32,
    MainColor = 256i32,
    MainTexture = 128i32,
    NoScaleOffset = 4i32,
    NonModifiableTextureData = 64i32,
    None = 0i32,
    Normal = 8i32,
    PerRendererData = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+ShaderPropertyFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ShaderPropertyFlags =>
    "UnityEngine.Rendering"."ShaderPropertyFlags"
);
