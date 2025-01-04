#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightMode")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightMode {
    #[default]
    Baked = 2u8,
    Mixed = 1u8,
    Realtime = 0u8,
    Unknown = 3u8,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::LightMode =>
    "UnityEngine.Experimental.GlobalIllumination"."LightMode"
);
