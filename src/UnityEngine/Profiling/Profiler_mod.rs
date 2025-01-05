#[cfg(feature = "UnityEngine+Profiling+Profiler")]
#[repr(C)]
#[derive(Debug)]
pub struct Profiler {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Profiling::Profiler =>
    "UnityEngine.Profiling"."Profiler"
);
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
impl std::ops::Deref for crate::UnityEngine::Profiling::Profiler {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
impl std::ops::DerefMut for crate::UnityEngine::Profiling::Profiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
impl crate::UnityEngine::Profiling::Profiler {
    pub fn GetMonoUsedSizeLong() -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMonoUsedSizeLong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTotalAllocatedMemoryLong() -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTotalAllocatedMemoryLong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTotalReservedMemoryLong() -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTotalReservedMemoryLong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableBinaryLog(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_enableBinaryLog", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_enabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_logFile(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_logFile", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxUsedMemory(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_maxUsedMemory", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Profiling::Profiler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
