#[cfg(feature = "UnityEngine+Resources")]
#[repr(C)]
#[derive(Debug)]
pub struct Resources {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Resources")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Resources => "UnityEngine"
    ."Resources"
);
#[cfg(feature = "UnityEngine+Resources")]
impl std::ops::Deref for crate::UnityEngine::Resources {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Resources")]
impl std::ops::DerefMut for crate::UnityEngine::Resources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Resources")]
impl crate::UnityEngine::Resources {}
#[cfg(feature = "UnityEngine+Resources")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Resources {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
