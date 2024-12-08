#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneManagerAPIInternal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SceneManagement::SceneManagerAPIInternal =>
    "UnityEngine.SceneManagement"."SceneManagerAPIInternal"
);
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
impl std::ops::Deref for crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
impl std::ops::DerefMut
for crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
impl crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}