#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SpriteIntermediateRendererInfo {
    pub SpriteID: i32,
    pub TextureID: i32,
    pub MaterialID: i32,
    pub Color: crate::UnityEngine::Color,
    pub Transform: crate::UnityEngine::Matrix4x4,
    pub Bounds: crate::UnityEngine::Bounds,
    pub Layer: i32,
    pub SortingLayer: i32,
    pub SortingOrder: i32,
    pub SceneCullingMask: u64,
    pub IndexData: crate::System::IntPtr,
    pub VertexData: crate::System::IntPtr,
    pub IndexCount: i32,
    pub VertexCount: i32,
    pub ShaderChannelMask: i32,
}
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo =>
    "UnityEngine.Experimental.U2D"."SpriteIntermediateRendererInfo"
);
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
impl crate::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo {}
