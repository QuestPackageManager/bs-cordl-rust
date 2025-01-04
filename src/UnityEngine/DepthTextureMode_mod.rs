#[cfg(feature = "UnityEngine+DepthTextureMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DepthTextureMode {
    #[default]
    Depth = 1i32,
    DepthNormals = 2i32,
    MotionVectors = 4i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+DepthTextureMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::DepthTextureMode => "UnityEngine"
    ."DepthTextureMode"
);
