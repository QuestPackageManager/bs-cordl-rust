#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncReadManagerMetricsFilters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub TypeIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    pub States: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::Unity::IO::LowLevel::Unsafe::ProcessingState,
        >,
    >,
    pub ReadTypes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::Unity::IO::LowLevel::Unsafe::FileReadType,
        >,
    >,
    pub PriorityLevels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::Unity::IO::LowLevel::Unsafe::Priority>,
    >,
    pub Subsystems: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem,
        >,
    >,
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerMetricsFilters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.IO.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "AsyncReadManagerMetricsFilters";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
impl std::ops::Deref
for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerMetricsFilters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerMetricsFilters")]
impl std::ops::DerefMut
for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerMetricsFilters {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
