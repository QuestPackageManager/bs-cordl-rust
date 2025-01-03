#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneManagerAPIInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SceneManagement::SceneManagerAPIInternal =>
    "UnityEngine.SceneManagement"."SceneManagerAPIInternal"
);
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
impl std::ops::Deref for crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {
    pub fn GetNumScenesInBuildSettings() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNumScenesInBuildSettings", ())?;
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
    pub fn LoadSceneAsyncNameIndexInternal_Injected(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sceneBuildIndex: i32,
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::LoadSceneParameters,
        >,
        mustCompleteNextFrame: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadSceneAsyncNameIndexInternal_Injected",
                (sceneName, sceneBuildIndex, parameters, mustCompleteNextFrame),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
