#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfilerUnsafeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility =>
    "Unity.Profiling.LowLevel.Unsafe"."ProfilerUnsafeUtility"
);
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
impl std::ops::Deref
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
impl std::ops::DerefMut
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
impl crate::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility {
    pub fn BeginSample(
        markerPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginSample", (markerPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCategory__Unmanaged(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        colorIndex: crate::Unity::Profiling::ProfilerCategoryColor,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCategory__Unmanaged", (name, nameLen, colorIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCounterValue__Unmanaged(
        counterPtr: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        categoryId: u16,
        flags: crate::Unity::Profiling::LowLevel::MarkerFlags,
        dataType: u8,
        dataUnit: u8,
        dataSize: i32,
        counterOptions: crate::Unity::Profiling::ProfilerCounterOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateCounterValue__Unmanaged",
                (
                    counterPtr,
                    name,
                    nameLen,
                    categoryId,
                    flags,
                    dataType,
                    dataUnit,
                    dataSize,
                    counterOptions,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMarker(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        categoryId: u16,
        flags: crate::Unity::Profiling::LowLevel::MarkerFlags,
        metadataCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMarker", (name, categoryId, flags, metadataCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMarker__Unmanaged(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        categoryId: u16,
        flags: crate::Unity::Profiling::LowLevel::MarkerFlags,
        metadataCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateMarker__Unmanaged",
                (name, nameLen, categoryId, flags, metadataCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndSample(
        markerPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndSample", (markerPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCategoryDescription(
        categoryId: u16,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription,
    > {
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCategoryDescription", (categoryId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCategoryDescription_Injected(
        categoryId: u16,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCategoryDescription_Injected", (categoryId, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMarkerMetadata__Unmanaged(
        markerPtr: crate::System::IntPtr,
        index: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        _cordl_type: u8,
        unit: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMarkerMetadata__Unmanaged",
                (markerPtr, index, name, nameLen, _cordl_type, unit),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Utf8ToString(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        charsLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Utf8ToString", (chars, charsLen))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
