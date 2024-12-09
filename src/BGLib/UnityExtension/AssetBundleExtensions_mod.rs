#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetBundleExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::AssetBundleExtensions =>
    "BGLib.UnityExtension"."AssetBundleExtensions"
);
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl std::ops::Deref for crate::BGLib::UnityExtension::AssetBundleExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::AssetBundleExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl crate::BGLib::UnityExtension::AssetBundleExtensions {
    #[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions+_ExistsAsync_d__0")]
    pub type _ExistsAsync_d__0 = crate::BGLib::UnityExtension::AssetBundleExtensions__ExistsAsync_d__0;
    #[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions+__c__DisplayClass1_0")]
    pub type __c__DisplayClass1_0 = crate::BGLib::UnityExtension::AssetBundleExtensions___c__DisplayClass1_0;
    #[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::BGLib::UnityExtension::AssetBundleExtensions___c__DisplayClass2_0;
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::AssetBundleExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
