#[cfg(feature = "MultiplayerBigAvatarInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBigAvatarInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _saberManagerInitData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SaberManager_InitData,
    >,
    pub _playerSpecificSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
    >,
}
#[cfg(feature = "MultiplayerBigAvatarInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerBigAvatarInstaller
    => ""."MultiplayerBigAvatarInstaller"
);
#[cfg(feature = "MultiplayerBigAvatarInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerBigAvatarInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBigAvatarInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerBigAvatarInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBigAvatarInstaller")]
impl crate::GlobalNamespace::MultiplayerBigAvatarInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerBigAvatarInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerBigAvatarInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
