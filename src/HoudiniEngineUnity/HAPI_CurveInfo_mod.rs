#[cfg(feature = "HoudiniEngineUnity+HAPI_CurveInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_CurveInfo {
    pub curveType: crate::HoudiniEngineUnity::HAPI_CurveType,
    pub curveCount: i32,
    pub vertexCount: i32,
    pub knotCount: i32,
    pub isPeriodic: bool,
    pub isRational: bool,
    pub order: i32,
    pub hasKnots: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_CurveInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_CurveInfo =>
    "HoudiniEngineUnity"."HAPI_CurveInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_CurveInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_CurveInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_CurveInfo")]
impl crate::HoudiniEngineUnity::HAPI_CurveInfo {}
