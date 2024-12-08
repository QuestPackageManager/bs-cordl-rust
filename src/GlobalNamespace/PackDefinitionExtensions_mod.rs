#[cfg(feature = "PackDefinitionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct PackDefinitionExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "PackDefinitionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PackDefinitionExtensions => ""
    ."PackDefinitionExtensions"
);
#[cfg(feature = "PackDefinitionExtensions")]
impl std::ops::Deref for PackDefinitionExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionExtensions")]
impl std::ops::DerefMut for PackDefinitionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionExtensions")]
impl PackDefinitionExtensions {
    #[cfg(feature = "PackDefinitionExtensions+_GetRiftLeaderboardIds_d__1")]
    pub type _GetRiftLeaderboardIds_d__1 = crate::GlobalNamespace::PackDefinitionExtensions__GetRiftLeaderboardIds_d__1;
    #[cfg(feature = "PackDefinitionExtensions+_GetPerceivedLoudnessSOs_d__9")]
    pub type _GetPerceivedLoudnessSOs_d__9 = crate::GlobalNamespace::PackDefinitionExtensions__GetPerceivedLoudnessSOs_d__9;
    #[cfg(feature = "PackDefinitionExtensions+_GetPS5LevelProductPacks_d__8")]
    pub type _GetPS5LevelProductPacks_d__8 = crate::GlobalNamespace::PackDefinitionExtensions__GetPS5LevelProductPacks_d__8;
    #[cfg(feature = "PackDefinitionExtensions+_GetPS4LevelProductPacks_d__7")]
    pub type _GetPS4LevelProductPacks_d__7 = crate::GlobalNamespace::PackDefinitionExtensions__GetPS4LevelProductPacks_d__7;
    #[cfg(feature = "PackDefinitionExtensions+_GetQuestLeaderboardIds_d__0")]
    pub type _GetQuestLeaderboardIds_d__0 = crate::GlobalNamespace::PackDefinitionExtensions__GetQuestLeaderboardIds_d__0;
    #[cfg(feature = "PackDefinitionExtensions+_GetPS4LeaderboardIds_d__3")]
    pub type _GetPS4LeaderboardIds_d__3 = crate::GlobalNamespace::PackDefinitionExtensions__GetPS4LeaderboardIds_d__3;
    #[cfg(feature = "PackDefinitionExtensions+_GetPS5LeaderboardIds_d__4")]
    pub type _GetPS5LeaderboardIds_d__4 = crate::GlobalNamespace::PackDefinitionExtensions__GetPS5LeaderboardIds_d__4;
    #[cfg(feature = "PackDefinitionExtensions+_GetOculusLevelProductPacks_d__5")]
    pub type _GetOculusLevelProductPacks_d__5 = crate::GlobalNamespace::PackDefinitionExtensions__GetOculusLevelProductPacks_d__5;
    #[cfg(feature = "PackDefinitionExtensions+_GetSteamLeaderboardIds_d__2")]
    pub type _GetSteamLeaderboardIds_d__2 = crate::GlobalNamespace::PackDefinitionExtensions__GetSteamLeaderboardIds_d__2;
    #[cfg(feature = "PackDefinitionExtensions+_GetSteamLevelProductPacks_d__6")]
    pub type _GetSteamLevelProductPacks_d__6 = crate::GlobalNamespace::PackDefinitionExtensions__GetSteamLevelProductPacks_d__6;
}
#[cfg(feature = "PackDefinitionExtensions")]
impl quest_hook::libil2cpp::ObjectType for PackDefinitionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
