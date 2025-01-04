#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeOwner")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_AttributeOwner {
    #[default]
    HAPI_ATTROWNER_DETAIL = 3i32,
    HAPI_ATTROWNER_INVALID = -1i32,
    HAPI_ATTROWNER_MAX = 4i32,
    HAPI_ATTROWNER_POINT = 1i32,
    HAPI_ATTROWNER_PRIM = 2i32,
    HAPI_ATTROWNER_VERTEX = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeOwner")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_AttributeOwner =>
    "HoudiniEngineUnity"."HAPI_AttributeOwner"
);
