#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobRanges")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct JobRanges {
    pub BatchSize: i32,
    pub NumJobs: i32,
    pub TotalIterationCount: i32,
    pub StartEndIndex: crate::System::IntPtr,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobRanges")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Jobs::LowLevel::Unsafe::JobRanges =>
    "Unity.Jobs.LowLevel.Unsafe"."JobRanges"
);
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobRanges")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Jobs::LowLevel::Unsafe::JobRanges {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobRanges")]
impl crate::Unity::Jobs::LowLevel::Unsafe::JobRanges {}
