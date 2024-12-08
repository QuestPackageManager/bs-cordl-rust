#[cfg(feature = "PlayingMultiplayerRichPresenceData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayingMultiplayerRichPresenceData {
    __cordl_parent: crate::GlobalNamespace::InMultiplayerRichPresenceData,
}
#[cfg(feature = "PlayingMultiplayerRichPresenceData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayingMultiplayerRichPresenceData => ""
    ."PlayingMultiplayerRichPresenceData"
);
#[cfg(feature = "PlayingMultiplayerRichPresenceData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayingMultiplayerRichPresenceData {
    type Target = crate::GlobalNamespace::InMultiplayerRichPresenceData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayingMultiplayerRichPresenceData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayingMultiplayerRichPresenceData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayingMultiplayerRichPresenceData")]
impl crate::GlobalNamespace::PlayingMultiplayerRichPresenceData {
    pub const kPlayingMultiplayerLobbyRichPresenceLocalizationKey: &'static str = "PLAYING_MULTIPLAYER_PRESENCE";
    pub fn New(atMaxPartySize: bool) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (atMaxPartySize))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        atMaxPartySize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (atMaxPartySize))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayingMultiplayerRichPresenceData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayingMultiplayerRichPresenceData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
