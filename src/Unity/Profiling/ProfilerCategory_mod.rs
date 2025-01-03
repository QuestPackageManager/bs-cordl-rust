#[cfg(feature = "Unity+Profiling+ProfilerCategory")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ProfilerCategory {
    padding: [u8; 2usize],
}
#[cfg(feature = "Unity+Profiling+ProfilerCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::ProfilerCategory =>
    "Unity.Profiling"."ProfilerCategory"
);
#[cfg(feature = "Unity+Profiling+ProfilerCategory")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::ProfilerCategory {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerCategory")]
impl crate::Unity::Profiling::ProfilerCategory {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        category: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (category),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Internal() -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerCategory,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Internal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Memory() -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerCategory,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Memory", ())?;
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
    pub fn get_Render() -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerCategory,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Render", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Scripts() -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::ProfilerCategory,
    > {
        let __cordl_ret: crate::Unity::Profiling::ProfilerCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Scripts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        category: crate::Unity::Profiling::ProfilerCategory,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (category))?;
        Ok(__cordl_ret.into())
    }
}
