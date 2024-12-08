#[cfg(feature = "UnityEngine+ProBuilder+MeshArrays")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshArrays {
    All = 255i32,
    Color = 32i32,
    Lightmap = 4i32,
    Normal = 64i32,
    Position = 1i32,
    Tangent = 128i32,
    Texture0 = 2i32,
    Texture2 = 8i32,
    Texture3 = 16i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshArrays")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MeshArrays =>
    "UnityEngine.ProBuilder"."MeshArrays"
);
