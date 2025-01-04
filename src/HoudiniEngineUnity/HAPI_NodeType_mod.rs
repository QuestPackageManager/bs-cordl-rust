#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_NodeType {
    #[default]
    HAPI_NODETYPE_ANY = -1i32,
    HAPI_NODETYPE_CHOP = 4i32,
    HAPI_NODETYPE_COP = 32i32,
    HAPI_NODETYPE_DOP = 128i32,
    HAPI_NODETYPE_NONE = 0i32,
    HAPI_NODETYPE_OBJ = 1i32,
    HAPI_NODETYPE_ROP = 8i32,
    HAPI_NODETYPE_SHOP = 16i32,
    HAPI_NODETYPE_SOP = 2i32,
    HAPI_NODETYPE_TOP = 256i32,
    HAPI_NODETYPE_VOP = 64i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_NodeType =>
    "HoudiniEngineUnity"."HAPI_NodeType"
);
