#[cfg(feature = "LobbyDataModelsManager")]
#[repr(C)]
#[derive(Debug)]
pub struct LobbyDataModelsManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lobbyStateDataModel: *mut crate::GlobalNamespace::ILobbyStateDataModel,
    pub _lobbyPlayersDataModel: *mut crate::GlobalNamespace::ILobbyPlayersDataModel,
    pub _nodePoseSyncStateManager: *mut crate::GlobalNamespace::INodePoseSyncStateManager,
    pub _lobbyPlayerPermissionsModel: *mut crate::GlobalNamespace::LobbyPlayerPermissionsModel,
    pub _lobbyGameStateController: *mut crate::GlobalNamespace::ILobbyGameStateControllerBase,
}
#[cfg(feature = "LobbyDataModelsManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LobbyDataModelsManager => ""
    ."LobbyDataModelsManager"
);
#[cfg(feature = "LobbyDataModelsManager")]
impl std::ops::Deref for crate::GlobalNamespace::LobbyDataModelsManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyDataModelsManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::LobbyDataModelsManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyDataModelsManager")]
impl crate::GlobalNamespace::LobbyDataModelsManager {
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Activate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
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
#[cfg(feature = "LobbyDataModelsManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LobbyDataModelsManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
