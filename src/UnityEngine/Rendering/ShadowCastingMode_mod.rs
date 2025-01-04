#[cfg(feature = "UnityEngine+Rendering+ShadowCastingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShadowCastingMode {
    #[default]
    Off = 0i32,
    On = 1i32,
    ShadowsOnly = 3i32,
    TwoSided = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+ShadowCastingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ShadowCastingMode =>
    "UnityEngine.Rendering"."ShadowCastingMode"
);
