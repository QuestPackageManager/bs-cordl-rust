#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_GeoType {
    #[default]
    HAPI_GEOTYPE_CURVE = 3i32,
    HAPI_GEOTYPE_DEFAULT = 0i32,
    HAPI_GEOTYPE_INPUT = 2i32,
    HAPI_GEOTYPE_INTERMEDIATE = 1i32,
    HAPI_GEOTYPE_INVALID = -1i32,
    HAPI_GEOTYPE_MAX = 4i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_GeoType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_GeoType =>
    "HoudiniEngineUnity"."HAPI_GeoType"
);
