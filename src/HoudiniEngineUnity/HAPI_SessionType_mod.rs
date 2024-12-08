#[cfg(feature = "HoudiniEngineUnity+HAPI_SessionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_SessionType {
    HAPI_SESSION_CUSTOM1 = 2i32,
    HAPI_SESSION_CUSTOM2 = 3i32,
    HAPI_SESSION_CUSTOM3 = 4i32,
    HAPI_SESSION_INPROCESS = 0i32,
    HAPI_SESSION_MAX = 5i32,
    HAPI_SESSION_THRIFT = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_SessionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_SessionType =>
    "HoudiniEngineUnity"."HAPI_SessionType"
);
