#[cfg(feature = "UnityEngine+AudioSourceCurveType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioSourceCurveType {
    #[default]
    CustomRolloff = 0i32,
    ReverbZoneMix = 2i32,
    SpatialBlend = 1i32,
    Spread = 3i32,
}
#[cfg(feature = "UnityEngine+AudioSourceCurveType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioSourceCurveType =>
    "UnityEngine"."AudioSourceCurveType"
);
