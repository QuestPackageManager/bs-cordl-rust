#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageFileFormat")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_ImageFileFormat {
    pub nameSH: i32,
    pub descriptionSH: i32,
    pub defaultExtensionSH: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageFileFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ImageFileFormat =>
    "HoudiniEngineUnity"."HAPI_ImageFileFormat"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageFileFormat")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_ImageFileFormat {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageFileFormat")]
impl crate::HoudiniEngineUnity::HAPI_ImageFileFormat {}
