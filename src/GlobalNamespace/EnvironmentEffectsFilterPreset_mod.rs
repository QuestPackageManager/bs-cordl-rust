#[cfg(feature = "EnvironmentEffectsFilterPreset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentEffectsFilterPreset {
    AllEffects = 0i32,
    NoEffects = 10i32,
    StrobeFilter = 1i32,
}
#[cfg(feature = "EnvironmentEffectsFilterPreset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentEffectsFilterPreset
    => ""."EnvironmentEffectsFilterPreset"
);
