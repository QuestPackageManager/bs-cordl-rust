#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SceneManagement::SceneManager =>
    "UnityEngine.SceneManagement"."SceneManager"
);
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
impl std::ops::Deref for crate::UnityEngine::SceneManagement::SceneManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
impl std::ops::DerefMut for crate::UnityEngine::SceneManagement::SceneManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
impl crate::UnityEngine::SceneManagement::SceneManager {}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SceneManagement::SceneManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
