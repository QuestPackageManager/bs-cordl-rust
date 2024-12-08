#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_VolumeType {
    HAPI_VOLUMETYPE_HOUDINI = 0i32,
    HAPI_VOLUMETYPE_INVALID = -1i32,
    HAPI_VOLUMETYPE_MAX = 2i32,
    HAPI_VOLUMETYPE_VDB = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_VolumeType =>
    "HoudiniEngineUnity"."HAPI_VolumeType"
);
