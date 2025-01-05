#[cfg(feature = "LobbyGameStateModel")]
#[repr(C)]
#[derive(Debug)]
pub struct LobbyGameStateModel {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub gameStateDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerGameState,
    >,
    pub gameStateDidChangeAlwaysSentEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerGameState,
    >,
    pub _gameState: crate::GlobalNamespace::MultiplayerGameState,
}
#[cfg(feature = "LobbyGameStateModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LobbyGameStateModel => ""
    ."LobbyGameStateModel"
);
#[cfg(feature = "LobbyGameStateModel")]
impl std::ops::Deref for crate::GlobalNamespace::LobbyGameStateModel {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyGameStateModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::LobbyGameStateModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyGameStateModel")]
impl crate::GlobalNamespace::LobbyGameStateModel {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetGameStateWithoutNotification(
        &mut self,
        newGameState: crate::GlobalNamespace::MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameStateWithoutNotification", (newGameState))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGameState_MultiplayerGameState0(
        &mut self,
        newGameState: crate::GlobalNamespace::MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameState", (newGameState))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGameState__cordl_bool1(
        &mut self,
        newGameState: crate::GlobalNamespace::MultiplayerGameState,
        sendNotification: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameState", (newGameState, sendNotification))?;
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
    pub fn add_gameStateDidChangeAlwaysSentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerGameState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStateDidChangeAlwaysSentEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_gameStateDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerGameState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStateDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gameState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerGameState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerGameState = __cordl_object
            .invoke("get_gameState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_gameStateDidChangeAlwaysSentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerGameState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStateDidChangeAlwaysSentEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_gameStateDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerGameState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStateDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LobbyGameStateModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LobbyGameStateModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
