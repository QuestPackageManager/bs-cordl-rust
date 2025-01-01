#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderDescription")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ProfilerRecorderDescription {
    padding: [u8; 24usize],
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderDescription")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription =>
    "Unity.Profiling.LowLevel.Unsafe"."ProfilerRecorderDescription"
);
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderDescription")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderDescription")]
impl crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription {
    pub fn get_Category(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerCategory> {
        let __cordl_ret: crate::Unity::Profiling::ProfilerCategory = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Category",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Name", ())?;
        Ok(__cordl_ret.into())
    }
}