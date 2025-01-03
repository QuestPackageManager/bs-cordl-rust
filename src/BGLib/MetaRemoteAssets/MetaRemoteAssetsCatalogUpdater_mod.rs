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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater => "BGLib.MetaRemoteAssets"
    ."MetaRemoteAssetsCatalogUpdater"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("CheckForCatalogUpdateWithInterval", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameSceneChanged",
                (sceneTransitionType, transitionSetupDataSo, container),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (remoteAssetsManager, scenesManager))?;
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
