#[cfg(feature = "Unity+Profiling+LowLevel+MarkerFlags")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerFlags {
    #[default]
    AvailabilityEditor = 4u16,
    AvailabilityNonDevelopment = 8u16,
    Counter = 128u16,
    Default = 0u16,
    SampleGPU = 256u16,
    Script = 2u16,
    ScriptDeepProfiler = 64u16,
    ScriptInvoke = 32u16,
    Warning = 16u16,
}
#[cfg(feature = "Unity+Profiling+LowLevel+MarkerFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::LowLevel::MarkerFlags =>
    "Unity.Profiling.LowLevel"."MarkerFlags"
);
