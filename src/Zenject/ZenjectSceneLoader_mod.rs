#[cfg(feature = "cordl_class_Zenject+ZenjectSceneLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenjectSceneLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _projectKernel: quest_hook::libil2cpp::Gc<crate::Zenject::ProjectKernel>,
    pub _sceneContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
}
#[cfg(feature = "cordl_class_Zenject+ZenjectSceneLoader")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::ZenjectSceneLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ZenjectSceneLoader";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectSceneLoader")]
impl std::ops::DerefMut for crate::Zenject::ZenjectSceneLoader {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectSceneLoader")]
impl crate::Zenject::ZenjectSceneLoader {
    pub fn LoadSceneAsync_Il2CppString0(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        1usize,
                    >("LoadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadSceneAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> =
            unsafe { cordl_method_info.invoke_unchecked(self, (sceneName))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneMode1(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>, 2usize>(
                        "LoadSceneAsync",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadSceneAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> =
            unsafe { cordl_method_info.invoke_unchecked(self, (sceneName, loadMode))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneMode_Action_1_2(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>, 3usize>(
                        "LoadSceneAsync",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadSceneAsync",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            cordl_method_info.invoke_unchecked(self, (sceneName, loadMode, extraBindings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneMode_Action_1_Action_1_LoadSceneRelationship_Action_1_4(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindingsEarly: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        crate::Zenject::LoadSceneRelationship,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>, 6usize>(
                        "LoadSceneAsync",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadSceneAsync",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    sceneName,
                    loadMode,
                    extraBindingsEarly,
                    extraBindings,
                    containerMode,
                    extraBindingsLate,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneMode_Action_1_LoadSceneRelationship3(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        crate::Zenject::LoadSceneRelationship,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>, 4usize>(
                        "LoadSceneAsync",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadSceneAsync",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (sceneName, loadMode, extraBindings, containerMode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_5(
        &mut self,
        sceneIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        1usize,
                    >("LoadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadSceneAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> =
            unsafe { cordl_method_info.invoke_unchecked(self, (sceneIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode6(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::SceneManagement::LoadSceneMode),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        2usize,
                    >("LoadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadSceneAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> =
            unsafe { cordl_method_info.invoke_unchecked(self, (sceneIndex, loadMode))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode_Action_1_7(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>, 3usize>(
                        "LoadSceneAsync",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadSceneAsync",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            cordl_method_info.invoke_unchecked(self, (sceneIndex, loadMode, extraBindings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode_Action_1_LoadSceneRelationship8(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        crate::Zenject::LoadSceneRelationship,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>, 4usize>(
                        "LoadSceneAsync",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadSceneAsync",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (sceneIndex, loadMode, extraBindings, containerMode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode_Action_1_LoadSceneRelationship_Action_1_9(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        crate::Zenject::LoadSceneRelationship,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>, 5usize>(
                        "LoadSceneAsync",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadSceneAsync",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    sceneIndex,
                    loadMode,
                    extraBindings,
                    containerMode,
                    extraBindingsLate,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneFromAddressablesAsync(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
        extraBindingsEarly: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::SceneManagement::LoadSceneMode,
                            bool,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                >,
                            >,
                            crate::Zenject::LoadSceneRelationship,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                >,
                            >,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                        >,
                        8usize,
                    >("LoadSceneFromAddressablesAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadSceneFromAddressablesAsync", 8usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString0(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadScene", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sceneName))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneMode1(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                    ), quest_hook::libil2cpp::Void, 2usize>("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadScene",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sceneName, loadMode))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneMode_Action_1_2(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadScene",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (sceneName, loadMode, extraBindings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneMode_Action_1_LoadSceneRelationship3(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        crate::Zenject::LoadSceneRelationship,
                    ), quest_hook::libil2cpp::Void, 4usize>("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadScene",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (sceneName, loadMode, extraBindings, containerMode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneMode_Action_1_LoadSceneRelationship_Action_1_4(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        crate::Zenject::LoadSceneRelationship,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadScene",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    sceneName,
                    loadMode,
                    extraBindings,
                    containerMode,
                    extraBindingsLate,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_5(
        &mut self,
        sceneIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadScene",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sceneIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode6(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::SceneManagement::LoadSceneMode),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadScene", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sceneIndex, loadMode))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode_Action_1_7(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadScene",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (sceneIndex, loadMode, extraBindings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode_Action_1_LoadSceneRelationship8(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        crate::Zenject::LoadSceneRelationship,
                    ), quest_hook::libil2cpp::Void, 4usize>("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadScene",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (sceneIndex, loadMode, extraBindings, containerMode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode_Action_1_LoadSceneRelationship_Action_1_9(
        &mut self,
        sceneIndex: i32,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        crate::Zenject::LoadSceneRelationship,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadScene",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    sceneIndex,
                    loadMode,
                    extraBindings,
                    containerMode,
                    extraBindingsLate,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        sceneRoot: quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
        projectKernel: quest_hook::libil2cpp::Gc<crate::Zenject::ProjectKernel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sceneRoot, projectKernel))?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareForLoadScene(
        &mut self,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        extraBindingsEarly: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        extraBindings: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        extraBindingsLate: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>>,
        >,
        containerMode: crate::Zenject::LoadSceneRelationship,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::SceneManagement::LoadSceneMode,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            >,
                        >,
                        crate::Zenject::LoadSceneRelationship,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "PrepareForLoadScene"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareForLoadScene",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    loadMode,
                    extraBindingsEarly,
                    extraBindings,
                    extraBindingsLate,
                    containerMode,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                    >), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 1usize>(
                        "__zenCreate",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "__zenCreate",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked((), (P_0))? };
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
                        0usize,
                    >("__zenCreateInjectTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "__zenCreateInjectTypeInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sceneRoot: quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
        projectKernel: quest_hook::libil2cpp::Gc<crate::Zenject::ProjectKernel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
                        quest_hook::libil2cpp::Gc<crate::Zenject::ProjectKernel>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sceneRoot, projectKernel))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+ZenjectSceneLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ZenjectSceneLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
