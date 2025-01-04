#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightType {
    #[default]
    Directional = 0u8,
    Disc = 4u8,
    Point = 1u8,
    Rectangle = 3u8,
    Spot = 2u8,
    SpotBoxShape = 6u8,
    SpotPyramidShape = 5u8,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::LightType =>
    "UnityEngine.Experimental.GlobalIllumination"."LightType"
);
