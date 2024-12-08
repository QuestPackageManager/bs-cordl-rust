#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainUtils::TerrainUtility =>
    "UnityEngine.TerrainUtils"."TerrainUtility"
);
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl std::ops::Deref for crate::UnityEngine::TerrainUtils::TerrainUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl std::ops::DerefMut for crate::UnityEngine::TerrainUtils::TerrainUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl crate::UnityEngine::TerrainUtils::TerrainUtility {
    #[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility+__c__DisplayClass2_1")]
    pub type __c__DisplayClass2_1 = crate::UnityEngine::TerrainUtils::TerrainUtility___c__DisplayClass2_1;
    #[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::UnityEngine::TerrainUtils::TerrainUtility___c__DisplayClass2_0;
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TerrainUtils::TerrainUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
