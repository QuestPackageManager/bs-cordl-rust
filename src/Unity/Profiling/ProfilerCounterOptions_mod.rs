#[cfg(feature = "Unity+Profiling+ProfilerCounterOptions")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfilerCounterOptions {
    FlushOnEndOfFrame = 2u16,
    None = 0u16,
    ResetToZeroOnFlush = 4u16,
}
#[cfg(feature = "Unity+Profiling+ProfilerCounterOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::ProfilerCounterOptions =>
    "Unity.Profiling"."ProfilerCounterOptions"
);
