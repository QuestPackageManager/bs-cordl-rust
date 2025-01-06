#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmChoiceInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HAPI_ParmChoiceInfo {
    pub parentParmId: i32,
    pub labelSH: i32,
    pub valueSH: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmChoiceInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ParmChoiceInfo =>
    "HoudiniEngineUnity"."HAPI_ParmChoiceInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmChoiceInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_ParmChoiceInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmChoiceInfo")]
impl crate::HoudiniEngineUnity::HAPI_ParmChoiceInfo {}
