#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct MetaRemoteAssetsCatalogUpdater {
    __cordl_parent: crate::System::Object,
    pub _remoteAssetsManager: *mut crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
    pub _scenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _checkForCatalogUpdateOngoingTask: *mut crate::System::Threading::Tasks::Task,
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater => "BGLib.MetaRemoteAssets"
    ."MetaRemoteAssetsCatalogUpdater"
);
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater")]
impl std::ops::Deref for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater {
    type Target = crate::System::Object;
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
    #[cfg(
        feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater+_CheckForCatalogUpdateWithInterval_d__9"
    )]
    pub type _CheckForCatalogUpdateWithInterval_d__9 = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater__CheckForCatalogUpdateWithInterval_d__9;
    #[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsCatalogUpdater+__c")]
    pub type __c = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsCatalogUpdater___c;
    pub fn CheckForCatalogUpdateWithInterval(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("CheckForCatalogUpdateWithInterval", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameSceneChanged(
        &mut self,
        transitionSetupDataSo: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameSceneChanged", (transitionSetupDataSo, container))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        remoteAssetsManager: *mut crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
        scenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (remoteAssetsManager, scenesManager))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        remoteAssetsManager: *mut crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
        scenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (remoteAssetsManager, scenesManager))?;
        Ok(__cordl_ret)
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
