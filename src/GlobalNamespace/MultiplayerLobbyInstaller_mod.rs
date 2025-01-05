#[cfg(feature = "MultiplayerLobbyInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLobbyInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _multiplayerLobbyAvatarControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLobbyAvatarController,
    >,
    pub _multiplayerAvatarPlacePrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLobbyAvatarPlace,
    >,
}
#[cfg(feature = "MultiplayerLobbyInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerLobbyInstaller => ""
    ."MultiplayerLobbyInstaller"
);
#[cfg(feature = "MultiplayerLobbyInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLobbyInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLobbyInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyInstaller")]
impl crate::GlobalNamespace::MultiplayerLobbyInstaller {
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
#[cfg(feature = "MultiplayerLobbyInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLobbyInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
