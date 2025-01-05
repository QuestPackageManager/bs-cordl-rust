#[cfg(feature = "MultiplayerLocalPlayerDisconnectHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalPlayerDisconnectHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameplayRpcManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IGameplayRpcManager,
    >,
    pub _multiplayerLevelEndActions: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerLevelEndActionsListener,
    >,
    pub _lobbyPlayerPermissionsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LobbyPlayerPermissionsModel,
    >,
}
#[cfg(feature = "MultiplayerLocalPlayerDisconnectHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLocalPlayerDisconnectHelper => ""
    ."MultiplayerLocalPlayerDisconnectHelper"
);
#[cfg(feature = "MultiplayerLocalPlayerDisconnectHelper")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLocalPlayerDisconnectHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalPlayerDisconnectHelper")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalPlayerDisconnectHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalPlayerDisconnectHelper")]
impl crate::GlobalNamespace::MultiplayerLocalPlayerDisconnectHelper {
    pub const kDisconnectLabel: &'static str = "BUTTON_DISCONNECT";
    pub const kEndGameLabel: &'static str = "BUTTON_END_GAME";
    pub fn Disconnect(
        &mut self,
        playerLevelEndState: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", (playerLevelEndState, levelCompletionResults))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveDisconnectButtonString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ResolveDisconnectButtonString", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "MultiplayerLocalPlayerDisconnectHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalPlayerDisconnectHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
