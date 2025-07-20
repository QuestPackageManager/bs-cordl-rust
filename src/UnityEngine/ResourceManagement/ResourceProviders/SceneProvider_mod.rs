#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "SceneProvider";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
    )]
    pub type SceneOp = crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
    )]
    pub type UnloadSceneOp = crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProvideScene_LoadSceneMode0(
        &mut self,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadSceneMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                            crate::UnityEngine::SceneManagement::LoadSceneMode,
                            bool,
                            i32,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                        >,
                        5usize,
                    >("ProvideScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProvideScene", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (resourceManager, location, loadSceneMode, activateOnLoad, priority),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProvideScene_LoadSceneParameters1(
        &mut self,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                            crate::UnityEngine::SceneManagement::LoadSceneParameters,
                            bool,
                            i32,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                        >,
                        5usize,
                    >("ProvideScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProvideScene", 5usize
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
                        resourceManager,
                        location,
                        loadSceneParameters,
                        activateOnLoad,
                        priority,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseScene(
        &mut self,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        sceneLoadHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceManager,
                            >,
                            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                                crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                            >,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                        >,
                        2usize,
                    >("ReleaseScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseScene", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (resourceManager, sceneLoadHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_ResourceProviders_ISceneProvider2_ReleaseScene(
        &mut self,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        sceneLoadHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceManager,
                            >,
                            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                                crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                            >,
                            crate::UnityEngine::SceneManagement::UnloadSceneOptions,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                        >,
                        3usize,
                    >(
                        "UnityEngine.ResourceManagement.ResourceProviders.ISceneProvider2.ReleaseScene",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.ResourceProviders.ISceneProvider2.ReleaseScene",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (resourceManager, sceneLoadHandle, unloadOptions),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl AsRef<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl AsMut<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl AsRef<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider")]
impl AsMut<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SceneProvider_SceneOp {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >,
    pub m_ActivateOnLoad: bool,
    pub m_Inst: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    pub m_Location: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    >,
    pub m_LoadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    pub m_Priority: i32,
    pub m_DepOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    >,
    pub m_ResourceManager: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceManager,
    >,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "SceneProvider/SceneOp";
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Destroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Destroy",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDependencies(
        &mut self,
        deps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetDependencies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDependencies", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (deps))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadStatus(
        &mut self,
        visited: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
                        1usize,
                    >("GetDownloadStatus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDownloadStatus", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus = unsafe {
            cordl_method_info.invoke_unchecked(self, (visited))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init_LoadSceneMode0(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadSceneMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
        depOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                            crate::UnityEngine::SceneManagement::LoadSceneMode,
                            bool,
                            i32,
                            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::IList_1<
                                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (location, loadSceneMode, activateOnLoad, priority, depOp),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init_LoadSceneParameters1(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
        depOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                            crate::UnityEngine::SceneManagement::LoadSceneParameters,
                            bool,
                            i32,
                            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::IList_1<
                                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (location, loadSceneParameters, activateOnLoad, priority, depOp),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalLoad(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadingFromBundle: bool,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            crate::UnityEngine::SceneManagement::LoadSceneParameters,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
                        3usize,
                    >("InternalLoad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalLoad", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (path, loadingFromBundle, loadSceneParameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalLoadScene(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadingFromBundle: bool,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                            bool,
                            crate::UnityEngine::SceneManagement::LoadSceneParameters,
                            bool,
                            i32,
                        ),
                        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                        5usize,
                    >("InternalLoadScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalLoadScene", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        location,
                        loadingFromBundle,
                        loadSceneParameters,
                        activateOnLoad,
                        priority,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("InvokeWaitForCompletion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeWaitForCompletion", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rm))?;
        Ok(__cordl_object.into())
    }
    pub fn UnityEngine_ResourceManagement_IUpdateReceiver_Update(
        &mut self,
        unscaledDeltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.ResourceManagement.IUpdateReceiver.Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.IUpdateReceiver.Update",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (unscaledDeltaTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ResourceManagement::ResourceManager,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rm))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DebugName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_DebugName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DebugName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_Progress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Progress", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl AsRef<crate::UnityEngine::ResourceManagement::IUpdateReceiver>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    fn as_ref(&self) -> &crate::UnityEngine::ResourceManagement::IUpdateReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+SceneOp"
)]
impl AsMut<crate::UnityEngine::ResourceManagement::IUpdateReceiver>
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_SceneOp {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::IUpdateReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SceneProvider_UnloadSceneOp {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >,
    pub m_Instance: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    pub m_sceneLoadHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >,
    pub m_UnloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "SceneProvider/UnloadSceneOp";
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        sceneLoadHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
        options: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                                crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                            >,
                            crate::UnityEngine::SceneManagement::UnloadSceneOptions,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (sceneLoadHandle, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("InvokeWaitForCompletion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeWaitForCompletion", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnloadSceneCompleted(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnloadSceneCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnloadSceneCompleted", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneCompletedNoRelease(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnloadSceneCompletedNoRelease")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnloadSceneCompletedNoRelease", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_Progress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Progress", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneProvider+UnloadSceneOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneProvider_UnloadSceneOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
