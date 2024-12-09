#[cfg(feature = "UnityEngine+AndroidReflection")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidReflection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AndroidReflection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidReflection => "UnityEngine"
    ."AndroidReflection"
);
#[cfg(feature = "UnityEngine+AndroidReflection")]
impl std::ops::Deref for crate::UnityEngine::AndroidReflection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidReflection")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidReflection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidReflection")]
impl crate::UnityEngine::AndroidReflection {}
#[cfg(feature = "UnityEngine+AndroidReflection")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidReflection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
