#[cfg(feature = "UnityEngine+Rendering+PassType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PassType {
    #[default]
    Deferred = 10i32,
    ForwardAdd = 5i32,
    ForwardBase = 4i32,
    GrabPass = 15i32,
    LightPrePassBase = 6i32,
    LightPrePassFinal = 7i32,
    Meta = 11i32,
    MotionVectors = 12i32,
    Normal = 0i32,
    ScriptableRenderPipeline = 13i32,
    ScriptableRenderPipelineDefaultUnlit = 14i32,
    ShadowCaster = 8i32,
    Vertex = 1i32,
    VertexLM = 2i32,
    VertexLMRGBM = 3i32,
}
#[cfg(feature = "UnityEngine+Rendering+PassType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::PassType =>
    "UnityEngine.Rendering"."PassType"
);
