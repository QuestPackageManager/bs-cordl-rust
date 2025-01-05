#[cfg(feature = "InMultiplayerRichPresenceData")]
#[repr(C)]
#[derive(Debug)]
pub struct InMultiplayerRichPresenceData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _apiName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _localizedDescription_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _multiplayerSecret_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _atMaxPartySize_k__BackingField: bool,
    pub _canInvite_k__BackingField: bool,
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::InMultiplayerRichPresenceData
    => ""."InMultiplayerRichPresenceData"
);
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl std::ops::Deref for crate::GlobalNamespace::InMultiplayerRichPresenceData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl std::ops::DerefMut for crate::GlobalNamespace::InMultiplayerRichPresenceData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl crate::GlobalNamespace::InMultiplayerRichPresenceData {
    pub const kInMultiplayerLobbyRichPresenceLocalizationKey: &'static str = "IN_MULTIPLAYER_LOBBY_PRESENCE";
    pub fn New(
        multiplayerSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        canInvite: bool,
        atMaxPartySize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSecret, canInvite, atMaxPartySize))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        multiplayerSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        canInvite: bool,
        atMaxPartySize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSecret, canInvite, atMaxPartySize))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_apiName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_apiName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_atMaxPartySize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_atMaxPartySize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canInvite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canInvite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isJoinable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isJoinable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localizedDescription(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_localizedDescription", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplayerSecret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_multiplayerSecret", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_apiName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_apiName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_atMaxPartySize(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atMaxPartySize", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_localizedDescription(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localizedDescription", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_multiplayerSecret(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiplayerSecret", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::InMultiplayerRichPresenceData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl AsRef<crate::GlobalNamespace::IMultiplayerRichPresenceData>
for crate::GlobalNamespace::InMultiplayerRichPresenceData {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerRichPresenceData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl AsMut<crate::GlobalNamespace::IMultiplayerRichPresenceData>
for crate::GlobalNamespace::InMultiplayerRichPresenceData {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMultiplayerRichPresenceData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl AsRef<crate::GlobalNamespace::IRichPresenceData>
for crate::GlobalNamespace::InMultiplayerRichPresenceData {
    fn as_ref(&self) -> &crate::GlobalNamespace::IRichPresenceData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "InMultiplayerRichPresenceData")]
impl AsMut<crate::GlobalNamespace::IRichPresenceData>
for crate::GlobalNamespace::InMultiplayerRichPresenceData {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IRichPresenceData {
        unsafe { std::mem::transmute(self) }
    }
}
