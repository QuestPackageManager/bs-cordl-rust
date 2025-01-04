#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_ParmType {
    #[default]
    HAPI_PARMTYPE_BUTTON = 3i32,
    HAPI_PARMTYPE_COLOR = 5i32,
    HAPI_PARMTYPE_CONTAINER_END = 12i32,
    HAPI_PARMTYPE_CONTAINER_START = 11i32,
    HAPI_PARMTYPE_FLOAT = 4i32,
    HAPI_PARMTYPE_FOLDER = 13i32,
    HAPI_PARMTYPE_INT = 0i32,
    HAPI_PARMTYPE_LABEL = 14i32,
    HAPI_PARMTYPE_MAX = 17i32,
    HAPI_PARMTYPE_MULTIPARMLIST = 1i32,
    HAPI_PARMTYPE_NODE = 10i32,
    HAPI_PARMTYPE_NONVALUE_END = 15i32,
    HAPI_PARMTYPE_PATH_END = 9i32,
    HAPI_PARMTYPE_PATH_FILE = 7i32,
    HAPI_PARMTYPE_PATH_FILE_DIR = 16i32,
    HAPI_PARMTYPE_PATH_FILE_GEO = 8i32,
    HAPI_PARMTYPE_STRING = 6i32,
    HAPI_PARMTYPE_TOGGLE = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ParmType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ParmType =>
    "HoudiniEngineUnity"."HAPI_ParmType"
);
