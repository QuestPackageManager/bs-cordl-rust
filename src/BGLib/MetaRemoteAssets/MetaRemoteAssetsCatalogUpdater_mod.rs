#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct MetaRemoteAssetsCatalogUpdater {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _remoteAssetsManager: quest_hook::libil2cpp::Gc<
        crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
    >,
    pub _scenesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameScenesManager,
    >,
    pub _cancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub _checkForCatalogUpdateOngoingTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task,
    >,
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.MetaRemoteAssets";
    const CLASS_NAME: &'static str = "MetaRemoteAssetsCatalogUpdater";
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
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
impl std::ops::Deref for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
impl std::ops::DerefMut
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
impl crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    pub const kGameplaySceneName: &'static str = "GameCore";
    pub const kWaitIntervalInSeconds: i32 = 20i32;
    pub fn CheckForCatalogUpdateWithInterval(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Threading::CancellationToken),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                1usize,
            >("CheckForCatalogUpdateWithInterval")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckForCatalogUpdateWithInterval", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, (cancellationToken)) };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameSceneChanged(
        &mut self,
        sceneTransitionType: crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
        transitionSetupDataSo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("HandleGameSceneChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleGameSceneChanged", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (sceneTransitionType, transitionSetupDataSo, container),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        remoteAssetsManager: quest_hook::libil2cpp::Gc<
            crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
        >,
        scenesManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameScenesManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (remoteAssetsManager, scenesManager))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        remoteAssetsManager: quest_hook::libil2cpp::Gc<
            crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
        >,
        scenesManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameScenesManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameScenesManager>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (remoteAssetsManager, scenesManager))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
impl AsRef<crate::System::IDisposable>
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
impl AsMut<crate::System::IDisposable>
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
impl AsRef<crate::Zenject::IInitializable>
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
impl AsMut<crate::Zenject::IInitializable>
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
