#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HAPI_PDG_WorkitemInfo {
    pub index: i32,
    pub numResults: i32,
    pub nameSH: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_PDG_WorkitemInfo =>
    "HoudiniEngineUnity"."HAPI_PDG_WorkitemInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_PDG_WorkitemInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemInfo")]
impl crate::HoudiniEngineUnity::HAPI_PDG_WorkitemInfo {}
