#[cfg(feature = "HoudiniEngineUnity+HAPI_CacheProperty")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_CacheProperty {
    #[default]
    HAPI_CACHEPROP_CULL_LEVEL = 5i32,
    HAPI_CACHEPROP_CURRENT = 0i32,
    HAPI_CACHEPROP_HAS_MAX = 3i32,
    HAPI_CACHEPROP_HAS_MIN = 1i32,
    HAPI_CACHEPROP_MAX = 4i32,
    HAPI_CACHEPROP_MIN = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_CacheProperty")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_CacheProperty =>
    "HoudiniEngineUnity"."HAPI_CacheProperty"
);
