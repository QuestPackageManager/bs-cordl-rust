#[cfg(feature = "UnityEngine+ParticleSystemSortMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleSystemSortMode {
    #[default]
    Depth = 4i32,
    Distance = 1i32,
    None = 0i32,
    OldestInFront = 2i32,
    YoungestInFront = 3i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemSortMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemSortMode =>
    "UnityEngine"."ParticleSystemSortMode"
);
