#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeTileInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HAPI_VolumeTileInfo {
    pub minX: i32,
    pub minY: i32,
    pub minZ: i32,
    pub isValid: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeTileInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_VolumeTileInfo =>
    "HoudiniEngineUnity"."HAPI_VolumeTileInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeTileInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_VolumeTileInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeTileInfo")]
impl crate::HoudiniEngineUnity::HAPI_VolumeTileInfo {}
