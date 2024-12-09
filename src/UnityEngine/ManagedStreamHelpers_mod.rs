#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ManagedStreamHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ManagedStreamHelpers =>
    "UnityEngine"."ManagedStreamHelpers"
);
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
impl std::ops::Deref for crate::UnityEngine::ManagedStreamHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::ManagedStreamHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
impl crate::UnityEngine::ManagedStreamHelpers {}
#[cfg(feature = "UnityEngine+ManagedStreamHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ManagedStreamHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
