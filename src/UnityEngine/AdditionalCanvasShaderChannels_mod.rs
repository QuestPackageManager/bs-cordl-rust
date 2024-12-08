#[cfg(feature = "UnityEngine+AdditionalCanvasShaderChannels")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdditionalCanvasShaderChannels {
    None = 0i32,
    Normal = 8i32,
    Tangent = 16i32,
    TexCoord1 = 1i32,
    TexCoord2 = 2i32,
    TexCoord3 = 4i32,
}
#[cfg(feature = "UnityEngine+AdditionalCanvasShaderChannels")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AdditionalCanvasShaderChannels =>
    "UnityEngine"."AdditionalCanvasShaderChannels"
);
