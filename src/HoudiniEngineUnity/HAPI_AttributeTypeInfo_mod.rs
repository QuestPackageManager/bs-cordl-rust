#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeTypeInfo")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_AttributeTypeInfo {
    HAPI_ATTRIBUTE_TYPE_BOX = 12i32,
    HAPI_ATTRIBUTE_TYPE_BOX2 = 11i32,
    HAPI_ATTRIBUTE_TYPE_COLOR = 5i32,
    HAPI_ATTRIBUTE_TYPE_HIDDEN = 10i32,
    HAPI_ATTRIBUTE_TYPE_HPOINT = 2i32,
    HAPI_ATTRIBUTE_TYPE_INVALID = -1i32,
    HAPI_ATTRIBUTE_TYPE_MATRIX = 8i32,
    HAPI_ATTRIBUTE_TYPE_MATRIX3 = 7i32,
    HAPI_ATTRIBUTE_TYPE_MAX = 14i32,
    HAPI_ATTRIBUTE_TYPE_NONE = 0i32,
    HAPI_ATTRIBUTE_TYPE_NORMAL = 4i32,
    HAPI_ATTRIBUTE_TYPE_POINT = 1i32,
    HAPI_ATTRIBUTE_TYPE_QUATERNION = 6i32,
    HAPI_ATTRIBUTE_TYPE_ST = 9i32,
    HAPI_ATTRIBUTE_TYPE_TEXTURE = 13i32,
    HAPI_ATTRIBUTE_TYPE_VECTOR = 3i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeTypeInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_AttributeTypeInfo =>
    "HoudiniEngineUnity"."HAPI_AttributeTypeInfo"
);
