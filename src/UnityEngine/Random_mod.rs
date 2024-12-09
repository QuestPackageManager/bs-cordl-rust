#[cfg(feature = "UnityEngine+Random")]
#[repr(C)]
#[derive(Debug)]
pub struct Random {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Random")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Random => "UnityEngine"."Random"
);
#[cfg(feature = "UnityEngine+Random")]
impl std::ops::Deref for crate::UnityEngine::Random {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Random")]
impl std::ops::DerefMut for crate::UnityEngine::Random {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Random")]
impl crate::UnityEngine::Random {}
#[cfg(feature = "UnityEngine+Random")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Random {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
