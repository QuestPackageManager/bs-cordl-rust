#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SceneManagement::SceneManager =>
    "UnityEngine.SceneManagement"."SceneManager"
);
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
impl std::ops::Deref for crate::UnityEngine::SceneManagement::SceneManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
impl crate::UnityEngine::SceneManagement::SceneManager {
    pub fn GetActiveScene() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SceneManagement::Scene,
    > {
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActiveScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveScene_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActiveScene_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSceneAt(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSceneAt", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSceneAt_Injected(
        index: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSceneAt_Injected", (index, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSceneByName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSceneByName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSceneByName_Injected(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSceneByName_Injected", (name, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ActiveSceneChanged(
        previousActiveScene: crate::UnityEngine::SceneManagement::Scene,
        newActiveScene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_ActiveSceneChanged",
                (previousActiveScene, newActiveScene),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SceneLoaded(
        scene: crate::UnityEngine::SceneManagement::Scene,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_SceneLoaded", (scene, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SceneUnloaded(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_SceneUnloaded", (scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFirstScene_Internal(
        _cordl_async: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFirstScene_Internal", (_cordl_async))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsyncNameIndexInternal(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sceneBuildIndex: i32,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        mustCompleteNextFrame: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadSceneAsyncNameIndexInternal",
                (sceneName, sceneBuildIndex, parameters, mustCompleteNextFrame),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Gc3(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSceneAsync", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Gc_LoadSceneMode2(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSceneAsync", (sceneName, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Gc_LoadSceneParameters4(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSceneAsync", (sceneName, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode0(
        sceneBuildIndex: i32,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSceneAsync", (sceneBuildIndex, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneParameters1(
        sceneBuildIndex: i32,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSceneAsync", (sceneBuildIndex, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Gc1(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadScene", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Gc_LoadSceneMode0(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadScene", (sceneName, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Gc_LoadSceneParameters2(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadScene", (sceneName, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode3(
        sceneBuildIndex: i32,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadScene", (sceneBuildIndex, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneParameters4(
        sceneBuildIndex: i32,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadScene", (sceneBuildIndex, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveGameObjectToScene(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MoveGameObjectToScene", (go, scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveGameObjectToScene_Injected(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        scene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MoveGameObjectToScene_Injected", (go, scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetActiveScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetActiveScene", (scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetActiveScene_Injected(
        scene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetActiveScene_Injected", (scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsyncInternal(
        scene: crate::UnityEngine::SceneManagement::Scene,
        options: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSceneAsyncInternal", (scene, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsyncInternal_Injected(
        scene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
        options: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSceneAsyncInternal_Injected", (scene, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsync_Scene0(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSceneAsync", (scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsync_UnloadSceneOptions1(
        scene: crate::UnityEngine::SceneManagement::Scene,
        options: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSceneAsync", (scene, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sceneLoaded(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::SceneManagement::Scene,
            crate::UnityEngine::SceneManagement::LoadSceneMode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_sceneLoaded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sceneUnloaded(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_sceneUnloaded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_sceneCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneCountInBuildSettings() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_sceneCountInBuildSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sceneLoaded(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::SceneManagement::Scene,
            crate::UnityEngine::SceneManagement::LoadSceneMode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_sceneLoaded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sceneUnloaded(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_sceneUnloaded", (value))?;
        Ok(__cordl_ret.into())
    }
}
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
