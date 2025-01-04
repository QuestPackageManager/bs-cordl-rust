#[cfg(feature = "UnityEngineInternal+LightmapType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightmapType {
    #[default]
    DynamicLightmap = 1i32,
    NoLightmap = -1i32,
    StaticLightmap = 0i32,
}
#[cfg(feature = "UnityEngineInternal+LightmapType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngineInternal::LightmapType =>
    "UnityEngineInternal"."LightmapType"
);
