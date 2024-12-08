#[cfg(feature = "BeatSaber+PerformancePresets+MainEffectPreset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainEffectPreset {
    Off = 0i32,
    Pyramid = 1i32,
    PyramidForBaking = 2i32,
}
#[cfg(feature = "BeatSaber+PerformancePresets+MainEffectPreset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::PerformancePresets::MainEffectPreset
    => "BeatSaber.PerformancePresets"."MainEffectPreset"
);
