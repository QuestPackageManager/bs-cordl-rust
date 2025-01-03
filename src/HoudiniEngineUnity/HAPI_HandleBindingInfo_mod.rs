#[cfg(feature = "HoudiniEngineUnity+HAPI_HandleBindingInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HAPI_HandleBindingInfo {
    pub handleParmNameSH: i32,
    pub assetParmNameSH: i32,
    pub assetParmId: i32,
    pub assetParmIndex: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_HandleBindingInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_HandleBindingInfo =>
    "HoudiniEngineUnity"."HAPI_HandleBindingInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_HandleBindingInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_HandleBindingInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_HandleBindingInfo")]
impl crate::HoudiniEngineUnity::HAPI_HandleBindingInfo {}
