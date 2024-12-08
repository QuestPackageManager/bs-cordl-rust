#[cfg(feature = "HoudiniEngineUnity+HAPI_StorageType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_StorageType {
    HAPI_STORAGETYPE_FLOAT = 2i32,
    HAPI_STORAGETYPE_FLOAT64 = 3i32,
    HAPI_STORAGETYPE_INT = 0i32,
    HAPI_STORAGETYPE_INT16 = 7i32,
    HAPI_STORAGETYPE_INT64 = 1i32,
    HAPI_STORAGETYPE_INT8 = 6i32,
    HAPI_STORAGETYPE_INVALID = -1i32,
    HAPI_STORAGETYPE_MAX = 8i32,
    HAPI_STORAGETYPE_STRING = 4i32,
    HAPI_STORAGETYPE_UINT8 = 5i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_StorageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_StorageType =>
    "HoudiniEngineUnity"."HAPI_StorageType"
);
