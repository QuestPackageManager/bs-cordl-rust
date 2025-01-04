#[cfg(feature = "HoudiniEngineUnity+HAPI_CurveType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_CurveType {
    #[default]
    HAPI_CURVETYPE_BEZIER = 2i32,
    HAPI_CURVETYPE_INVALID = -1i32,
    HAPI_CURVETYPE_LINEAR = 0i32,
    HAPI_CURVETYPE_MAX = 3i32,
    HAPI_CURVETYPE_NURBS = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_CurveType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_CurveType =>
    "HoudiniEngineUnity"."HAPI_CurveType"
);
