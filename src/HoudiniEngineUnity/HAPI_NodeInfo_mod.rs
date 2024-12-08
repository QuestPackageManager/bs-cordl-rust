#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_NodeInfo {
    pub id: i32,
    pub parentId: i32,
    pub nameSH: i32,
    pub _cordl_type: crate::HoudiniEngineUnity::HAPI_NodeType,
    pub isValid: bool,
    pub totalCookCount: i32,
    pub uniqueHoudiniNodeId: i32,
    pub internalNodePathSH: i32,
    pub parmCount: i32,
    pub parmIntValueCount: i32,
    pub parmFloatValueCount: i32,
    pub parmStringValueCount: i32,
    pub parmChoiceCount: i32,
    pub childNodeCount: i32,
    pub inputCount: i32,
    pub outputCount: i32,
    pub createdPostAssetLoad: bool,
    pub isTimeDependent: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_NodeInfo =>
    "HoudiniEngineUnity"."HAPI_NodeInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_NodeInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeInfo")]
impl crate::HoudiniEngineUnity::HAPI_NodeInfo {}
