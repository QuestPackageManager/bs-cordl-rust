#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerCategoryDescription")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ProfilerCategoryDescription {
    padding: quest_hook::libil2cpp::ValueTypePadding<24usize>,
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerCategoryDescription")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription =>
    "Unity.Profiling.LowLevel.Unsafe"."ProfilerCategoryDescription"
);
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerCategoryDescription")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerCategoryDescription")]
impl crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription {}
