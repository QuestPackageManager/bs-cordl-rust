#[cfg(feature = "Unity+Profiling+ProfilerRecorderOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfilerRecorderOptions {
    #[default]
    CollectOnlyOnCurrentThread = 4i32,
    Default = 24i32,
    GpuRecorder = 64i32,
    KeepAliveDuringDomainReload = 2i32,
    None = 0i32,
    StartImmediately = 1i32,
    SumAllSamplesInFrame = 16i32,
    WrapAroundWhenCapacityReached = 8i32,
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorderOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::ProfilerRecorderOptions =>
    "Unity.Profiling"."ProfilerRecorderOptions"
);
