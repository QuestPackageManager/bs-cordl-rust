#[cfg(feature = "LeaderboardPlayerInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardPlayerInfo {
    __cordl_parent: crate::System::Object,
    pub serverKey: *mut crate::System::String,
    pub _playerId_k__BackingField: *mut crate::System::String,
    pub _playerName_k__BackingField: *mut crate::System::String,
    pub _playerKey_k__BackingField: *mut crate::System::String,
    pub _authType_k__BackingField: *mut crate::System::String,
    pub _playerFriends_k__BackingField: *mut crate::System::String,
    pub _succeeded_k__BackingField: bool,
}
#[cfg(feature = "LeaderboardPlayerInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LeaderboardPlayerInfo => ""."LeaderboardPlayerInfo"
);
#[cfg(feature = "LeaderboardPlayerInfo")]
impl std::ops::Deref for LeaderboardPlayerInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardPlayerInfo")]
impl std::ops::DerefMut for LeaderboardPlayerInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardPlayerInfo")]
impl LeaderboardPlayerInfo {
    pub fn New(
        succeeded: bool,
        playerId: *mut crate::System::String,
        playerName: *mut crate::System::String,
        playerKey: *mut crate::System::String,
        authType: *mut crate::System::String,
        playerFriends: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (succeeded, playerId, playerName, playerKey, authType, playerFriends),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        succeeded: bool,
        playerId: *mut crate::System::String,
        playerName: *mut crate::System::String,
        playerKey: *mut crate::System::String,
        authType: *mut crate::System::String,
        playerFriends: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (succeeded, playerId, playerName, playerKey, authType, playerFriends),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_authType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_authType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerFriends(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_playerFriends", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_playerId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_playerKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_playerName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_succeeded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_succeeded", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_authType(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_authType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerFriends(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerFriends", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerKey(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_succeeded(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_succeeded", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LeaderboardPlayerInfo")]
impl quest_hook::libil2cpp::ObjectType for LeaderboardPlayerInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
