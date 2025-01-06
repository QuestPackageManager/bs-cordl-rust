#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HAPI_ImageInfo {
    pub imageFileFormatNameSH: i32,
    pub xRes: i32,
    pub yRes: i32,
    pub dataFormat: crate::HoudiniEngineUnity::HAPI_ImageDataFormat,
    pub interleaved: bool,
    pub packing: crate::HoudiniEngineUnity::HAPI_ImagePacking,
    pub gamma: f64,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ImageInfo =>
    "HoudiniEngineUnity"."HAPI_ImageInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_ImageInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageInfo")]
impl crate::HoudiniEngineUnity::HAPI_ImageInfo {}
