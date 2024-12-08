#[cfg(feature = "UnityEngine+Rendering+CullingSplit")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CullingSplit {
    pub sphereCenter: crate::UnityEngine::Vector3,
    pub sphereRadius: f32,
    pub cullingPlaneOffset: i32,
    pub cullingPlaneCount: i32,
    pub cascadeBlendCullingFactor: f32,
    pub nearPlane: f32,
    pub cullingMatrix: crate::UnityEngine::Matrix4x4,
}
#[cfg(feature = "UnityEngine+Rendering+CullingSplit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::CullingSplit =>
    "UnityEngine.Rendering"."CullingSplit"
);
#[cfg(feature = "UnityEngine+Rendering+CullingSplit")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::CullingSplit {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CullingSplit")]
impl crate::UnityEngine::Rendering::CullingSplit {}
