#[cfg(feature = "LobbyGameStateModel")]
#[repr(C)]
#[derive(Debug)]
pub struct LobbyGameStateModel {
    __cordl_parent: crate::System::Object,
    pub gameStateDidChangeEvent: *mut crate::System::Action_1<MultiplayerGameState>,
    pub gameStateDidChangeAlwaysSentEvent: *mut crate::System::Action_1<
        MultiplayerGameState,
    >,
    pub _gameState: MultiplayerGameState,
}
#[cfg(feature = "LobbyGameStateModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LobbyGameStateModel => ""."LobbyGameStateModel"
);
#[cfg(feature = "LobbyGameStateModel")]
impl std::ops::Deref for LobbyGameStateModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyGameStateModel")]
impl std::ops::DerefMut for LobbyGameStateModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyGameStateModel")]
impl LobbyGameStateModel {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetGameStateWithoutNotification(
        &mut self,
        newGameState: MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameStateWithoutNotification", (newGameState))?;
        Ok(__cordl_ret)
    }
    pub fn SetGameState_MultiplayerGameState0(
        &mut self,
        newGameState: MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameState", (newGameState))?;
        Ok(__cordl_ret)
    }
    pub fn SetGameState__cordl_bool1(
        &mut self,
        newGameState: MultiplayerGameState,
        sendNotification: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameState", (newGameState, sendNotification))?;
        Ok(__cordl_ret)
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
    pub fn add_gameStateDidChangeAlwaysSentEvent(
        &mut self,
        value: *mut crate::System::Action_1<MultiplayerGameState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStateDidChangeAlwaysSentEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_gameStateDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<MultiplayerGameState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStateDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_gameState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<MultiplayerGameState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: MultiplayerGameState = __cordl_object
            .invoke("get_gameState", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_gameStateDidChangeAlwaysSentEvent(
        &mut self,
        value: *mut crate::System::Action_1<MultiplayerGameState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStateDidChangeAlwaysSentEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_gameStateDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<MultiplayerGameState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStateDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LobbyGameStateModel")]
impl quest_hook::libil2cpp::ObjectType for LobbyGameStateModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
