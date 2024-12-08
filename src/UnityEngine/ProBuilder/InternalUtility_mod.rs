#[cfg(feature = "UnityEngine+ProBuilder+InternalUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+InternalUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::InternalUtility =>
    "UnityEngine.ProBuilder"."InternalUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+InternalUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::InternalUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+InternalUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::InternalUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+InternalUtility")]
impl crate::UnityEngine::ProBuilder::InternalUtility {
    #[cfg(feature = "UnityEngine+ProBuilder+InternalUtility+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::UnityEngine::ProBuilder::InternalUtility___c__DisplayClass6_0;
}
#[cfg(feature = "UnityEngine+ProBuilder+InternalUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::InternalUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
