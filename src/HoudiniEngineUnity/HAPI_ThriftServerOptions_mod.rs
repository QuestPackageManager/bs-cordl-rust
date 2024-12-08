#[cfg(feature = "HoudiniEngineUnity+HAPI_ThriftServerOptions")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_ThriftServerOptions {
    pub autoClose: bool,
    pub timeoutMs: f32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ThriftServerOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ThriftServerOptions =>
    "HoudiniEngineUnity"."HAPI_ThriftServerOptions"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_ThriftServerOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_ThriftServerOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ThriftServerOptions")]
impl crate::HoudiniEngineUnity::HAPI_ThriftServerOptions {}
