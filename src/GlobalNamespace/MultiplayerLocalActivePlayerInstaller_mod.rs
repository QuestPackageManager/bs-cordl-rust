#[cfg(feature = "MultiplayerLocalActivePlayerInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActivePlayerInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _sceneSetupData: *mut GameplayCoreSceneSetupData,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _optionalAvatarDataSender: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataSender,
}
#[cfg(feature = "MultiplayerLocalActivePlayerInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLocalActivePlayerInstaller => ""
    ."MultiplayerLocalActivePlayerInstaller"
);
#[cfg(feature = "MultiplayerLocalActivePlayerInstaller")]
impl std::ops::Deref for MultiplayerLocalActivePlayerInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerInstaller")]
impl std::ops::DerefMut for MultiplayerLocalActivePlayerInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerInstaller")]
impl MultiplayerLocalActivePlayerInstaller {
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
#[cfg(feature = "MultiplayerLocalActivePlayerInstaller")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLocalActivePlayerInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}