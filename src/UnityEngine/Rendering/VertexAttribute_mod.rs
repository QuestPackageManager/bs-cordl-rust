#[cfg(feature = "UnityEngine+Rendering+VertexAttribute")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexAttribute {
    BlendIndices = 13i32,
    BlendWeight = 12i32,
    Color = 3i32,
    Normal = 1i32,
    Position = 0i32,
    Tangent = 2i32,
    TexCoord0 = 4i32,
    TexCoord1 = 5i32,
    TexCoord2 = 6i32,
    TexCoord3 = 7i32,
    TexCoord4 = 8i32,
    TexCoord5 = 9i32,
    TexCoord6 = 10i32,
    TexCoord7 = 11i32,
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttribute")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::VertexAttribute =>
    "UnityEngine.Rendering"."VertexAttribute"
);
