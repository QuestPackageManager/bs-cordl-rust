#[cfg(feature = "UnityEngine+Graphics")]
#[repr(C)]
#[derive(Debug)]
pub struct Graphics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Graphics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Graphics => "UnityEngine"
    ."Graphics"
);
#[cfg(feature = "UnityEngine+Graphics")]
impl std::ops::Deref for crate::UnityEngine::Graphics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Graphics")]
impl std::ops::DerefMut for crate::UnityEngine::Graphics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Graphics")]
impl crate::UnityEngine::Graphics {}
#[cfg(feature = "UnityEngine+Graphics")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Graphics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
