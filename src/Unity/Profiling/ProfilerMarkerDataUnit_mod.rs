#[cfg(feature = "Unity+Profiling+ProfilerMarkerDataUnit")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfilerMarkerDataUnit {
    Bytes = 2u8,
    Count = 3u8,
    FrequencyHz = 5u8,
    Percent = 4u8,
    TimeNanoseconds = 1u8,
    Undefined = 0u8,
}
#[cfg(feature = "Unity+Profiling+ProfilerMarkerDataUnit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::ProfilerMarkerDataUnit =>
    "Unity.Profiling"."ProfilerMarkerDataUnit"
);
