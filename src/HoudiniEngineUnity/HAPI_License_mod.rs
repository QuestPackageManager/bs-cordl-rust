#[cfg(feature = "HoudiniEngineUnity+HAPI_License")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_License {
    HAPI_LICENSE_HOUDINI = 2i32,
    HAPI_LICENSE_HOUDINI_ENGINE = 1i32,
    HAPI_LICENSE_HOUDINI_ENGINE_INDIE = 4i32,
    HAPI_LICENSE_HOUDINI_FX = 3i32,
    HAPI_LICENSE_HOUDINI_INDIE = 5i32,
    HAPI_LICENSE_MAX = 6i32,
    HAPI_LICENSE_NONE = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_License")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_License =>
    "HoudiniEngineUnity"."HAPI_License"
);
