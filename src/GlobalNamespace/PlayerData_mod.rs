#[cfg(feature = "PlayerData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _playerId_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _playerName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _shouldShowTutorialPrompt_k__BackingField: bool,
    pub _shouldShow360Warning_k__BackingField: bool,
    pub _agreedToEula_k__BackingField: bool,
    pub _didSelectLanguage_k__BackingField: bool,
    pub _agreedToMultiplayerDisclaimer_k__BackingField: bool,
    pub _didSelectRegionVersion_k__BackingField: i32,
    pub _selectedAvatarSystemTypeId_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _playerAgreements_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerAgreements,
    >,
    pub _lastSelectedBeatmapDifficulty_k__BackingField: crate::GlobalNamespace::BeatmapDifficulty,
    pub _lastSelectedBeatmapCharacteristic_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
    pub _gameplayModifiers_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
    pub _playerSpecificSettings_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSpecificSettings,
    >,
    pub _practiceSettings_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PracticeSettings,
    >,
    pub _playerAllOverallStatsData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerAllOverallStatsData,
    >,
    pub _levelsStatsData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::BeatmapKey,
            *mut crate::GlobalNamespace::PlayerLevelStatsData,
        >,
    >,
    pub _missionsStatsData_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::PlayerMissionStatsData,
        >,
    >,
    pub _showedMissionHelpIds_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _guestPlayerNames_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _colorSchemesSettings_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSchemesSettings,
    >,
    pub _overrideEnvironmentSettings_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OverrideEnvironmentSettings,
    >,
    pub _favoritesLevelIds_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _multiplayerModeSettings_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerModeSettings,
    >,
    pub _userAgeCategory_k__BackingField: crate::GlobalNamespace::UserAgeCategory,
    pub _desiredSensitivityFlag_k__BackingField: crate::GlobalNamespace::PlayerSensitivityFlag,
    pub _currentDlcPromoDisplayCount_k__BackingField: i32,
    pub _currentDlcPromoId_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub favoriteLevelsSetDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub didIncreaseNumberOfGameplaysEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
}
#[cfg(feature = "PlayerData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerData => ""."PlayerData"
);
#[cfg(feature = "PlayerData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerData")]
impl crate::GlobalNamespace::PlayerData {
    pub const kCurrentRegionVersion: i32 = 1i32;
    pub const kMaxGuestPlayers: i32 = 10i32;
    pub fn AddGuestPlayerName(
        &mut self,
        guestPlayerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddGuestPlayerName", (guestPlayerName))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddLevelToFavorites(
        &mut self,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLevelToFavorites", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteAllGuestPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteAllGuestPlayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DidSelectRegion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DidSelectRegion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreatePlayerLevelStatsData_ByRefMut0(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerLevelStatsData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerLevelStatsData,
        > = __cordl_object.invoke("GetOrCreatePlayerLevelStatsData", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreatePlayerLevelStatsData_Il2CppString_BeatmapDifficulty_BeatmapCharacteristicSO1(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerLevelStatsData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerLevelStatsData,
        > = __cordl_object
            .invoke(
                "GetOrCreatePlayerLevelStatsData",
                (levelId, difficulty, beatmapCharacteristic),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreatePlayerMissionStatsData(
        &mut self,
        missionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerMissionStatsData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerMissionStatsData,
        > = __cordl_object.invoke("GetOrCreatePlayerMissionStatsData", (missionId))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncreaseCurrentDlcPromoDisplayCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncreaseCurrentDlcPromoDisplayCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IncreaseNumberOfGameplays(
        &mut self,
        playerLevelStats: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerLevelStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncreaseNumberOfGameplays", (playerLevelStats))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLevelUserFavorite(
        &mut self,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLevelUserFavorite", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mark360WarningAsShown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mark360WarningAsShown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkEulaAsAgreed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkEulaAsAgreed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkHealthAndSafetyAsAgreed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkHealthAndSafetyAsAgreed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkLanguageAsSelected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkLanguageAsSelected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkMultiplayerDisclaimerAsAgreed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkMultiplayerDisclaimerAsAgreed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkPrivacyPolicyAsAgreed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkPrivacyPolicyAsAgreed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkRegionAsSelected(
        &mut self,
        version: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkRegionAsSelected", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkTutorialAsShown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkTutorialAsShown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MissionHelpWasShowed(
        &mut self,
        missionHelp: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionHelpSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MissionHelpWasShowed", (missionHelp))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_BeatmapCharacteristicSO_ColorSchemesSettings_OverrideEnvironmentSettings0(
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lastSelectedBeatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        colorSchemesSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        >,
        overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    playerId,
                    playerName,
                    lastSelectedBeatmapCharacteristic,
                    colorSchemesSettings,
                    overrideEnvironmentSettings,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_Il2CppString_PlayerAgreements_BeatmapDifficulty_BeatmapCharacteristicSO_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_PlayerAllOverallStatsData_List_1_List_1_List_1_List_1_ColorSchemesSettings_OverrideEnvironmentSettings_List_1_MultiplayerModeSettings_i32_Il2CppString_UserAgeCategory_PlayerSensitivityFlag1(
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shouldShowTutorialPrompt: bool,
        shouldShow360Warning: bool,
        agreedToEula: bool,
        didSelectLanguage: bool,
        agreedToMultiplayerDisclaimer: bool,
        didSelectRegionVersion: i32,
        selectedAvatarSystemTypeId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        playerAgreements: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAgreements,
        >,
        lastSelectedBeatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        lastSelectedBeatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        practiceSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        >,
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        >,
        levelsStatsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::PlayerLevelStatsData,
            >,
        >,
        missionsStatsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::PlayerMissionStatsData,
            >,
        >,
        showedMissionHelpIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        guestPlayerNames: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        colorSchemesSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        >,
        overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
        favoritesLevelIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        multiplayerModeSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerModeSettings,
        >,
        currentDlcPromoDisplayCount: i32,
        currentDlcPromoId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        userAgeCategory: crate::GlobalNamespace::UserAgeCategory,
        desiredSensitivityFlag: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    playerId,
                    playerName,
                    shouldShowTutorialPrompt,
                    shouldShow360Warning,
                    agreedToEula,
                    didSelectLanguage,
                    agreedToMultiplayerDisclaimer,
                    didSelectRegionVersion,
                    selectedAvatarSystemTypeId,
                    playerAgreements,
                    lastSelectedBeatmapDifficulty,
                    lastSelectedBeatmapCharacteristic,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    playerAllOverallStatsData,
                    levelsStatsData,
                    missionsStatsData,
                    showedMissionHelpIds,
                    guestPlayerNames,
                    colorSchemesSettings,
                    overrideEnvironmentSettings,
                    favoritesLevelIds,
                    multiplayerModeSettings,
                    currentDlcPromoDisplayCount,
                    currentDlcPromoId,
                    userAgeCategory,
                    desiredSensitivityFlag,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveLevelFromFavorites(
        &mut self,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveLevelFromFavorites", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetDlcPromoDisplayCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetDlcPromoDisplayCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectAvatarSystemTypeId(
        &mut self,
        selectedAvatarSystemTypeId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectAvatarSystemTypeId", (selectedAvatarSystemTypeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGameplayModifiers(
        &mut self,
        newGameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameplayModifiers", (newGameplayModifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLastSelectedBeatmapCharacteristic(
        &mut self,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLastSelectedBeatmapCharacteristic", (beatmapCharacteristic))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLastSelectedBeatmapDifficulty(
        &mut self,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLastSelectedBeatmapDifficulty", (beatmapDifficulty))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMultiplayerModeSettings(
        &mut self,
        multiplayerModeSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerModeSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMultiplayerModeSettings", (multiplayerModeSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNewDlcPromo(
        &mut self,
        dlcPromoId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNewDlcPromo", (dlcPromoId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerSpecificSettings(
        &mut self,
        newPlayerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerSpecificSettings", (newPlayerSpecificSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldForceApplySensitivity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldForceApplySensitivity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPlayerLevelStatsData(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerLevelStatsData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerLevelStatsData,
        > = __cordl_object.invoke("TryGetPlayerLevelStatsData", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn WasMissionHelpShowed(
        &mut self,
        missionHelp: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionHelpSO>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("WasMissionHelpShowed", (missionHelp))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BeatmapCharacteristicSO_ColorSchemesSettings_OverrideEnvironmentSettings0(
        &mut self,
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lastSelectedBeatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        colorSchemesSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        >,
        overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    playerId,
                    playerName,
                    lastSelectedBeatmapCharacteristic,
                    colorSchemesSettings,
                    overrideEnvironmentSettings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_Il2CppString_PlayerAgreements_BeatmapDifficulty_BeatmapCharacteristicSO_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_PlayerAllOverallStatsData_List_1_List_1_List_1_List_1_ColorSchemesSettings_OverrideEnvironmentSettings_List_1_MultiplayerModeSettings_i32_Il2CppString_UserAgeCategory_PlayerSensitivityFlag1(
        &mut self,
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shouldShowTutorialPrompt: bool,
        shouldShow360Warning: bool,
        agreedToEula: bool,
        didSelectLanguage: bool,
        agreedToMultiplayerDisclaimer: bool,
        didSelectRegionVersion: i32,
        selectedAvatarSystemTypeId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        playerAgreements: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAgreements,
        >,
        lastSelectedBeatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        lastSelectedBeatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        practiceSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        >,
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        >,
        levelsStatsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::PlayerLevelStatsData,
            >,
        >,
        missionsStatsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::PlayerMissionStatsData,
            >,
        >,
        showedMissionHelpIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        guestPlayerNames: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        colorSchemesSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        >,
        overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
        favoritesLevelIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        multiplayerModeSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerModeSettings,
        >,
        currentDlcPromoDisplayCount: i32,
        currentDlcPromoId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        userAgeCategory: crate::GlobalNamespace::UserAgeCategory,
        desiredSensitivityFlag: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    playerId,
                    playerName,
                    shouldShowTutorialPrompt,
                    shouldShow360Warning,
                    agreedToEula,
                    didSelectLanguage,
                    agreedToMultiplayerDisclaimer,
                    didSelectRegionVersion,
                    selectedAvatarSystemTypeId,
                    playerAgreements,
                    lastSelectedBeatmapDifficulty,
                    lastSelectedBeatmapCharacteristic,
                    gameplayModifiers,
                    playerSpecificSettings,
                    practiceSettings,
                    playerAllOverallStatsData,
                    levelsStatsData,
                    missionsStatsData,
                    showedMissionHelpIds,
                    guestPlayerNames,
                    colorSchemesSettings,
                    overrideEnvironmentSettings,
                    favoritesLevelIds,
                    multiplayerModeSettings,
                    currentDlcPromoDisplayCount,
                    currentDlcPromoId,
                    userAgeCategory,
                    desiredSensitivityFlag,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didIncreaseNumberOfGameplaysEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didIncreaseNumberOfGameplaysEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_favoriteLevelsSetDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_favoriteLevelsSetDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_agreedToEula(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_agreedToEula", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_agreedToMultiplayerDisclaimer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_agreedToMultiplayerDisclaimer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorSchemesSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSchemesSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        > = __cordl_object.invoke("get_colorSchemesSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentDlcPromoDisplayCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_currentDlcPromoDisplayCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentDlcPromoId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_currentDlcPromoId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_desiredSensitivityFlag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PlayerSensitivityFlag> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PlayerSensitivityFlag = __cordl_object
            .invoke("get_desiredSensitivityFlag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_didSelectLanguage(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_didSelectLanguage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_didSelectRegionVersion(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_didSelectRegionVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_favoritesLevelIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("get_favoritesLevelIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = __cordl_object.invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_guestPlayerNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("get_guestPlayerNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastSelectedBeatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        > = __cordl_object.invoke("get_lastSelectedBeatmapCharacteristic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastSelectedBeatmapDifficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = __cordl_object
            .invoke("get_lastSelectedBeatmapDifficulty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelsStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::BeatmapKey,
                *mut crate::GlobalNamespace::PlayerLevelStatsData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::BeatmapKey,
                *mut crate::GlobalNamespace::PlayerLevelStatsData,
            >,
        > = __cordl_object.invoke("get_levelsStatsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_missionsStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::PlayerMissionStatsData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::PlayerMissionStatsData,
            >,
        > = __cordl_object.invoke("get_missionsStatsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplayerModeSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerModeSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerModeSettings,
        > = __cordl_object.invoke("get_multiplayerModeSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideEnvironmentSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OverrideEnvironmentSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        > = __cordl_object.invoke("get_overrideEnvironmentSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerAgreements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAgreements>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAgreements,
        > = __cordl_object.invoke("get_playerAgreements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerAllOverallStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAllOverallStatsData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        > = __cordl_object.invoke("get_playerAllOverallStatsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_playerId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_playerName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerSpecificSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        > = __cordl_object.invoke("get_playerSpecificSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_practiceSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PracticeSettings,
        > = __cordl_object.invoke("get_practiceSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedAvatarSystemTypeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_selectedAvatarSystemTypeId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shouldShow360Warning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_shouldShow360Warning", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shouldShowTutorialPrompt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_shouldShowTutorialPrompt", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_showedMissionHelpIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("get_showedMissionHelpIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userAgeCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::UserAgeCategory> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::UserAgeCategory = __cordl_object
            .invoke("get_userAgeCategory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didIncreaseNumberOfGameplaysEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didIncreaseNumberOfGameplaysEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_favoriteLevelsSetDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_favoriteLevelsSetDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_agreedToEula(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_agreedToEula", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_agreedToMultiplayerDisclaimer(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_agreedToMultiplayerDisclaimer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colorSchemesSettings(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSchemesSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorSchemesSettings", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_currentDlcPromoDisplayCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentDlcPromoDisplayCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_currentDlcPromoId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentDlcPromoId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_desiredSensitivityFlag(
        &mut self,
        value: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_desiredSensitivityFlag", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_didSelectLanguage(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_didSelectLanguage", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_didSelectRegionVersion(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_didSelectRegionVersion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_favoritesLevelIds(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_favoritesLevelIds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gameplayModifiers(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayModifiers", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_guestPlayerNames(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_guestPlayerNames", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lastSelectedBeatmapCharacteristic(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lastSelectedBeatmapCharacteristic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lastSelectedBeatmapDifficulty(
        &mut self,
        value: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lastSelectedBeatmapDifficulty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_levelsStatsData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::GlobalNamespace::BeatmapKey,
                *mut crate::GlobalNamespace::PlayerLevelStatsData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelsStatsData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_missionsStatsData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::PlayerMissionStatsData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_missionsStatsData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_multiplayerModeSettings(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerModeSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiplayerModeSettings", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overrideEnvironmentSettings(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overrideEnvironmentSettings", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playerAgreements(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAgreements>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerAgreements", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playerAllOverallStatsData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerAllOverallStatsData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playerId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playerName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playerSpecificSettings(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerSpecificSettings", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_practiceSettings(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_practiceSettings", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectedAvatarSystemTypeId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectedAvatarSystemTypeId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_shouldShow360Warning(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shouldShow360Warning", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_shouldShowTutorialPrompt(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shouldShowTutorialPrompt", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showedMissionHelpIds(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showedMissionHelpIds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_userAgeCategory(
        &mut self,
        value: crate::GlobalNamespace::UserAgeCategory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userAgeCategory", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlayerData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
