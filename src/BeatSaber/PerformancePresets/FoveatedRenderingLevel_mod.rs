#[cfg(feature = "BeatSaber+PerformancePresets+FoveatedRenderingLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoveatedRenderingLevel {
    High = 3i32,
    HighTop = 4i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "BeatSaber+PerformancePresets+FoveatedRenderingLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::PerformancePresets::FoveatedRenderingLevel =>
    "BeatSaber.PerformancePresets"."FoveatedRenderingLevel"
);
