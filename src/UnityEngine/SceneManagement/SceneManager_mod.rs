#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SceneManagement::SceneManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.SceneManagement";
    const CLASS_NAME: &'static str = "SceneManager";
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
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
impl std::ops::Deref for crate::UnityEngine::SceneManagement::SceneManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
impl std::ops::DerefMut for crate::UnityEngine::SceneManagement::SceneManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManager")]
impl crate::UnityEngine::SceneManagement::SceneManager {
    pub fn GetActiveScene() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SceneManagement::Scene,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::SceneManagement::Scene,
                        0usize,
                    >("GetActiveScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetActiveScene", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveScene_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::SceneManagement::Scene,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetActiveScene_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetActiveScene_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSceneAt(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32),
                        crate::UnityEngine::SceneManagement::Scene,
                        1usize,
                    >("GetSceneAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSceneAt", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = unsafe {
            method.invoke_unchecked((), (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSceneAt_Injected(
        index: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::SceneManagement::Scene,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GetSceneAt_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSceneAt_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (index, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSceneByName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::SceneManagement::Scene,
                        1usize,
                    >("GetSceneByName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSceneByName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = unsafe {
            method.invoke_unchecked((), (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSceneByName_Injected(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::SceneManagement::Scene,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GetSceneByName_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSceneByName_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ActiveSceneChanged(
        previousActiveScene: crate::UnityEngine::SceneManagement::Scene,
        newActiveScene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::SceneManagement::Scene,
                            crate::UnityEngine::SceneManagement::Scene,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Internal_ActiveSceneChanged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Internal_ActiveSceneChanged", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (previousActiveScene, newActiveScene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SceneLoaded(
        scene: crate::UnityEngine::SceneManagement::Scene,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::SceneManagement::Scene,
                            crate::UnityEngine::SceneManagement::LoadSceneMode,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Internal_SceneLoaded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Internal_SceneLoaded", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (scene, mode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SceneUnloaded(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::SceneManagement::Scene),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Internal_SceneUnloaded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Internal_SceneUnloaded", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (scene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFirstScene_Internal(
        _cordl_async: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (bool),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        1usize,
                    >("LoadFirstScene_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadFirstScene_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (_cordl_async))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            crate::UnityEngine::SceneManagement::LoadSceneParameters,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        4usize,
                    >("LoadSceneAsyncNameIndexInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadSceneAsyncNameIndexInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (sceneName, sceneBuildIndex, parameters, mustCompleteNextFrame),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString3(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        1usize,
                    >("LoadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadSceneAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (sceneName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneMode2(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::SceneManagement::LoadSceneMode,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        2usize,
                    >("LoadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadSceneAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (sceneName, mode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppString_LoadSceneParameters4(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::SceneManagement::LoadSceneParameters,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        2usize,
                    >("LoadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadSceneAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (sceneName, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneMode0(
        sceneBuildIndex: i32,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::UnityEngine::SceneManagement::LoadSceneMode),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        2usize,
                    >("LoadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadSceneAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (sceneBuildIndex, mode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_i32_LoadSceneParameters1(
        sceneBuildIndex: i32,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::UnityEngine::SceneManagement::LoadSceneParameters),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        2usize,
                    >("LoadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadSceneAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (sceneBuildIndex, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString1(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadScene", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sceneName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneMode0(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::SceneManagement::LoadSceneMode,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadScene", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sceneName, mode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppString_LoadSceneParameters2(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::SceneManagement::LoadSceneParameters,
                        ),
                        crate::UnityEngine::SceneManagement::Scene,
                        2usize,
                    >("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadScene", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = unsafe {
            method.invoke_unchecked((), (sceneName, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneMode3(
        sceneBuildIndex: i32,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::UnityEngine::SceneManagement::LoadSceneMode),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadScene", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sceneBuildIndex, mode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_i32_LoadSceneParameters4(
        sceneBuildIndex: i32,
        parameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::UnityEngine::SceneManagement::LoadSceneParameters),
                        crate::UnityEngine::SceneManagement::Scene,
                        2usize,
                    >("LoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadScene", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = unsafe {
            method.invoke_unchecked((), (sceneBuildIndex, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveGameObjectToScene(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            crate::UnityEngine::SceneManagement::Scene,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("MoveGameObjectToScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MoveGameObjectToScene", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (go, scene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveGameObjectToScene_Injected(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        scene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::SceneManagement::Scene,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("MoveGameObjectToScene_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MoveGameObjectToScene_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (go, scene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetActiveScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::SceneManagement::Scene),
                        bool,
                        1usize,
                    >("SetActiveScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetActiveScene", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (scene))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetActiveScene_Injected(
        scene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::SceneManagement::Scene,
                        >),
                        bool,
                        1usize,
                    >("SetActiveScene_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetActiveScene_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (scene))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsyncInternal(
        scene: crate::UnityEngine::SceneManagement::Scene,
        options: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::SceneManagement::Scene,
                            crate::UnityEngine::SceneManagement::UnloadSceneOptions,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        2usize,
                    >("UnloadSceneAsyncInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UnloadSceneAsyncInternal", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (scene, options))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::SceneManagement::Scene,
                            >,
                            crate::UnityEngine::SceneManagement::UnloadSceneOptions,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        2usize,
                    >("UnloadSceneAsyncInternal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UnloadSceneAsyncInternal_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (scene, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsync_Scene0(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::SceneManagement::Scene),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        1usize,
                    >("UnloadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UnloadSceneAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (scene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsync_UnloadSceneOptions1(
        scene: crate::UnityEngine::SceneManagement::Scene,
        options: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::SceneManagement::Scene,
                            crate::UnityEngine::SceneManagement::UnloadSceneOptions,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        2usize,
                    >("UnloadSceneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UnloadSceneAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            method.invoke_unchecked((), (scene, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_sceneLoaded(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_2<
                crate::UnityEngine::SceneManagement::Scene,
                crate::UnityEngine::SceneManagement::LoadSceneMode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Events::UnityAction_2<
                                crate::UnityEngine::SceneManagement::Scene,
                                crate::UnityEngine::SceneManagement::LoadSceneMode,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_sceneLoaded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "add_sceneLoaded", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_sceneUnloaded(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<
                crate::UnityEngine::SceneManagement::Scene,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Events::UnityAction_1<
                                crate::UnityEngine::SceneManagement::Scene,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_sceneUnloaded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "add_sceneUnloaded", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneCount() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(), i32, 0usize>("get_sceneCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_sceneCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneCountInBuildSettings() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        i32,
                        0usize,
                    >("get_sceneCountInBuildSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_sceneCountInBuildSettings", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_sceneLoaded(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_2<
                crate::UnityEngine::SceneManagement::Scene,
                crate::UnityEngine::SceneManagement::LoadSceneMode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Events::UnityAction_2<
                                crate::UnityEngine::SceneManagement::Scene,
                                crate::UnityEngine::SceneManagement::LoadSceneMode,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_sceneLoaded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "remove_sceneLoaded", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_sceneUnloaded(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<
                crate::UnityEngine::SceneManagement::Scene,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Events::UnityAction_1<
                                crate::UnityEngine::SceneManagement::Scene,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_sceneUnloaded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "remove_sceneUnloaded", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
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
