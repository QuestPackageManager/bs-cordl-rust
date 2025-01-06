#[cfg(feature = "HoudiniEngineUnity+HAPI_ObjectInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HAPI_ObjectInfo {
    pub nameSH: i32,
    pub objectInstancePathSH: i32,
    pub hasTransformChanged: bool,
    pub haveGeosChanged: bool,
    pub isVisible: bool,
    pub isInstancer: bool,
    pub isInstanced: bool,
    pub geoCount: i32,
    pub nodeId: i32,
    pub objectToInstanceId: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ObjectInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ObjectInfo =>
    "HoudiniEngineUnity"."HAPI_ObjectInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_ObjectInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_ObjectInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ObjectInfo")]
impl crate::HoudiniEngineUnity::HAPI_ObjectInfo {}
