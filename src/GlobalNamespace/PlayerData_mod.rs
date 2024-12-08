#[cfg(feature = "PlayerData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerData {
    __cordl_parent: crate::System::Object,
    pub _playerId_k__BackingField: *mut crate::System::String,
    pub _playerName_k__BackingField: *mut crate::System::String,
    pub _shouldShowTutorialPrompt_k__BackingField: bool,
    pub _shouldShow360Warning_k__BackingField: bool,
    pub _agreedToEula_k__BackingField: bool,
    pub _didSelectLanguage_k__BackingField: bool,
    pub _agreedToMultiplayerDisclaimer_k__BackingField: bool,
    pub _didSelectRegionVersion_k__BackingField: i32,
    pub _selectedAvatarSystemTypeId_k__BackingField: *mut crate::System::String,
    pub _playerAgreements_k__BackingField: *mut PlayerAgreements,
    pub _lastSelectedBeatmapDifficulty_k__BackingField: BeatmapDifficulty,
    pub _lastSelectedBeatmapCharacteristic_k__BackingField: *mut BeatmapCharacteristicSO,
    pub _gameplayModifiers_k__BackingField: *mut GameplayModifiers,
    pub _playerSpecificSettings_k__BackingField: *mut PlayerSpecificSettings,
    pub _practiceSettings_k__BackingField: *mut PracticeSettings,
    pub _playerAllOverallStatsData_k__BackingField: *mut PlayerAllOverallStatsData,
    pub _levelsStatsData_k__BackingField: *mut crate::System::Collections::Generic::Dictionary_2<
        BeatmapKey,
        *mut PlayerLevelStatsData,
    >,
    pub _missionsStatsData_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut PlayerMissionStatsData,
    >,
    pub _showedMissionHelpIds_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _guestPlayerNames_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _colorSchemesSettings_k__BackingField: *mut ColorSchemesSettings,
    pub _overrideEnvironmentSettings_k__BackingField: *mut OverrideEnvironmentSettings,
    pub _favoritesLevelIds_k__BackingField: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _multiplayerModeSettings_k__BackingField: *mut MultiplayerModeSettings,
    pub _userAgeCategory_k__BackingField: UserAgeCategory,
    pub _desiredSensitivityFlag_k__BackingField: PlayerSensitivityFlag,
    pub _currentDlcPromoDisplayCount_k__BackingField: i32,
    pub _currentDlcPromoId_k__BackingField: *mut crate::System::String,
    pub favoriteLevelsSetDidChangeEvent: *mut crate::System::Action,
    pub didIncreaseNumberOfGameplaysEvent: *mut crate::System::Action,
}
#[cfg(feature = "PlayerData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerData => ""."PlayerData"
);
#[cfg(feature = "PlayerData")]
impl std::ops::Deref for PlayerData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerData")]
impl std::ops::DerefMut for PlayerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerData")]
impl PlayerData {
    pub const kCurrentRegionVersion: i32 = 1i32;
    pub const kMaxGuestPlayers: i32 = 10i32;
    pub fn SetLastSelectedBeatmapDifficulty(
        &mut self,
        beatmapDifficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLastSelectedBeatmapDifficulty", (beatmapDifficulty))?;
        Ok(__cordl_ret)
    }
    pub fn SelectAvatarSystemTypeId(
        &mut self,
        selectedAvatarSystemTypeId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectAvatarSystemTypeId", (selectedAvatarSystemTypeId))?;
        Ok(__cordl_ret)
    }
    pub fn set_multiplayerModeSettings(
        &mut self,
        value: *mut MultiplayerModeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiplayerModeSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetPlayerLevelStatsData(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerLevelStatsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerLevelStatsData = __cordl_object
            .invoke("TryGetPlayerLevelStatsData", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetOrCreatePlayerLevelStatsData_ByRefMut0(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerLevelStatsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerLevelStatsData = __cordl_object
            .invoke("GetOrCreatePlayerLevelStatsData", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetOrCreatePlayerLevelStatsData_String_BeatmapDifficulty_BeatmapCharacteristicSO1(
        &mut self,
        levelId: *mut crate::System::String,
        difficulty: BeatmapDifficulty,
        beatmapCharacteristic: *mut BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerLevelStatsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerLevelStatsData = __cordl_object
            .invoke(
                "GetOrCreatePlayerLevelStatsData",
                (levelId, difficulty, beatmapCharacteristic),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_guestPlayerNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_guestPlayerNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_guestPlayerNames(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_guestPlayerNames", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_colorSchemesSettings(
        &mut self,
        value: *mut ColorSchemesSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorSchemesSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplayerModeSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerModeSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerModeSettings = __cordl_object
            .invoke("get_multiplayerModeSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lastSelectedBeatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapCharacteristicSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapCharacteristicSO = __cordl_object
            .invoke("get_lastSelectedBeatmapCharacteristic", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentDlcPromoDisplayCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_currentDlcPromoDisplayCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOrCreatePlayerMissionStatsData(
        &mut self,
        missionId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerMissionStatsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerMissionStatsData = __cordl_object
            .invoke("GetOrCreatePlayerMissionStatsData", (missionId))?;
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
    pub fn set_currentDlcPromoDisplayCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentDlcPromoDisplayCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Mark360WarningAsShown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mark360WarningAsShown", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkMultiplayerDisclaimerAsAgreed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkMultiplayerDisclaimerAsAgreed", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn AddGuestPlayerName(
        &mut self,
        guestPlayerName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddGuestPlayerName", (guestPlayerName))?;
        Ok(__cordl_ret)
    }
    pub fn get_missionsStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut PlayerMissionStatsData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut PlayerMissionStatsData,
        > = __cordl_object.invoke("get_missionsStatsData", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkEulaAsAgreed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkEulaAsAgreed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorSchemesSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ColorSchemesSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ColorSchemesSettings = __cordl_object
            .invoke("get_colorSchemesSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShouldForceApplySensitivity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldForceApplySensitivity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_didSelectLanguage(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_didSelectLanguage", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_favoritesLevelIds(
        &mut self,
        value: *mut crate::System::Collections::Generic::HashSet_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_favoritesLevelIds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_shouldShow360Warning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_shouldShow360Warning", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_selectedAvatarSystemTypeId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectedAvatarSystemTypeId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_lastSelectedBeatmapDifficulty(
        &mut self,
        value: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lastSelectedBeatmapDifficulty", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_agreedToEula(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_agreedToEula", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_userAgeCategory(
        &mut self,
        value: UserAgeCategory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userAgeCategory", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didIncreaseNumberOfGameplaysEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didIncreaseNumberOfGameplaysEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_playerAgreements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerAgreements> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerAgreements = __cordl_object
            .invoke("get_playerAgreements", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetDlcPromoDisplayCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetDlcPromoDisplayCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerSpecificSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerSpecificSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerSpecificSettings = __cordl_object
            .invoke("get_playerSpecificSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_practiceSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PracticeSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PracticeSettings = __cordl_object
            .invoke("get_practiceSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_overrideEnvironmentSettings(
        &mut self,
        value: *mut OverrideEnvironmentSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overrideEnvironmentSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn IncreaseNumberOfGameplays(
        &mut self,
        playerLevelStats: *mut PlayerLevelStatsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncreaseNumberOfGameplays", (playerLevelStats))?;
        Ok(__cordl_ret)
    }
    pub fn get_levelsStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            BeatmapKey,
            *mut PlayerLevelStatsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            BeatmapKey,
            *mut PlayerLevelStatsData,
        > = __cordl_object.invoke("get_levelsStatsData", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn MarkHealthAndSafetyAsAgreed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkHealthAndSafetyAsAgreed", ())?;
        Ok(__cordl_ret)
    }
    pub fn WasMissionHelpShowed(
        &mut self,
        missionHelp: *mut MissionHelpSO,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("WasMissionHelpShowed", (missionHelp))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerSpecificSettings(
        &mut self,
        value: *mut PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerSpecificSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_favoriteLevelsSetDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_favoriteLevelsSetDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetLastSelectedBeatmapCharacteristic(
        &mut self,
        beatmapCharacteristic: *mut BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLastSelectedBeatmapCharacteristic", (beatmapCharacteristic))?;
        Ok(__cordl_ret)
    }
    pub fn SetMultiplayerModeSettings(
        &mut self,
        multiplayerModeSettings: *mut MultiplayerModeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMultiplayerModeSettings", (multiplayerModeSettings))?;
        Ok(__cordl_ret)
    }
    pub fn get_favoritesLevelIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::HashSet_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::HashSet_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_favoritesLevelIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_didSelectRegionVersion(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_didSelectRegionVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_currentDlcPromoId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentDlcPromoId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_practiceSettings(
        &mut self,
        value: *mut PracticeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_practiceSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_overrideEnvironmentSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut OverrideEnvironmentSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OverrideEnvironmentSettings = __cordl_object
            .invoke("get_overrideEnvironmentSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_levelsStatsData(
        &mut self,
        value: *mut crate::System::Collections::Generic::Dictionary_2<
            BeatmapKey,
            *mut PlayerLevelStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelsStatsData", (value))?;
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
    pub fn MarkTutorialAsShown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkTutorialAsShown", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_playerAllOverallStatsData(
        &mut self,
        value: *mut PlayerAllOverallStatsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerAllOverallStatsData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerAgreements(
        &mut self,
        value: *mut PlayerAgreements,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerAgreements", (value))?;
        Ok(__cordl_ret)
    }
    pub fn MissionHelpWasShowed(
        &mut self,
        missionHelp: *mut MissionHelpSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MissionHelpWasShowed", (missionHelp))?;
        Ok(__cordl_ret)
    }
    pub fn remove_favoriteLevelsSetDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_favoriteLevelsSetDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_showedMissionHelpIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_showedMissionHelpIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_showedMissionHelpIds(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showedMissionHelpIds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_desiredSensitivityFlag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<PlayerSensitivityFlag> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: PlayerSensitivityFlag = __cordl_object
            .invoke("get_desiredSensitivityFlag", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedAvatarSystemTypeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_selectedAvatarSystemTypeId", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeleteAllGuestPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteAllGuestPlayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetNewDlcPromo(
        &mut self,
        dlcPromoId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNewDlcPromo", (dlcPromoId))?;
        Ok(__cordl_ret)
    }
    pub fn AddLevelToFavorites(
        &mut self,
        level: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLevelToFavorites", (level))?;
        Ok(__cordl_ret)
    }
    pub fn set_lastSelectedBeatmapCharacteristic(
        &mut self,
        value: *mut BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lastSelectedBeatmapCharacteristic", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut GameplayModifiers> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut GameplayModifiers = __cordl_object
            .invoke("get_gameplayModifiers", ())?;
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
    pub fn set_missionsStatsData(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut PlayerMissionStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_missionsStatsData", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_BeatmapCharacteristicSO_ColorSchemesSettings_OverrideEnvironmentSettings0(
        &mut self,
        playerId: *mut crate::System::String,
        playerName: *mut crate::System::String,
        lastSelectedBeatmapCharacteristic: *mut BeatmapCharacteristicSO,
        colorSchemesSettings: *mut ColorSchemesSettings,
        overrideEnvironmentSettings: *mut OverrideEnvironmentSettings,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_String_PlayerAgreements_BeatmapDifficulty_BeatmapCharacteristicSO_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_PlayerAllOverallStatsData_List_1_List_1_List_1_List_1_ColorSchemesSettings_OverrideEnvironmentSettings_List_1_MultiplayerModeSettings_i32_String_UserAgeCategory_PlayerSensitivityFlag1(
        &mut self,
        playerId: *mut crate::System::String,
        playerName: *mut crate::System::String,
        shouldShowTutorialPrompt: bool,
        shouldShow360Warning: bool,
        agreedToEula: bool,
        didSelectLanguage: bool,
        agreedToMultiplayerDisclaimer: bool,
        didSelectRegionVersion: i32,
        selectedAvatarSystemTypeId: *mut crate::System::String,
        playerAgreements: *mut PlayerAgreements,
        lastSelectedBeatmapDifficulty: BeatmapDifficulty,
        lastSelectedBeatmapCharacteristic: *mut BeatmapCharacteristicSO,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        practiceSettings: *mut PracticeSettings,
        playerAllOverallStatsData: *mut PlayerAllOverallStatsData,
        levelsStatsData: *mut crate::System::Collections::Generic::List_1<
            *mut PlayerLevelStatsData,
        >,
        missionsStatsData: *mut crate::System::Collections::Generic::List_1<
            *mut PlayerMissionStatsData,
        >,
        showedMissionHelpIds: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        guestPlayerNames: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        colorSchemesSettings: *mut ColorSchemesSettings,
        overrideEnvironmentSettings: *mut OverrideEnvironmentSettings,
        favoritesLevelIds: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        multiplayerModeSettings: *mut MultiplayerModeSettings,
        currentDlcPromoDisplayCount: i32,
        currentDlcPromoId: *mut crate::System::String,
        userAgeCategory: UserAgeCategory,
        desiredSensitivityFlag: PlayerSensitivityFlag,
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
        Ok(__cordl_ret)
    }
    pub fn IncreaseCurrentDlcPromoDisplayCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncreaseCurrentDlcPromoDisplayCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveLevelFromFavorites(
        &mut self,
        level: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveLevelFromFavorites", (level))?;
        Ok(__cordl_ret)
    }
    pub fn get_shouldShowTutorialPrompt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_shouldShowTutorialPrompt", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lastSelectedBeatmapDifficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapDifficulty = __cordl_object
            .invoke("get_lastSelectedBeatmapDifficulty", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetGameplayModifiers(
        &mut self,
        newGameplayModifiers: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameplayModifiers", (newGameplayModifiers))?;
        Ok(__cordl_ret)
    }
    pub fn add_didIncreaseNumberOfGameplaysEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didIncreaseNumberOfGameplaysEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_userAgeCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<UserAgeCategory> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: UserAgeCategory = __cordl_object
            .invoke("get_userAgeCategory", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsLevelUserFavorite(
        &mut self,
        level: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLevelUserFavorite", (level))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_agreedToMultiplayerDisclaimer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_agreedToMultiplayerDisclaimer", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_gameplayModifiers(
        &mut self,
        value: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayModifiers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn MarkPrivacyPolicyAsAgreed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkPrivacyPolicyAsAgreed", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkLanguageAsSelected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkLanguageAsSelected", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetPlayerSpecificSettings(
        &mut self,
        newPlayerSpecificSettings: *mut PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerSpecificSettings", (newPlayerSpecificSettings))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentDlcPromoId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_currentDlcPromoId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerAllOverallStatsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerAllOverallStatsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerAllOverallStatsData = __cordl_object
            .invoke("get_playerAllOverallStatsData", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_desiredSensitivityFlag(
        &mut self,
        value: PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_desiredSensitivityFlag", (value))?;
        Ok(__cordl_ret)
    }
    pub fn DidSelectRegion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DidSelectRegion", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_BeatmapCharacteristicSO_ColorSchemesSettings_OverrideEnvironmentSettings0(
        playerId: *mut crate::System::String,
        playerName: *mut crate::System::String,
        lastSelectedBeatmapCharacteristic: *mut BeatmapCharacteristicSO,
        colorSchemesSettings: *mut ColorSchemesSettings,
        overrideEnvironmentSettings: *mut OverrideEnvironmentSettings,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_String_PlayerAgreements_BeatmapDifficulty_BeatmapCharacteristicSO_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_PlayerAllOverallStatsData_List_1_List_1_List_1_List_1_ColorSchemesSettings_OverrideEnvironmentSettings_List_1_MultiplayerModeSettings_i32_String_UserAgeCategory_PlayerSensitivityFlag1(
        playerId: *mut crate::System::String,
        playerName: *mut crate::System::String,
        shouldShowTutorialPrompt: bool,
        shouldShow360Warning: bool,
        agreedToEula: bool,
        didSelectLanguage: bool,
        agreedToMultiplayerDisclaimer: bool,
        didSelectRegionVersion: i32,
        selectedAvatarSystemTypeId: *mut crate::System::String,
        playerAgreements: *mut PlayerAgreements,
        lastSelectedBeatmapDifficulty: BeatmapDifficulty,
        lastSelectedBeatmapCharacteristic: *mut BeatmapCharacteristicSO,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
        practiceSettings: *mut PracticeSettings,
        playerAllOverallStatsData: *mut PlayerAllOverallStatsData,
        levelsStatsData: *mut crate::System::Collections::Generic::List_1<
            *mut PlayerLevelStatsData,
        >,
        missionsStatsData: *mut crate::System::Collections::Generic::List_1<
            *mut PlayerMissionStatsData,
        >,
        showedMissionHelpIds: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        guestPlayerNames: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        colorSchemesSettings: *mut ColorSchemesSettings,
        overrideEnvironmentSettings: *mut OverrideEnvironmentSettings,
        favoritesLevelIds: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        multiplayerModeSettings: *mut MultiplayerModeSettings,
        currentDlcPromoDisplayCount: i32,
        currentDlcPromoId: *mut crate::System::String,
        userAgeCategory: UserAgeCategory,
        desiredSensitivityFlag: PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerData")]
impl quest_hook::libil2cpp::ObjectType for PlayerData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
