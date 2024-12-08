#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+AngularFalloffType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngularFalloffType {
    AnalyticAndInnerAngle = 1u8,
    LUT = 0u8,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+AngularFalloffType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::AngularFalloffType =>
    "UnityEngine.Experimental.GlobalIllumination"."AngularFalloffType"
);
