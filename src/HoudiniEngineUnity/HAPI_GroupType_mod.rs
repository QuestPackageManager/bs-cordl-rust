#[cfg(feature = "HoudiniEngineUnity+HAPI_GroupType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_GroupType {
    HAPI_GROUPTYPE_EDGE = 2i32,
    HAPI_GROUPTYPE_INVALID = -1i32,
    HAPI_GROUPTYPE_MAX = 3i32,
    HAPI_GROUPTYPE_POINT = 0i32,
    HAPI_GROUPTYPE_PRIM = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_GroupType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_GroupType =>
    "HoudiniEngineUnity"."HAPI_GroupType"
);
