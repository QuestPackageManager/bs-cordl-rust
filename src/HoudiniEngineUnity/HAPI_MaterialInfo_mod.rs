#[cfg(feature = "HoudiniEngineUnity+HAPI_MaterialInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HAPI_MaterialInfo {
    pub nodeId: i32,
    pub exists: bool,
    pub hasChanged: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_MaterialInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_MaterialInfo =>
    "HoudiniEngineUnity"."HAPI_MaterialInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_MaterialInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_MaterialInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_MaterialInfo")]
impl crate::HoudiniEngineUnity::HAPI_MaterialInfo {}
