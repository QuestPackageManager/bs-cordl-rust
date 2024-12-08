#[cfg(feature = "HoudiniEngineUnity+HAPI_PartType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_PartType {
    HAPI_PARTTYPE_BOX = 4i32,
    HAPI_PARTTYPE_CURVE = 1i32,
    HAPI_PARTTYPE_INSTANCER = 3i32,
    HAPI_PARTTYPE_INVALID = -1i32,
    HAPI_PARTTYPE_MAX = 6i32,
    HAPI_PARTTYPE_MESH = 0i32,
    HAPI_PARTTYPE_SPHERE = 5i32,
    HAPI_PARTTYPE_VOLUME = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PartType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_PartType =>
    "HoudiniEngineUnity"."HAPI_PartType"
);
