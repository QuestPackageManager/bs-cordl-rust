#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageDataFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_ImageDataFormat {
    HAPI_IMAGE_DATA_DEFAULT = 0i32,
    HAPI_IMAGE_DATA_FLOAT16 = 3i32,
    HAPI_IMAGE_DATA_FLOAT32 = 4i32,
    HAPI_IMAGE_DATA_INT16 = 1i32,
    HAPI_IMAGE_DATA_INT32 = 2i32,
    HAPI_IMAGE_DATA_MAX = 5i32,
    HAPI_IMAGE_DATA_UNKNOWN = -1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ImageDataFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ImageDataFormat =>
    "HoudiniEngineUnity"."HAPI_ImageDataFormat"
);
