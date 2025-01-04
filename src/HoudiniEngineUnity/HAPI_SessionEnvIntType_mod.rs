#[cfg(feature = "HoudiniEngineUnity+HAPI_SessionEnvIntType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_SessionEnvIntType {
    #[default]
    HAPI_SESSIONENVINT_INVALID = -1i32,
    HAPI_SESSIONENVINT_LICENSE = 100i32,
    HAPI_SESSIONENVINT_MAX = 101i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_SessionEnvIntType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_SessionEnvIntType =>
    "HoudiniEngineUnity"."HAPI_SessionEnvIntType"
);
