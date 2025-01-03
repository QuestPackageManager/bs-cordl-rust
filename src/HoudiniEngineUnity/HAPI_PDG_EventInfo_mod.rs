#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_EventInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HAPI_PDG_EventInfo {
    pub nodeId: i32,
    pub workitemId: i32,
    pub dependencyId: i32,
    pub currentState: i32,
    pub lastState: i32,
    pub eventType: i32,
    pub msgSH: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_EventInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_PDG_EventInfo =>
    "HoudiniEngineUnity"."HAPI_PDG_EventInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_EventInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_PDG_EventInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_EventInfo")]
impl crate::HoudiniEngineUnity::HAPI_PDG_EventInfo {}
