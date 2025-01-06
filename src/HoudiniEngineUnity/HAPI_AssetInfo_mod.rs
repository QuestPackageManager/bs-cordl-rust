#[cfg(feature = "HoudiniEngineUnity+HAPI_AssetInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HAPI_AssetInfo {
    pub nodeId: i32,
    pub objectNodeId: i32,
    pub hasEverCooked: bool,
    pub nameSH: i32,
    pub labelSH: i32,
    pub filePathSH: i32,
    pub versionSH: i32,
    pub fullOpNameSH: i32,
    pub helpTextSH: i32,
    pub helpURLSH: i32,
    pub objectCount: i32,
    pub handleCount: i32,
    pub transformInputCount: i32,
    pub geoInputCount: i32,
    pub geoOutputCount: i32,
    pub haveObjectsChanged: bool,
    pub haveMaterialsChanged: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AssetInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_AssetInfo =>
    "HoudiniEngineUnity"."HAPI_AssetInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_AssetInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_AssetInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AssetInfo")]
impl crate::HoudiniEngineUnity::HAPI_AssetInfo {}
