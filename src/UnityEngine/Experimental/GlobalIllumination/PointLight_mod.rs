#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+PointLight")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PointLight {
    pub instanceID: i32,
    pub shadow: bool,
    pub mode: crate::UnityEngine::Experimental::GlobalIllumination::LightMode,
    pub position: crate::UnityEngine::Vector3,
    pub orientation: crate::UnityEngine::Quaternion,
    pub color: crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
    pub indirectColor: crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
    pub range: f32,
    pub sphereRadius: f32,
    pub falloff: crate::UnityEngine::Experimental::GlobalIllumination::FalloffType,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+PointLight")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::PointLight =>
    "UnityEngine.Experimental.GlobalIllumination"."PointLight"
);
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+PointLight")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::GlobalIllumination::PointLight {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+PointLight")]
impl crate::UnityEngine::Experimental::GlobalIllumination::PointLight {}
