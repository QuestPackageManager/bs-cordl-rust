#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
#[repr(C)]
#[derive(Debug)]
pub struct ClassLibraryInitializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ClassLibraryInitializer =>
    "UnityEngine"."ClassLibraryInitializer"
);
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
impl std::ops::Deref for crate::UnityEngine::ClassLibraryInitializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
impl std::ops::DerefMut for crate::UnityEngine::ClassLibraryInitializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
impl crate::UnityEngine::ClassLibraryInitializer {
    #[cfg(feature = "UnityEngine+ClassLibraryInitializer+__c")]
    pub type __c = crate::UnityEngine::ClassLibraryInitializer___c;
}
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ClassLibraryInitializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
