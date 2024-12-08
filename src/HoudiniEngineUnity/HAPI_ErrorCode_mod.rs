#[cfg(feature = "HoudiniEngineUnity+HAPI_ErrorCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_ErrorCode {
    HAPI_ERRORCODE_ASSET_DEF_NOT_FOUND = 1i32,
    HAPI_ERRORCODE_PYTHON_NODE_ERROR = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ErrorCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ErrorCode =>
    "HoudiniEngineUnity"."HAPI_ErrorCode"
);
