#[cfg(feature = "HoudiniEngineUnity+HAPI_EnvIntType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_EnvIntType {
    HAPI_ENVINT_INVALID = -1i32,
    HAPI_ENVINT_MAX = 221i32,
    HAPI_ENVINT_VERSION_HOUDINI_BUILD = 120i32,
    HAPI_ENVINT_VERSION_HOUDINI_ENGINE_API = 220i32,
    HAPI_ENVINT_VERSION_HOUDINI_ENGINE_MAJOR = 200i32,
    HAPI_ENVINT_VERSION_HOUDINI_ENGINE_MINOR = 210i32,
    HAPI_ENVINT_VERSION_HOUDINI_MAJOR = 100i32,
    HAPI_ENVINT_VERSION_HOUDINI_MINOR = 110i32,
    HAPI_ENVINT_VERSION_HOUDINI_PATCH = 130i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_EnvIntType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_EnvIntType =>
    "HoudiniEngineUnity"."HAPI_EnvIntType"
);
