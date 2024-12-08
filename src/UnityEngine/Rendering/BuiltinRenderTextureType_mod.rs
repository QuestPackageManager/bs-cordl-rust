#[cfg(feature = "UnityEngine+Rendering+BuiltinRenderTextureType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinRenderTextureType {
    BindableTexture = -1i32,
    BufferPtr = -3i32,
    CameraTarget = 2i32,
    CurrentActive = 1i32,
    Depth = 3i32,
    DepthNormals = 4i32,
    GBuffer0 = 10i32,
    GBuffer1 = 11i32,
    GBuffer2 = 12i32,
    GBuffer3 = 13i32,
    GBuffer4 = 16i32,
    GBuffer5 = 17i32,
    GBuffer6 = 18i32,
    GBuffer7 = 19i32,
    MotionVectors = 15i32,
    None = 0i32,
    PrepassLight = 8i32,
    PrepassLightSpec = 9i32,
    PrepassNormalsSpec = 7i32,
    PropertyName = -4i32,
    Reflections = 14i32,
    RenderTexture = -2i32,
    ResolvedDepth = 5i32,
}
#[cfg(feature = "UnityEngine+Rendering+BuiltinRenderTextureType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BuiltinRenderTextureType
    => "UnityEngine.Rendering"."BuiltinRenderTextureType"
);
