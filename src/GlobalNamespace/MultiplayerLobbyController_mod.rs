#[cfg(feature = "MultiplayerLobbyController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLobbyController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _innerCircleRadius: f32,
    pub _minOuterCircleRadius: f32,
    pub _multiplayerLobbyAvatarManager: *mut MultiplayerLobbyAvatarManager,
    pub _multiplayerLobbyCenterStageManager: *mut MultiplayerLobbyCenterStageManager,
    pub _multiplayerLobbyAvatarPlaceManager: *mut MultiplayerLobbyAvatarPlaceManager,
    pub _menuEnvironmentManager: *mut MenuEnvironmentManager,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _optionalAvatarDataSender: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataSender,
    pub _lobbyActivated_k__BackingField: bool,
}
#[cfg(feature = "MultiplayerLobbyController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLobbyController => ""
    ."MultiplayerLobbyController"
);
#[cfg(feature = "MultiplayerLobbyController")]
impl std::ops::Deref for MultiplayerLobbyController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyController")]
impl std::ops::DerefMut for MultiplayerLobbyController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyController")]
impl MultiplayerLobbyController {
    pub fn ActivateMultiplayerLobby(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateMultiplayerLobby", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeactivateMultiplayerLobby(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateMultiplayerLobby", ())?;
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
    pub fn get_lobbyActivated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_lobbyActivated", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_lobbyActivated(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lobbyActivated", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerLobbyController")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLobbyController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
