#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeVisualType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_VolumeVisualType {
    #[default]
    HAPI_VOLUMEVISTYPE_HEIGHTFIELD = 4i32,
    HAPI_VOLUMEVISTYPE_INVALID = -1i32,
    HAPI_VOLUMEVISTYPE_INVISIBLE = 3i32,
    HAPI_VOLUMEVISTYPE_ISO = 2i32,
    HAPI_VOLUMEVISTYPE_MAX = 5i32,
    HAPI_VOLUMEVISTYPE_RAINBOW = 1i32,
    HAPI_VOLUMEVISTYPE_SMOKE = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_VolumeVisualType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_VolumeVisualType =>
    "HoudiniEngineUnity"."HAPI_VolumeVisualType"
);
