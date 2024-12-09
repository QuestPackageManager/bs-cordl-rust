#[cfg(feature = "UnityEngine+Input")]
#[repr(C)]
#[derive(Debug)]
pub struct Input {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Input")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Input => "UnityEngine"."Input"
);
#[cfg(feature = "UnityEngine+Input")]
impl std::ops::Deref for crate::UnityEngine::Input {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Input")]
impl std::ops::DerefMut for crate::UnityEngine::Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Input")]
impl crate::UnityEngine::Input {}
#[cfg(feature = "UnityEngine+Input")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Input {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
