#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncReadManagerMetricsFilters {
    __cordl_parent: crate::System::Object,
    pub TypeIDs: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
    pub States: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::Unity::IO::LowLevel::Unsafe::ProcessingState,
    >,
    pub ReadTypes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::Unity::IO::LowLevel::Unsafe::FileReadType,
    >,
    pub PriorityLevels: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::Unity::IO::LowLevel::Unsafe::Priority,
    >,
    pub Subsystems: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem,
    >,
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::IO::LowLevel::Unsafe::AsyncReadManagerMetricsFilters =>
    "Unity.IO.LowLevel.Unsafe"."AsyncReadManagerMetricsFilters"
);
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
impl std::ops::Deref
for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerMetricsFilters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
impl std::ops::DerefMut
for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerMetricsFilters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
impl crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerMetricsFilters {}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerMetricsFilters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
