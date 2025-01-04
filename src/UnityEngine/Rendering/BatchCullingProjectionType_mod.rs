#[cfg(feature = "UnityEngine+Rendering+BatchCullingProjectionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BatchCullingProjectionType {
    #[default]
    Orthographic = 2i32,
    Perspective = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingProjectionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::BatchCullingProjectionType => "UnityEngine.Rendering"
    ."BatchCullingProjectionType"
);
