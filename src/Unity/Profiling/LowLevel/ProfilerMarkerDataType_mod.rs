#[cfg(feature = "Unity+Profiling+LowLevel+ProfilerMarkerDataType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfilerMarkerDataType {
    #[default]
    Blob8 = 11u8,
    Double = 7u8,
    Float = 6u8,
    GfxResourceId = 12u8,
    InstanceId = 1u8,
    Int32 = 2u8,
    Int64 = 4u8,
    String16 = 9u8,
    UInt32 = 3u8,
    UInt64 = 5u8,
}
#[cfg(feature = "Unity+Profiling+LowLevel+ProfilerMarkerDataType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Profiling::LowLevel::ProfilerMarkerDataType => "Unity.Profiling.LowLevel"
    ."ProfilerMarkerDataType"
);
