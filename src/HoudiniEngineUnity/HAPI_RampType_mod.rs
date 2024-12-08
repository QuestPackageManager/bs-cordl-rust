#[cfg(feature = "HoudiniEngineUnity+HAPI_RampType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_RampType {
    HAPI_RAMPTYPE_COLOR = 1i32,
    HAPI_RAMPTYPE_FLOAT = 0i32,
    HAPI_RAMPTYPE_INVALID = -1i32,
    HAPI_RAMPTYPE_MAX = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_RampType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_RampType =>
    "HoudiniEngineUnity"."HAPI_RampType"
);
