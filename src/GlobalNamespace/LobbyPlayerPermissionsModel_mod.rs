#[cfg(feature = "LobbyPlayerPermissionsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct LobbyPlayerPermissionsModel {
    __cordl_parent: crate::System::Object,
    pub _menuRpcManager: *mut crate::GlobalNamespace::IMenuRpcManager,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _isPartyOwner_k__BackingField: bool,
    pub _hasRecommendBeatmapPermission_k__BackingField: bool,
    pub _hasRecommendModifiersPermission_k__BackingField: bool,
    pub _hasKickVotePermission_k__BackingField: bool,
    pub _hasInvitePermission_k__BackingField: bool,
    pub permissionsChangedEvent: *mut crate::System::Action,
}
#[cfg(feature = "LobbyPlayerPermissionsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LobbyPlayerPermissionsModel =>
    ""."LobbyPlayerPermissionsModel"
);
#[cfg(feature = "LobbyPlayerPermissionsModel")]
impl std::ops::Deref for crate::GlobalNamespace::LobbyPlayerPermissionsModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyPlayerPermissionsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::LobbyPlayerPermissionsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyPlayerPermissionsModel")]
impl crate::GlobalNamespace::LobbyPlayerPermissionsModel {
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Activate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMenuRpcManagerSetPlayersPermissionConfiguration(
        &mut self,
        userId: *mut crate::System::String,
        playersLobbyPermissionConfiguration: *mut crate::GlobalNamespace::PlayersLobbyPermissionConfigurationNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerSetPlayersPermissionConfiguration",
                (userId, playersLobbyPermissionConfiguration),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetPlayerPermissions(
        &mut self,
        isPartyOwner: bool,
        hasRecommendBeatmapPermission: bool,
        hasRecommendModifiersPermission: bool,
        hasKickVotePermission: bool,
        hasInvitePermission: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetPlayerPermissions",
                (
                    isPartyOwner,
                    hasRecommendBeatmapPermission,
                    hasRecommendModifiersPermission,
                    hasKickVotePermission,
                    hasInvitePermission,
                ),
            )?;
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
    pub fn add_permissionsChangedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_permissionsChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_hasInvitePermission(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasInvitePermission", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasKickVotePermission(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasKickVotePermission", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasRecommendBeatmapPermission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasRecommendBeatmapPermission", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasRecommendModifiersPermission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasRecommendModifiersPermission", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isPartyOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPartyOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_permissionsChangedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_permissionsChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hasInvitePermission(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasInvitePermission", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hasKickVotePermission(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasKickVotePermission", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hasRecommendBeatmapPermission(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasRecommendBeatmapPermission", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hasRecommendModifiersPermission(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasRecommendModifiersPermission", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isPartyOwner(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isPartyOwner", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LobbyPlayerPermissionsModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LobbyPlayerPermissionsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
