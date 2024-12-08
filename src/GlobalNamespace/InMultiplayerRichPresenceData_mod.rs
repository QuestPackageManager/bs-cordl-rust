#[cfg(feature = "InMultiplayerRichPresenceData")]
#[repr(C)]
#[derive(Debug)]
pub struct InMultiplayerRichPresenceData {
    __cordl_parent: crate::System::Object,
    pub _apiName_k__BackingField: *mut crate::System::String,
    pub _localizedDescription_k__BackingField: *mut crate::System::String,
    pub _multiplayerSecret_k__BackingField: *mut crate::System::String,
    pub _atMaxPartySize_k__BackingField: bool,
    pub _canInvite_k__BackingField: bool,
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for InMultiplayerRichPresenceData => ""
    ."InMultiplayerRichPresenceData"
);
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl std::ops::Deref for InMultiplayerRichPresenceData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl std::ops::DerefMut for InMultiplayerRichPresenceData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl InMultiplayerRichPresenceData {
    pub const kInMultiplayerLobbyRichPresenceLocalizationKey: &'static str = "IN_MULTIPLAYER_LOBBY_PRESENCE";
    pub fn set_atMaxPartySize(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atMaxPartySize", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_apiName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_apiName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_canInvite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canInvite", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_canInvite(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_canInvite", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_atMaxPartySize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_atMaxPartySize", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_multiplayerSecret(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiplayerSecret", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_apiName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_apiName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isJoinable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isJoinable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplayerSecret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_multiplayerSecret", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSecret: *mut crate::System::String,
        canInvite: bool,
        atMaxPartySize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSecret, canInvite, atMaxPartySize))?;
        Ok(__cordl_ret)
    }
    pub fn get_localizedDescription(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_localizedDescription", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_localizedDescription(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localizedDescription", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSecret: *mut crate::System::String,
        canInvite: bool,
        atMaxPartySize: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSecret, canInvite, atMaxPartySize))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl quest_hook::libil2cpp::ObjectType for InMultiplayerRichPresenceData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
