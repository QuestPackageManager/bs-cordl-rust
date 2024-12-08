#[cfg(feature = "BeatSaber+PerformancePresets+MirrorQualityPreset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirrorQualityPreset {
    Fake = 1i32,
    Off = 0i32,
    RenderedHQ = 3i32,
    RenderedLQ = 2i32,
}
#[cfg(feature = "BeatSaber+PerformancePresets+MirrorQualityPreset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::PerformancePresets::MirrorQualityPreset =>
    "BeatSaber.PerformancePresets"."MirrorQualityPreset"
);
