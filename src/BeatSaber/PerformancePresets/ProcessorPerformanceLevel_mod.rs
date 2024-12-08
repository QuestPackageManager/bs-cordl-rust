#[cfg(feature = "BeatSaber+PerformancePresets+ProcessorPerformanceLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorPerformanceLevel {
    Boost = 3i32,
    PowerSavings = 0i32,
    SustainedHigh = 2i32,
    SustainedLow = 1i32,
    Unknown = -1i32,
}
#[cfg(feature = "BeatSaber+PerformancePresets+ProcessorPerformanceLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::PerformancePresets::ProcessorPerformanceLevel =>
    "BeatSaber.PerformancePresets"."ProcessorPerformanceLevel"
);
