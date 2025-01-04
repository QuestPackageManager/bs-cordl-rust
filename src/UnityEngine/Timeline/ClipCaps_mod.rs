#[cfg(feature = "UnityEngine+Timeline+ClipCaps")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClipCaps {
    #[default]
    All = -1i32,
    AutoScale = 40i32,
    Blending = 16i32,
    ClipIn = 4i32,
    Extrapolation = 2i32,
    Looping = 1i32,
    None = 0i32,
    SpeedMultiplier = 8i32,
}
#[cfg(feature = "UnityEngine+Timeline+ClipCaps")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::ClipCaps =>
    "UnityEngine.Timeline"."ClipCaps"
);
