#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateOptions")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HEU_GenerateOptions {
    pub _generateUVs: bool,
    pub _generateTangents: bool,
    pub _generateNormals: bool,
    pub _useLODGroups: bool,
    pub _splitPoints: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_GenerateOptions =>
    "HoudiniEngineUnity"."HEU_GenerateOptions"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HEU_GenerateOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateOptions")]
impl crate::HoudiniEngineUnity::HEU_GenerateOptions {}
