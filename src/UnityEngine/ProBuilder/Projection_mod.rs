#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
#[repr(C)]
#[derive(Debug)]
pub struct Projection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Projection =>
    "UnityEngine.ProBuilder"."Projection"
);
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Projection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Projection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
impl crate::UnityEngine::ProBuilder::Projection {
    #[cfg(feature = "UnityEngine+ProBuilder+Projection+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::Projection___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Projection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
