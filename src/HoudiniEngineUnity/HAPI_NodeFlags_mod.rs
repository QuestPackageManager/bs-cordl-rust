#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_NodeFlags {
    #[default]
    HAPI_NODEFLAGS_ANY = -1i32,
    HAPI_NODEFLAGS_BYPASS = 32i32,
    HAPI_NODEFLAGS_DISPLAY = 1i32,
    HAPI_NODEFLAGS_EDITABLE = 16i32,
    HAPI_NODEFLAGS_LOCKED = 8i32,
    HAPI_NODEFLAGS_NETWORK = 64i32,
    HAPI_NODEFLAGS_NONE = 0i32,
    HAPI_NODEFLAGS_NON_BYPASS = 16384i32,
    HAPI_NODEFLAGS_OBJ_CAMERA = 256i32,
    HAPI_NODEFLAGS_OBJ_GEOMETRY = 128i32,
    HAPI_NODEFLAGS_OBJ_LIGHT = 512i32,
    HAPI_NODEFLAGS_OBJ_SUBNET = 1024i32,
    HAPI_NODEFLAGS_RENDER = 2i32,
    HAPI_NODEFLAGS_SOP_CURVE = 2048i32,
    HAPI_NODEFLAGS_SOP_GUIDE = 4096i32,
    HAPI_NODEFLAGS_TEMPLATED = 4i32,
    HAPI_NODEFLAGS_TOP_NONSCHEDULER = 8192i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_NodeFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_NodeFlags =>
    "HoudiniEngineUnity"."HAPI_NodeFlags"
);
