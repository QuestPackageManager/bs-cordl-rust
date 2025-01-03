#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeVisualInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HAPI_VolumeVisualInfo {
    pub _cordl_type: crate::HoudiniEngineUnity::HAPI_VolumeVisualType,
    pub iso: f32,
    pub density: f32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeVisualInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_VolumeVisualInfo =>
    "HoudiniEngineUnity"."HAPI_VolumeVisualInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeVisualInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_VolumeVisualInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeVisualInfo")]
impl crate::HoudiniEngineUnity::HAPI_VolumeVisualInfo {}
