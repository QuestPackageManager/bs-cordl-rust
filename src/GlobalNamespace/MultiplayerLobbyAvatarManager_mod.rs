#[cfg(feature = "MultiplayerLobbyAvatarManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLobbyAvatarManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lobbyStateDataModel: *mut crate::GlobalNamespace::ILobbyStateDataModel,
    pub _avatarControllerFactory: *mut crate::GlobalNamespace::MultiplayerLobbyAvatarController_Factory,
    pub _innerCircleRadius: f32,
    pub _minOuterCircleRadius: f32,
    pub _playerIdToAvatarMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::MultiplayerLobbyAvatarController,
    >,
    pub _inProgressDespawnAnimations: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::GlobalNamespace::MultiplayerLobbyAvatarController,
    >,
}
#[cfg(feature = "MultiplayerLobbyAvatarManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerLobbyAvatarManager
    => ""."MultiplayerLobbyAvatarManager"
);
#[cfg(feature = "MultiplayerLobbyAvatarManager")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLobbyAvatarManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyAvatarManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLobbyAvatarManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyAvatarManager")]
impl crate::GlobalNamespace::MultiplayerLobbyAvatarManager {
    #[cfg(feature = "MultiplayerLobbyAvatarManager+_RemovePlayerAndDestroy_d__13")]
    pub type _RemovePlayerAndDestroy_d__13 = crate::GlobalNamespace::MultiplayerLobbyAvatarManager__RemovePlayerAndDestroy_d__13;
    pub fn ActivateMultiplayerLobbyAvatarManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateMultiplayerLobbyAvatarManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPlayer(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPlayer", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeactivateMultiplayerLobbyAvatarManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateMultiplayerLobbyAvatarManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyStateDataModelPlayerConnected(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyStateDataModelPlayerConnected", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLobbyStateDataModelPlayerDisconnected(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyStateDataModelPlayerDisconnected", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        innerCircleRadius: f32,
        minOuterCircleRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (innerCircleRadius, minOuterCircleRadius))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemovePlayer(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePlayer", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePlayerAndDestroy(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        multiplayerAvatar: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLobbyAvatarController,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke("RemovePlayerAndDestroy", (userId, multiplayerAvatar))?;
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
#[cfg(feature = "MultiplayerLobbyAvatarManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLobbyAvatarManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
