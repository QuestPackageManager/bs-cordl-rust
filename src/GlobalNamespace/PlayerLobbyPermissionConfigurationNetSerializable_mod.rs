#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerLobbyPermissionConfigurationNetSerializable {
    __cordl_parent: PoolableSerializable,
    pub _userId: *mut crate::System::String,
    pub _isServerOwner: bool,
    pub _hasRecommendBeatmapsPermission: bool,
    pub _hasRecommendGameplayModifiersPermission: bool,
    pub _hasKickVotePermission: bool,
    pub _hasInvitePermission: bool,
}
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerLobbyPermissionConfigurationNetSerializable => ""
    ."PlayerLobbyPermissionConfigurationNetSerializable"
);
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
impl std::ops::Deref for PlayerLobbyPermissionConfigurationNetSerializable {
    type Target = PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
impl std::ops::DerefMut for PlayerLobbyPermissionConfigurationNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
impl PlayerLobbyPermissionConfigurationNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        userId: *mut crate::System::String,
        isServerOwner: bool,
        hasRecommendBeatmapsPermission: bool,
        hasRecommendGameplayModifiersPermission: bool,
        hasKickVotePermission: bool,
        hasInvitePermission: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut PlayerLobbyPermissionConfigurationNetSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerLobbyPermissionConfigurationNetSerializable = __cordl_object
            .invoke(
                "Init",
                (
                    userId,
                    isServerOwner,
                    hasRecommendBeatmapsPermission,
                    hasRecommendGameplayModifiersPermission,
                    hasKickVotePermission,
                    hasInvitePermission,
                ),
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
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
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
    pub fn get_hasRecommendBeatmapsPermission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasRecommendBeatmapsPermission", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasRecommendGameplayModifiersPermission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasRecommendGameplayModifiersPermission", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isServerOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isServerOwner", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for PlayerLobbyPermissionConfigurationNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
