#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneManagerAPIInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.SceneManagement";
    const CLASS_NAME: &'static str = "SceneManagerAPIInternal";
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
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
impl std::ops::Deref for crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
impl std::ops::DerefMut
for crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+SceneManagerAPIInternal")]
impl crate::UnityEngine::SceneManagement::SceneManagerAPIInternal {
    pub fn GetNumScenesInBuildSettings() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("GetNumScenesInBuildSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNumScenesInBuildSettings", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
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
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadSceneAsyncNameIndexInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (sceneName, sceneBuildIndex, parameters, mustCompleteNextFrame),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::SceneManagement::LoadSceneParameters,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        4usize,
                    >("LoadSceneAsyncNameIndexInternal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadSceneAsyncNameIndexInternal_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (sceneName, sceneBuildIndex, parameters, mustCompleteNextFrame),
                )?
        };
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
