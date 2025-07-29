#[cfg(
    feature = "cordl_class_UnityEngine+ResourceManagement+ResourceProviders+ISceneProvider2"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ISceneProvider2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+ResourceManagement+ResourceProviders+ISceneProvider2"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "ISceneProvider2";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+ISceneProvider2")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+ISceneProvider2")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+ISceneProvider2")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
    pub fn ReleaseScene(
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
                    >("ReleaseScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseScene", 3usize
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+ResourceManagement+ResourceProviders+ISceneProvider2"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+ISceneProvider2")]
impl AsRef<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider>
for crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+ISceneProvider2")]
impl AsMut<crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider>
for crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider2 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider {
        unsafe { std::mem::transmute(self) }
    }
}
