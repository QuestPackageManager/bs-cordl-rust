#[cfg(feature = "Zenject+ZenjectSceneLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenjectSceneLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _projectKernel: quest_hook::libil2cpp::Gc<crate::Zenject::ProjectKernel>,
    pub _sceneContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
}
#[cfg(feature = "Zenject+ZenjectSceneLoader")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::ZenjectSceneLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ZenjectSceneLoader";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Zenject+ZenjectSceneLoader")]
impl std::ops::Deref for crate::Zenject::ZenjectSceneLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectSceneLoader")]
impl std::ops::DerefMut for crate::Zenject::ZenjectSceneLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectSceneLoader")]
impl crate::Zenject::ZenjectSceneLoader {
    pub fn LoadSceneAsync_Il2CppString0(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke("LoadSceneAsync", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneMode1(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke("LoadSceneAsync", (sceneName, loadMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneMode_Action_1_2(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke("LoadSceneAsync", (sceneName, loadMode, extraBindings))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneMode_Action_1_Action_1_LoadSceneRelationship_Action_1_4(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindingsEarly: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke(
                "LoadSceneAsync",
                (
                    sceneName,
                    loadMode,
                    extraBindingsEarly,
                    extraBindings,
                    containerMode,
                    extraBindingsLate,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneMode_Action_1_LoadSceneRelationship3(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke(
                "LoadSceneAsync",
                (sceneName, loadMode, extraBindings, containerMode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_5(
        &mut self,
        sceneIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke("LoadSceneAsync", (sceneIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode6(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke("LoadSceneAsync", (sceneIndex, loadMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode_Action_1_7(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke("LoadSceneAsync", (sceneIndex, loadMode, extraBindings))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode_Action_1_LoadSceneRelationship8(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke(
                "LoadSceneAsync",
                (sceneIndex, loadMode, extraBindings, containerMode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode_Action_1_LoadSceneRelationship_Action_1_9(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = __cordl_object
            .invoke(
                "LoadSceneAsync",
                (sceneIndex, loadMode, extraBindings, containerMode, extraBindingsLate),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneFromAddressablesAsync(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
        extraBindingsEarly: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "LoadSceneFromAddressablesAsync",
                (
                    sceneName,
                    loadMode,
                    activateOnLoad,
                    priority,
                    extraBindingsEarly,
                    extraBindings,
                    containerMode,
                    extraBindingsLate,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString0(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadScene", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneMode1(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadScene", (sceneName, loadMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneMode_Action_1_2(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadScene", (sceneName, loadMode, extraBindings))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneMode_Action_1_LoadSceneRelationship3(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadScene", (sceneName, loadMode, extraBindings, containerMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneMode_Action_1_LoadSceneRelationship_Action_1_4(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadScene",
                (sceneName, loadMode, extraBindings, containerMode, extraBindingsLate),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_5(
        &mut self,
        sceneIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadScene", (sceneIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode6(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadScene", (sceneIndex, loadMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode_Action_1_7(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadScene", (sceneIndex, loadMode, extraBindings))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode_Action_1_LoadSceneRelationship8(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadScene", (sceneIndex, loadMode, extraBindings, containerMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode_Action_1_LoadSceneRelationship_Action_1_9(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadScene",
                (sceneIndex, loadMode, extraBindings, containerMode, extraBindingsLate),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        sceneRoot: quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
        projectKernel: quest_hook::libil2cpp::Gc<crate::Zenject::ProjectKernel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sceneRoot, projectKernel))?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareForLoadScene(
        &mut self,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindingsEarly: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PrepareForLoadScene",
                (
                    loadMode,
                    extraBindingsEarly,
                    extraBindings,
                    extraBindingsLate,
                    containerMode,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("__zenCreate", (P_0))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sceneRoot: quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
        projectKernel: quest_hook::libil2cpp::Gc<crate::Zenject::ProjectKernel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sceneRoot, projectKernel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ZenjectSceneLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ZenjectSceneLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
