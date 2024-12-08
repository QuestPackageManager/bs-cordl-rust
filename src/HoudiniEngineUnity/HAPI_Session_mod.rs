#[cfg(feature = "HoudiniEngineUnity+HAPI_Session")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_Session {
    pub _cordl_type: crate::HoudiniEngineUnity::HAPI_SessionType,
    pub id: i64,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_Session")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_Session =>
    "HoudiniEngineUnity"."HAPI_Session"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_Session")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_Session {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_Session")]
impl crate::HoudiniEngineUnity::HAPI_Session {}
