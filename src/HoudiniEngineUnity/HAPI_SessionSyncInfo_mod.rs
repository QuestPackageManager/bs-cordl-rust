#[cfg(feature = "HoudiniEngineUnity+HAPI_SessionSyncInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_SessionSyncInfo {
    pub cookUsingHoudiniTime: bool,
    pub syncViewport: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_SessionSyncInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_SessionSyncInfo =>
    "HoudiniEngineUnity"."HAPI_SessionSyncInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_SessionSyncInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_SessionSyncInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_SessionSyncInfo")]
impl crate::HoudiniEngineUnity::HAPI_SessionSyncInfo {}
