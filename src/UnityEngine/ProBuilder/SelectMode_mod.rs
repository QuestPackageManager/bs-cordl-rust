#[cfg(feature = "UnityEngine+ProBuilder+SelectMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectMode {
    #[default]
    Any = 65535i32,
    Edge = 4i32,
    Face = 8i32,
    InputTool = 128i32,
    None = 0i32,
    Object = 1i32,
    TextureEdge = 32i32,
    TextureFace = 16i32,
    TextureVertex = 64i32,
    Vertex = 2i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::SelectMode =>
    "UnityEngine.ProBuilder"."SelectMode"
);
