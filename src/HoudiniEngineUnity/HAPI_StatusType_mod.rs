#[cfg(feature = "HoudiniEngineUnity+HAPI_StatusType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_StatusType {
    HAPI_STATUS_CALL_RESULT = 0i32,
    HAPI_STATUS_COOK_RESULT = 1i32,
    HAPI_STATUS_COOK_STATE = 2i32,
    HAPI_STATUS_MAX = 3i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_StatusType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_StatusType =>
    "HoudiniEngineUnity"."HAPI_StatusType"
);
