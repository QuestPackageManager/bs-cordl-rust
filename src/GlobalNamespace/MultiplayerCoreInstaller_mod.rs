#[cfg(feature = "MultiplayerCoreInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerCoreInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _scoreSyncStateManagerPrefab: *mut ScoreSyncStateManager,
    pub _multiplayerBadgesModel: *mut MultiplayerBadgesModelSO,
    pub _sceneSetupData: *mut GameplayCoreSceneSetupData,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
}
#[cfg(feature = "MultiplayerCoreInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerCoreInstaller => ""
    ."MultiplayerCoreInstaller"
);
#[cfg(feature = "MultiplayerCoreInstaller")]
impl std::ops::Deref for MultiplayerCoreInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerCoreInstaller")]
impl std::ops::DerefMut for MultiplayerCoreInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerCoreInstaller")]
impl MultiplayerCoreInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerCoreInstaller")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerCoreInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
