#[cfg(feature = "CannotStartGameReasonMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct CannotStartGameReasonMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "CannotStartGameReasonMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::CannotStartGameReasonMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CannotStartGameReasonMethods";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "CannotStartGameReasonMethods")]
impl std::ops::Deref for crate::GlobalNamespace::CannotStartGameReasonMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CannotStartGameReasonMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::CannotStartGameReasonMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CannotStartGameReasonMethods")]
impl crate::GlobalNamespace::CannotStartGameReasonMethods {
    pub const kAllPlayersNotInLobby: &'static str = "LABEL_CANT_START_GAME_ALL_PLAYERS_NOT_IN_LOBBY";
    pub const kAllPlayersSpectating: &'static str = "LABEL_CANT_START_GAME_ALL_PLAYERS_SPECTATING";
    pub const kDoNotOwnSong: &'static str = "LABEL_CANT_START_GAME_DO_NOT_OWN_SONG";
    pub const kNoSongSelected: &'static str = "LABEL_CANT_START_GAME_NO_SONG_SELECTED";
    pub fn LocalizedKey(
        cannotStartGameReason: crate::GlobalNamespace::CannotStartGameReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocalizedKey", (cannotStartGameReason))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CannotStartGameReasonMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CannotStartGameReasonMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
