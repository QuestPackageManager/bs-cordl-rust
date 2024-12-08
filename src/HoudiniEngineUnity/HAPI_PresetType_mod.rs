#[cfg(feature = "HoudiniEngineUnity+HAPI_PresetType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_PresetType {
    HAPI_PRESETTYPE_BINARY = 0i32,
    HAPI_PRESETTYPE_IDX = 1i32,
    HAPI_PRESETTYPE_INVALID = -1i32,
    HAPI_PRESETTYPE_MAX = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PresetType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_PresetType =>
    "HoudiniEngineUnity"."HAPI_PresetType"
);
