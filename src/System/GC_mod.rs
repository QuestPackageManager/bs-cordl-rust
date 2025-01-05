#[cfg(feature = "System+GC")]
#[repr(C)]
#[derive(Debug)]
pub struct GC {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+GC")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::GC => "System"."GC"
);
#[cfg(feature = "System+GC")]
impl std::ops::Deref for crate::System::GC {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+GC")]
impl std::ops::DerefMut for crate::System::GC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+GC")]
impl crate::System::GC {
    pub fn Collect() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Collect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CollectionCount(generation: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CollectionCount", (generation))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCollectionCount(generation: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCollectionCount", (generation))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxGeneration() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMaxGeneration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMemoryInfo(
        highMemLoadThreshold: quest_hook::libil2cpp::ByRefMut<u32>,
        totalPhysicalMem: quest_hook::libil2cpp::ByRefMut<u64>,
        lastRecordedMemLoad: quest_hook::libil2cpp::ByRefMut<u32>,
        lastRecordedHeapSize: quest_hook::libil2cpp::ByRefMut<crate::System::UIntPtr>,
        lastRecordedFragmentation: quest_hook::libil2cpp::ByRefMut<
            crate::System::UIntPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetMemoryInfo",
                (
                    highMemLoadThreshold,
                    totalPhysicalMem,
                    lastRecordedMemLoad,
                    lastRecordedHeapSize,
                    lastRecordedFragmentation,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalCollect(
        generation: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalCollect", (generation))?;
        Ok(__cordl_ret.into())
    }
    pub fn KeepAlive(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("KeepAlive", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReRegisterForFinalize(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReRegisterForFinalize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn SuppressFinalize(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SuppressFinalize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ReRegisterForFinalize(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_ReRegisterForFinalize", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn _SuppressFinalize(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_SuppressFinalize", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxGeneration() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MaxGeneration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ephemeron_tombstone() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ephemeron_tombstone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn register_ephemeron_array(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Runtime::CompilerServices::Ephemeron,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("register_ephemeron_array", (array))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+GC")]
impl quest_hook::libil2cpp::ObjectType for crate::System::GC {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
