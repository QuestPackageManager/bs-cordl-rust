#[cfg(feature = "HoudiniEngineUnity+HEU_AssetEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_AssetEventType {
    #[default]
    BAKE_NEW = 3i32,
    BAKE_UPDATE = 4i32,
    COOK = 2i32,
    RELOAD = 1i32,
    UNKNOWN = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_AssetEventType =>
    "HoudiniEngineUnity"."HEU_AssetEventType"
);
