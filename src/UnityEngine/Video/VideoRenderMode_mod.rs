#[cfg(feature = "UnityEngine+Video+VideoRenderMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoRenderMode {
    APIOnly = 4i32,
    CameraFarPlane = 0i32,
    CameraNearPlane = 1i32,
    MaterialOverride = 3i32,
    RenderTexture = 2i32,
}
#[cfg(feature = "UnityEngine+Video+VideoRenderMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Video::VideoRenderMode =>
    "UnityEngine.Video"."VideoRenderMode"
);
