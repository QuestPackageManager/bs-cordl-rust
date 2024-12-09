#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AsyncReadManagerRequestMetric {
    pub _AssetName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _FileName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _OffsetBytes_k__BackingField: u64,
    pub _SizeBytes_k__BackingField: u64,
    pub _AssetTypeId_k__BackingField: u64,
    pub _CurrentBytesRead_k__BackingField: u64,
    pub _BatchReadCount_k__BackingField: u32,
    pub _IsBatchRead_k__BackingField: bool,
    pub _State_k__BackingField: crate::Unity::IO::LowLevel::Unsafe::ProcessingState,
    pub _ReadType_k__BackingField: crate::Unity::IO::LowLevel::Unsafe::FileReadType,
    pub _PriorityLevel_k__BackingField: crate::Unity::IO::LowLevel::Unsafe::Priority,
    pub _Subsystem_k__BackingField: crate::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem,
    pub _RequestTimeMicroseconds_k__BackingField: f64,
    pub _TimeInQueueMicroseconds_k__BackingField: f64,
    pub _TotalTimeMicroseconds_k__BackingField: f64,
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric =>
    "Unity.IO.LowLevel.Unsafe"."AsyncReadManagerRequestMetric"
);
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
impl crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric {}
