#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+FalloffType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FalloffType {
    #[default]
    InverseSquared = 0u8,
    InverseSquaredNoRangeAttenuation = 1u8,
    Legacy = 3u8,
    Linear = 2u8,
    Undefined = 4u8,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+FalloffType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::FalloffType =>
    "UnityEngine.Experimental.GlobalIllumination"."FalloffType"
);
