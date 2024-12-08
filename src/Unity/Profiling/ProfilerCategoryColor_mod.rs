#[cfg(feature = "Unity+Profiling+ProfilerCategoryColor")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfilerCategoryColor {
    Animation = 5u16,
    Audio = 6u16,
    AudioJob = 7u16,
    AudioUpdateJob = 8u16,
    Build = 15u16,
    BurstJobs = 2u16,
    GC = 10u16,
    Input = 16u16,
    Internal = 13u16,
    Lighting = 9u16,
    Memory = 12u16,
    Other = 3u16,
    Physics = 4u16,
    Render = 0u16,
    Scripts = 1u16,
    UI = 14u16,
    VSync = 11u16,
}
#[cfg(feature = "Unity+Profiling+ProfilerCategoryColor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::ProfilerCategoryColor =>
    "Unity.Profiling"."ProfilerCategoryColor"
);
