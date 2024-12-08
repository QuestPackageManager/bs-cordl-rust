#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemResultInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_PDG_WorkitemResultInfo {
    pub resultSH: i32,
    pub resultTagSH: i32,
    pub resultHash: i64,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemResultInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_PDG_WorkitemResultInfo
    => "HoudiniEngineUnity"."HAPI_PDG_WorkitemResultInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemResultInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_PDG_WorkitemResultInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemResultInfo")]
impl crate::HoudiniEngineUnity::HAPI_PDG_WorkitemResultInfo {}
