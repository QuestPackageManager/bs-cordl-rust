#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct EdgeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::EdgeUtility =>
    "UnityEngine.ProBuilder"."EdgeUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::EdgeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::EdgeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
impl crate::UnityEngine::ProBuilder::EdgeUtility {
    #[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::UnityEngine::ProBuilder::EdgeUtility___c__DisplayClass0_0;
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::EdgeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
