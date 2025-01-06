#[cfg(feature = "HoudiniEngineUnity+HAPI_CookOptions")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HAPI_CookOptions {
    pub splitGeosByGroup: bool,
    pub splitGeosByAttribute: bool,
    pub splitAttrSH: i32,
    pub maxVerticesPerPrimitive: i32,
    pub refineCurveToLinear: bool,
    pub curveRefineLOD: f32,
    pub clearErrorsAndWarnings: bool,
    pub cookTemplatedGeos: bool,
    pub splitPointsByVertexAttributes: bool,
    pub packedPrimInstancingMode: crate::HoudiniEngineUnity::HAPI_PackedPrimInstancingMode,
    pub handleBoxPartTypes: bool,
    pub handleSpherePartTypes: bool,
    pub checkPartChanges: bool,
    pub cacheMeshTopology: bool,
    pub extraFlags: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_CookOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_CookOptions =>
    "HoudiniEngineUnity"."HAPI_CookOptions"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_CookOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_CookOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_CookOptions")]
impl crate::HoudiniEngineUnity::HAPI_CookOptions {}
