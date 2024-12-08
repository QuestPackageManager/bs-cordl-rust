#[cfg(feature = "CannotStartGameReasonMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct CannotStartGameReasonMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "CannotStartGameReasonMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CannotStartGameReasonMethods => ""
    ."CannotStartGameReasonMethods"
);
#[cfg(feature = "CannotStartGameReasonMethods")]
impl std::ops::Deref for CannotStartGameReasonMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CannotStartGameReasonMethods")]
impl std::ops::DerefMut for CannotStartGameReasonMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CannotStartGameReasonMethods")]
impl CannotStartGameReasonMethods {
    pub const kAllPlayersNotInLobby: &'static str = "LABEL_CANT_START_GAME_ALL_PLAYERS_NOT_IN_LOBBY";
    pub const kAllPlayersSpectating: &'static str = "LABEL_CANT_START_GAME_ALL_PLAYERS_SPECTATING";
    pub const kDoNotOwnSong: &'static str = "LABEL_CANT_START_GAME_DO_NOT_OWN_SONG";
    pub const kNoSongSelected: &'static str = "LABEL_CANT_START_GAME_NO_SONG_SELECTED";
}
#[cfg(feature = "CannotStartGameReasonMethods")]
impl quest_hook::libil2cpp::ObjectType for CannotStartGameReasonMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
