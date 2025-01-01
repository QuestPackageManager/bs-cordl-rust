#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ProfilerRecorderHandle {
    padding: [u8; 8usize],
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle =>
    "Unity.Profiling.LowLevel.Unsafe"."ProfilerRecorderHandle"
);
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
impl crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    pub fn get_Valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}