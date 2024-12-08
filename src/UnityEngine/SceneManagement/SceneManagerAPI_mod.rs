#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneManagerAPI {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SceneManagement::SceneManagerAPI =>
    "UnityEngine.SceneManagement"."SceneManagerAPI"
);
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
impl std::ops::Deref for crate::UnityEngine::SceneManagement::SceneManagerAPI {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
impl std::ops::DerefMut for crate::UnityEngine::SceneManagement::SceneManagerAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
impl crate::UnityEngine::SceneManagement::SceneManagerAPI {
    pub fn GetNumScenesInBuildSettings(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNumScenesInBuildSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadFirstScene(
        &mut self,
        mustLoadAsync: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AsyncOperation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AsyncOperation = __cordl_object
            .invoke("LoadFirstScene", (mustLoadAsync))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneAsyncByNameOrIndex(
        &mut self,
        sceneName: *mut crate::System::String,
        sceneBuildIndex: i32,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        mustCompleteNextFrame: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AsyncOperation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AsyncOperation = __cordl_object
            .invoke(
                "LoadSceneAsyncByNameOrIndex",
                (sceneName, sceneBuildIndex, parameters, mustCompleteNextFrame),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPI")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SceneManagement::SceneManagerAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}