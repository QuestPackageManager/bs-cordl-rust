#[cfg(feature = "FileStorageExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FileStorageExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "FileStorageExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileStorageExtensions => ""
    ."FileStorageExtensions"
);
#[cfg(feature = "FileStorageExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::FileStorageExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileStorageExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileStorageExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileStorageExtensions")]
impl crate::GlobalNamespace::FileStorageExtensions {
    pub const kSizeInBytesUntilDeserializeWarning: i32 = 10000i32;
    #[cfg(feature = "FileStorageExtensions+_LoadFromJSONFileAsync_d__8_1")]
    pub type _LoadFromJSONFileAsync_d__8_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::FileStorageExtensions__LoadFromJSONFileAsync_d__8_1<
        T,
    >;
    #[cfg(feature = "FileStorageExtensions+__c__DisplayClass1_0")]
    pub type __c__DisplayClass1_0 = crate::GlobalNamespace::FileStorageExtensions___c__DisplayClass1_0;
    #[cfg(feature = "FileStorageExtensions+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::GlobalNamespace::FileStorageExtensions___c__DisplayClass2_0;
    #[cfg(feature = "FileStorageExtensions+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::GlobalNamespace::FileStorageExtensions___c__DisplayClass3_0;
    #[cfg(feature = "FileStorageExtensions+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::GlobalNamespace::FileStorageExtensions___c__DisplayClass4_0;
}
#[cfg(feature = "FileStorageExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FileStorageExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
