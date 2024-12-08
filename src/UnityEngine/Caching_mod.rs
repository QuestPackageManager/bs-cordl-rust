#[cfg(feature = "UnityEngine+Caching")]
#[repr(C)]
#[derive(Debug)]
pub struct Caching {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Caching")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Caching => "UnityEngine"."Caching"
);
#[cfg(feature = "UnityEngine+Caching")]
impl std::ops::Deref for crate::UnityEngine::Caching {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Caching")]
impl std::ops::DerefMut for crate::UnityEngine::Caching {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Caching")]
impl crate::UnityEngine::Caching {}
#[cfg(feature = "UnityEngine+Caching")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Caching {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
