#[cfg(feature = "UnityEngine+MixedLightingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MixedLightingMode {
    #[default]
    IndirectOnly = 0i32,
    Shadowmask = 2i32,
    Subtractive = 1i32,
}
#[cfg(feature = "UnityEngine+MixedLightingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::MixedLightingMode => "UnityEngine"
    ."MixedLightingMode"
);
