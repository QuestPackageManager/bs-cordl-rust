#[cfg(feature = "BeatSaber+PerformancePresets+BloomPrepassTextureEffectPreset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BloomPrepassTextureEffectPreset {
    HD = 0i32,
    HDWithoutToneMapping = 1i32,
}
#[cfg(feature = "BeatSaber+PerformancePresets+BloomPrepassTextureEffectPreset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::PerformancePresets::BloomPrepassTextureEffectPreset =>
    "BeatSaber.PerformancePresets"."BloomPrepassTextureEffectPreset"
);
