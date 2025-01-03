#[cfg(feature = "PackDefinitionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct PackDefinitionExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PackDefinitionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackDefinitionExtensions => ""
    ."PackDefinitionExtensions"
);
#[cfg(feature = "PackDefinitionExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::PackDefinitionExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::PackDefinitionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionExtensions")]
impl crate::GlobalNamespace::PackDefinitionExtensions {
    #[cfg(feature = "PackDefinitionExtensions+_GetOculusLevelProductPacks_d__5")]
    pub type _GetOculusLevelProductPacks_d__5 = crate::GlobalNamespace::PackDefinitionExtensions__GetOculusLevelProductPacks_d__5;
    #[cfg(feature = "PackDefinitionExtensions+_GetPS4LeaderboardIds_d__3")]
    pub type _GetPS4LeaderboardIds_d__3 = crate::GlobalNamespace::PackDefinitionExtensions__GetPS4LeaderboardIds_d__3;
    #[cfg(feature = "PackDefinitionExtensions+_GetPS4LevelProductPacks_d__7")]
    pub type _GetPS4LevelProductPacks_d__7 = crate::GlobalNamespace::PackDefinitionExtensions__GetPS4LevelProductPacks_d__7;
    #[cfg(feature = "PackDefinitionExtensions+_GetPS5LeaderboardIds_d__4")]
    pub type _GetPS5LeaderboardIds_d__4 = crate::GlobalNamespace::PackDefinitionExtensions__GetPS5LeaderboardIds_d__4;
    #[cfg(feature = "PackDefinitionExtensions+_GetPS5LevelProductPacks_d__8")]
    pub type _GetPS5LevelProductPacks_d__8 = crate::GlobalNamespace::PackDefinitionExtensions__GetPS5LevelProductPacks_d__8;
    #[cfg(feature = "PackDefinitionExtensions+_GetPerceivedLoudnessSOs_d__9")]
    pub type _GetPerceivedLoudnessSOs_d__9 = crate::GlobalNamespace::PackDefinitionExtensions__GetPerceivedLoudnessSOs_d__9;
    #[cfg(feature = "PackDefinitionExtensions+_GetQuestLeaderboardIds_d__0")]
    pub type _GetQuestLeaderboardIds_d__0 = crate::GlobalNamespace::PackDefinitionExtensions__GetQuestLeaderboardIds_d__0;
    #[cfg(feature = "PackDefinitionExtensions+_GetRiftLeaderboardIds_d__1")]
    pub type _GetRiftLeaderboardIds_d__1 = crate::GlobalNamespace::PackDefinitionExtensions__GetRiftLeaderboardIds_d__1;
    #[cfg(feature = "PackDefinitionExtensions+_GetSteamLeaderboardIds_d__2")]
    pub type _GetSteamLeaderboardIds_d__2 = crate::GlobalNamespace::PackDefinitionExtensions__GetSteamLeaderboardIds_d__2;
    #[cfg(feature = "PackDefinitionExtensions+_GetSteamLevelProductPacks_d__6")]
    pub type _GetSteamLevelProductPacks_d__6 = crate::GlobalNamespace::PackDefinitionExtensions__GetSteamLevelProductPacks_d__6;
    pub fn GetOculusLevelProductPacks(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::OculusLevelProductPacksSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::OculusLevelProductPacksSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOculusLevelProductPacks", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPS4LeaderboardIds(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::SonyLeaderboardIdsSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::SonyLeaderboardIdsSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPS4LeaderboardIds", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPS4LevelProductPacks(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PS4LevelProductPacksSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PS4LevelProductPacksSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPS4LevelProductPacks", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPS5LeaderboardIds(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::SonyLeaderboardIdsSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::SonyLeaderboardIdsSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPS5LeaderboardIds", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPS5LevelProductPacks(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PS5LevelProductPacksSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PS5LevelProductPacksSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPS5LevelProductPacks", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPerceivedLoudnessSOs(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PerceivedLoudnessSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PerceivedLoudnessSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPerceivedLoudnessSOs", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQuestLeaderboardIds(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LeaderboardIdsSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LeaderboardIdsSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetQuestLeaderboardIds", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRiftLeaderboardIds(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LeaderboardIdsSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LeaderboardIdsSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRiftLeaderboardIds", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSteamLeaderboardIds(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LeaderboardIdsSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LeaderboardIdsSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSteamLeaderboardIds", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSteamLevelProductPacks(
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::SteamLevelProductPacksSO,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::SteamLevelProductPacksSO,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSteamLevelProductPacks", (packDefinitions))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PackDefinitionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PackDefinitionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
