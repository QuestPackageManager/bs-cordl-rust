#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSpecificSettings_ArcVisibilityTypeSaveData {
    High = 3i32,
    Low = 1i32,
    None = 0i32,
    Standard = 2i32,
}
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSpecificSettings_ArcVisibilityTypeSaveData => ""
    ."PlayerSaveData/PlayerSpecificSettings/ArcVisibilityTypeSaveData"
);
#[cfg(feature = "PlayerSaveData+ColorScheme")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_ColorScheme {
    __cordl_parent: crate::System::Object,
    pub colorSchemeId: *mut crate::System::String,
    pub saberAColor: crate::UnityEngine::Color,
    pub saberBColor: crate::UnityEngine::Color,
    pub environmentColor0: crate::UnityEngine::Color,
    pub environmentColor1: crate::UnityEngine::Color,
    pub obstaclesColor: crate::UnityEngine::Color,
    pub environmentColor0Boost: crate::UnityEngine::Color,
    pub environmentColor1Boost: crate::UnityEngine::Color,
}
#[cfg(feature = "PlayerSaveData+ColorScheme")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerSaveData_ColorScheme =>
    ""."PlayerSaveData/ColorScheme"
);
#[cfg(feature = "PlayerSaveData+ColorScheme")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_ColorScheme {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+ColorScheme")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_ColorScheme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+ColorScheme")]
impl crate::GlobalNamespace::PlayerSaveData_ColorScheme {
    pub fn _ctor(
        &mut self,
        colorSchemeId: *mut crate::System::String,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    colorSchemeId,
                    saberAColor,
                    saberBColor,
                    environmentColor0,
                    environmentColor1,
                    obstaclesColor,
                    environmentColor0Boost,
                    environmentColor1Boost,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        colorSchemeId: *mut crate::System::String,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    colorSchemeId,
                    saberAColor,
                    saberBColor,
                    environmentColor0,
                    environmentColor1,
                    obstaclesColor,
                    environmentColor0Boost,
                    environmentColor1Boost,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+ColorScheme")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_ColorScheme {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_ColorSchemesSettings {
    __cordl_parent: crate::System::Object,
    pub overrideDefaultColors: bool,
    pub selectedColorSchemeId: *mut crate::System::String,
    pub colorSchemes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PlayerSaveData_ColorScheme,
    >,
}
#[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_ColorSchemesSettings => ""
    ."PlayerSaveData/ColorSchemesSettings"
);
#[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
impl crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings {
    pub fn _ctor(
        &mut self,
        overrideDefaultColors: bool,
        selectedColorSchemeId: *mut crate::System::String,
        colorSchemes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::PlayerSaveData_ColorScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (overrideDefaultColors, selectedColorSchemeId, colorSchemes),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        overrideDefaultColors: bool,
        selectedColorSchemeId: *mut crate::System::String,
        colorSchemes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::PlayerSaveData_ColorScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (overrideDefaultColors, selectedColorSchemeId, colorSchemes),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplayModifiers_EnabledObstacleType {
    All = 0i32,
    FullHeightOnly = 1i32,
    None = 2i32,
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayModifiers_EnabledObstacleType => ""
    ."PlayerSaveData/GameplayModifiers/EnabledObstacleType"
);
#[cfg(feature = "PlayerSaveData+GameplayModifiers+EnergyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplayModifiers_EnergyType {
    Bar = 0i32,
    Battery = 1i32,
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers+EnergyType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifiers_EnergyType =>
    ""."PlayerSaveData/GameplayModifiers/EnergyType"
);
#[cfg(
    feature = "PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSpecificSettings_EnvironmentEffectsFilterPresetSaveData {
    AllEffects = 0i32,
    NoEffects = 10i32,
    StrobeFilter = 1i32,
}
#[cfg(
    feature = "PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSpecificSettings_EnvironmentEffectsFilterPresetSaveData =>
    ""."PlayerSaveData/PlayerSpecificSettings/EnvironmentEffectsFilterPresetSaveData"
);
#[cfg(feature = "PlayerSaveData+GameplayModifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_GameplayModifiers {
    __cordl_parent: crate::System::Object,
    pub energyType: crate::GlobalNamespace::GameplayModifiers_EnergyType,
    pub instaFail: bool,
    pub failOnSaberClash: bool,
    pub enabledObstacleType: crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
    pub fastNotes: bool,
    pub strictAngles: bool,
    pub disappearingArrows: bool,
    pub ghostNotes: bool,
    pub noBombs: bool,
    pub songSpeed: crate::GlobalNamespace::GameplayModifiers_SongSpeed,
    pub noArrows: bool,
    pub noFailOn0Energy: bool,
    pub proMode: bool,
    pub zenMode: bool,
    pub smallCubes: bool,
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_GameplayModifiers => ""
    ."PlayerSaveData/GameplayModifiers"
);
#[cfg(feature = "PlayerSaveData+GameplayModifiers")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_GameplayModifiers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_GameplayModifiers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers")]
impl crate::GlobalNamespace::PlayerSaveData_GameplayModifiers {
    #[cfg(feature = "PlayerSaveData+GameplayModifiers+SongSpeed")]
    pub type SongSpeed = crate::GlobalNamespace::GameplayModifiers_SongSpeed;
    #[cfg(feature = "PlayerSaveData+GameplayModifiers+EnabledObstacleType")]
    pub type EnabledObstacleType = crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType;
    #[cfg(feature = "PlayerSaveData+GameplayModifiers+EnergyType")]
    pub type EnergyType = crate::GlobalNamespace::GameplayModifiers_EnergyType;
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_GameplayModifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+GuestPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_GuestPlayer {
    __cordl_parent: crate::System::Object,
    pub playerName: *mut crate::System::String,
}
#[cfg(feature = "PlayerSaveData+GuestPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerSaveData_GuestPlayer =>
    ""."PlayerSaveData/GuestPlayer"
);
#[cfg(feature = "PlayerSaveData+GuestPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_GuestPlayer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+GuestPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_GuestPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+GuestPlayer")]
impl crate::GlobalNamespace::PlayerSaveData_GuestPlayer {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+GuestPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_GuestPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+LocalPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_LocalPlayer {
    __cordl_parent: crate::System::Object,
    pub playerId: *mut crate::System::String,
    pub playerName: *mut crate::System::String,
    pub shouldShowTutorialPrompt: bool,
    pub shouldShow360Warning: bool,
    pub agreedToEula: bool,
    pub didSelectLanguage: bool,
    pub agreedToMultiplayerDisclaimer: bool,
    pub didSelectRegionVersion: i32,
    pub selectedAvatarSystemTypeId: *mut crate::System::String,
    pub playerAgreements: *mut crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData,
    pub lastSelectedBeatmapDifficulty: BeatmapDifficulty,
    pub lastSelectedBeatmapCharacteristicName: *mut crate::System::String,
    pub gameplayModifiers: *mut crate::GlobalNamespace::PlayerSaveData_GameplayModifiers,
    pub playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings,
    pub practiceSettings: *mut crate::GlobalNamespace::PlayerSaveData_PracticeSettings,
    pub playerAllOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
    pub levelsStatsData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData,
    >,
    pub missionsStatsData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData,
    >,
    pub showedMissionHelpIds: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub colorSchemesSettings: *mut crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings,
    pub overrideEnvironmentSettings: *mut crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings,
    pub favoritesLevelIds: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub multiplayerModeSettings: *mut crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings,
    pub currentDlcPromoDisplayCount: i32,
    pub currentDlcPromoId: *mut crate::System::String,
    pub userAgeCategory: crate::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData,
    pub desiredSensitivityFlag: crate::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData,
}
#[cfg(feature = "PlayerSaveData+LocalPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerSaveData_LocalPlayer =>
    ""."PlayerSaveData/LocalPlayer"
);
#[cfg(feature = "PlayerSaveData+LocalPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_LocalPlayer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+LocalPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_LocalPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+LocalPlayer")]
impl crate::GlobalNamespace::PlayerSaveData_LocalPlayer {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+LocalPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_LocalPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_MultiplayerModeSettings {
    __cordl_parent: crate::System::Object,
    pub createServerNumberOfPlayers: i32,
    pub quickPlayDifficulty: *mut crate::System::String,
    pub quickPlaySongPackMask: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub quickPlaySongPackMaskSerializedName: *mut crate::System::String,
    pub quickPlayEnableLevelSelection: bool,
}
#[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings => ""
    ."PlayerSaveData/MultiplayerModeSettings"
);
#[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
impl crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSpecificSettings_NoteJumpDurationTypeSettingsSaveData {
    Dynamic = 0i32,
    Static = 1i32,
}
#[cfg(
    feature = "PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSpecificSettings_NoteJumpDurationTypeSettingsSaveData => ""
    ."PlayerSaveData/PlayerSpecificSettings/NoteJumpDurationTypeSettingsSaveData"
);
#[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_OverrideEnvironmentSettings {
    __cordl_parent: crate::System::Object,
    pub overrideEnvironments: bool,
    pub overrideNormalEnvironmentName: *mut crate::System::String,
    pub override360EnvironmentName: *mut crate::System::String,
}
#[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings => ""
    ."PlayerSaveData/OverrideEnvironmentSettings"
);
#[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
impl crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerAgreementsData {
    __cordl_parent: crate::System::Object,
    pub eulaVersion: i32,
    pub privacyPolicyVersion: i32,
    pub healthAndSafetyVersion: i32,
    pub playerSensitivityFlagVersion: i32,
    pub endOfLifeVersion: i32,
}
#[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_PlayerAgreementsData => ""
    ."PlayerSaveData/PlayerAgreementsData"
);
#[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerAllOverallStatsData {
    __cordl_parent: crate::System::Object,
    pub campaignOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
    pub soloFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
    pub partyFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
    pub onlinePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
}
#[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData => ""
    ."PlayerSaveData/PlayerAllOverallStatsData"
);
#[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData {
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData1(
        &mut self,
        campaignOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        soloFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        partyFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        onlinePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    campaignOverallStatsData,
                    soloFreePlayOverallStatsData,
                    partyFreePlayOverallStatsData,
                    onlinePlayOverallStatsData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData_PlayerSaveData_PlayerOverallStatsData1(
        campaignOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        soloFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        partyFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        onlinePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    campaignOverallStatsData,
                    soloFreePlayOverallStatsData,
                    partyFreePlayOverallStatsData,
                    onlinePlayOverallStatsData,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerLevelStatsData {
    __cordl_parent: crate::System::Object,
    pub levelId: *mut crate::System::String,
    pub difficulty: BeatmapDifficulty,
    pub beatmapCharacteristicName: *mut crate::System::String,
    pub highScore: i32,
    pub maxCombo: i32,
    pub fullCombo: bool,
    pub maxRank: crate::GlobalNamespace::RankModel_Rank,
    pub validScore: bool,
    pub playCount: i32,
}
#[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData => ""
    ."PlayerSaveData/PlayerLevelStatsData"
);
#[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerMissionStatsData {
    __cordl_parent: crate::System::Object,
    pub missionId: *mut crate::System::String,
    pub cleared: bool,
}
#[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData => ""
    ."PlayerSaveData/PlayerMissionStatsData"
);
#[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerOverallStatsData {
    __cordl_parent: crate::System::Object,
    pub goodCutsCount: i32,
    pub badCutsCount: i32,
    pub missedCutsCount: i32,
    pub totalScore: i64,
    pub playedLevelsCount: i32,
    pub cleardLevelsCount: i32,
    pub failedLevelsCount: i32,
    pub fullComboCount: i32,
    pub timePlayed: f32,
    pub handDistanceTravelled: i32,
    pub cummulativeCutScoreWithoutMultiplier: i64,
}
#[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData => ""
    ."PlayerSaveData/PlayerOverallStatsData"
);
#[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData {
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_i64_i32_i32_i32_i32_f32_i32_i64_1(
        &mut self,
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCutsCount: i32,
        totalScore: i64,
        playedLevelsCount: i32,
        cleardLevelsCount: i32,
        failedLevelsCount: i32,
        fullComboCount: i32,
        timePlayed: f32,
        handDistanceTravelled: i32,
        cummulativeCutScoreWithoutMultiplier: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    goodCutsCount,
                    badCutsCount,
                    missedCutsCount,
                    totalScore,
                    playedLevelsCount,
                    cleardLevelsCount,
                    failedLevelsCount,
                    fullComboCount,
                    timePlayed,
                    handDistanceTravelled,
                    cummulativeCutScoreWithoutMultiplier,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_i64_i32_i32_i32_i32_f32_i32_i64_1(
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCutsCount: i32,
        totalScore: i64,
        playedLevelsCount: i32,
        cleardLevelsCount: i32,
        failedLevelsCount: i32,
        fullComboCount: i32,
        timePlayed: f32,
        handDistanceTravelled: i32,
        cummulativeCutScoreWithoutMultiplier: i64,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    goodCutsCount,
                    badCutsCount,
                    missedCutsCount,
                    totalScore,
                    playedLevelsCount,
                    cleardLevelsCount,
                    failedLevelsCount,
                    fullComboCount,
                    timePlayed,
                    handDistanceTravelled,
                    cummulativeCutScoreWithoutMultiplier,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData {
    __cordl_parent: VersionSaveData,
    pub localPlayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PlayerSaveData_LocalPlayer,
    >,
    pub guestPlayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PlayerSaveData_GuestPlayer,
    >,
}
#[cfg(feature = "PlayerSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerSaveData => ""."PlayerSaveData"
);
#[cfg(feature = "PlayerSaveData")]
impl std::ops::Deref for PlayerSaveData {
    type Target = VersionSaveData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData")]
impl std::ops::DerefMut for PlayerSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData")]
impl PlayerSaveData {
    pub const kCurrentVersion: &'static str = "2.0.27";
    #[cfg(feature = "PlayerSaveData+MultiplayerModeSettings")]
    pub type MultiplayerModeSettings = crate::GlobalNamespace::PlayerSaveData_MultiplayerModeSettings;
    #[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
    pub type PlayerSpecificSettings = crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings;
    #[cfg(feature = "PlayerSaveData+PlayerMissionStatsData")]
    pub type PlayerMissionStatsData = crate::GlobalNamespace::PlayerSaveData_PlayerMissionStatsData;
    #[cfg(feature = "PlayerSaveData+UserAgeCategorySaveData")]
    pub type UserAgeCategorySaveData = crate::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData;
    #[cfg(feature = "PlayerSaveData+PlayerLevelStatsData")]
    pub type PlayerLevelStatsData = crate::GlobalNamespace::PlayerSaveData_PlayerLevelStatsData;
    #[cfg(feature = "PlayerSaveData+PracticeSettings")]
    pub type PracticeSettings = crate::GlobalNamespace::PlayerSaveData_PracticeSettings;
    #[cfg(feature = "PlayerSaveData+PlayerAllOverallStatsData")]
    pub type PlayerAllOverallStatsData = crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData;
    #[cfg(feature = "PlayerSaveData+ColorScheme")]
    pub type ColorScheme = crate::GlobalNamespace::PlayerSaveData_ColorScheme;
    #[cfg(feature = "PlayerSaveData+PlayerSensitivityFlagSaveData")]
    pub type PlayerSensitivityFlagSaveData = crate::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData;
    #[cfg(feature = "PlayerSaveData+PlayerOverallStatsData")]
    pub type PlayerOverallStatsData = crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData;
    #[cfg(feature = "PlayerSaveData+OverrideEnvironmentSettings")]
    pub type OverrideEnvironmentSettings = crate::GlobalNamespace::PlayerSaveData_OverrideEnvironmentSettings;
    #[cfg(feature = "PlayerSaveData+LocalPlayer")]
    pub type LocalPlayer = crate::GlobalNamespace::PlayerSaveData_LocalPlayer;
    #[cfg(feature = "PlayerSaveData+PlayerAgreementsData")]
    pub type PlayerAgreementsData = crate::GlobalNamespace::PlayerSaveData_PlayerAgreementsData;
    #[cfg(feature = "PlayerSaveData+GameplayModifiers")]
    pub type GameplayModifiers = crate::GlobalNamespace::PlayerSaveData_GameplayModifiers;
    #[cfg(feature = "PlayerSaveData+GuestPlayer")]
    pub type GuestPlayer = crate::GlobalNamespace::PlayerSaveData_GuestPlayer;
    #[cfg(feature = "PlayerSaveData+ColorSchemesSettings")]
    pub type ColorSchemesSettings = crate::GlobalNamespace::PlayerSaveData_ColorSchemesSettings;
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData")]
impl quest_hook::libil2cpp::ObjectType for PlayerSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerSensitivityFlagSaveData")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSaveData_PlayerSensitivityFlagSaveData {
    Explicit = 3i32,
    Safe = 1i32,
    Themes = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "PlayerSaveData+PlayerSensitivityFlagSaveData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_PlayerSensitivityFlagSaveData => ""
    ."PlayerSaveData/PlayerSensitivityFlagSaveData"
);
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PlayerSpecificSettings {
    __cordl_parent: crate::System::Object,
    pub staticLights: bool,
    pub leftHanded: bool,
    pub playerHeight: f32,
    pub automaticPlayerHeight: bool,
    pub sfxVolume: f32,
    pub reduceDebris: bool,
    pub noTextsAndHuds: bool,
    pub advancedHud: bool,
    pub saberTrailIntensity: f32,
    pub _noteJumpDurationTypeSettingsSaveData: crate::GlobalNamespace::PlayerSpecificSettings_NoteJumpDurationTypeSettingsSaveData,
    pub noteJumpFixedDuration: f32,
    pub autoRestart: bool,
    pub noFailEffects: bool,
    pub noteJumpBeatOffset: f32,
    pub hideNoteSpawnEffect: bool,
    pub adaptiveSfx: bool,
    pub arcsHapticFeedback: bool,
    pub arcsVisibleSaveData: crate::GlobalNamespace::PlayerSpecificSettings_ArcVisibilityTypeSaveData,
    pub environmentEffectsFilterDefaultPreset: crate::GlobalNamespace::PlayerSpecificSettings_EnvironmentEffectsFilterPresetSaveData,
    pub environmentEffectsFilterExpertPlusPreset: crate::GlobalNamespace::PlayerSpecificSettings_EnvironmentEffectsFilterPresetSaveData,
    pub headsetHapticIntensity: f32,
}
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings => ""
    ."PlayerSaveData/PlayerSpecificSettings"
);
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
impl crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings {
    #[cfg(
        feature = "PlayerSaveData+PlayerSpecificSettings+NoteJumpDurationTypeSettingsSaveData"
    )]
    pub type NoteJumpDurationTypeSettingsSaveData = crate::GlobalNamespace::PlayerSpecificSettings_NoteJumpDurationTypeSettingsSaveData;
    #[cfg(
        feature = "PlayerSaveData+PlayerSpecificSettings+EnvironmentEffectsFilterPresetSaveData"
    )]
    pub type EnvironmentEffectsFilterPresetSaveData = crate::GlobalNamespace::PlayerSpecificSettings_EnvironmentEffectsFilterPresetSaveData;
    #[cfg(feature = "PlayerSaveData+PlayerSpecificSettings+ArcVisibilityTypeSaveData")]
    pub type ArcVisibilityTypeSaveData = crate::GlobalNamespace::PlayerSpecificSettings_ArcVisibilityTypeSaveData;
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+PlayerSpecificSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PlayerSpecificSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+PracticeSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveData_PracticeSettings {
    __cordl_parent: crate::System::Object,
    pub startSongTime: f32,
    pub songSpeedMul: f32,
}
#[cfg(feature = "PlayerSaveData+PracticeSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerSaveData_PracticeSettings
    => ""."PlayerSaveData/PracticeSettings"
);
#[cfg(feature = "PlayerSaveData+PracticeSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveData_PracticeSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PracticeSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveData_PracticeSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveData+PracticeSettings")]
impl crate::GlobalNamespace::PlayerSaveData_PracticeSettings {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveData+PracticeSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveData_PracticeSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers+SongSpeed")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplayModifiers_SongSpeed {
    Faster = 1i32,
    Normal = 0i32,
    Slower = 2i32,
}
#[cfg(feature = "PlayerSaveData+GameplayModifiers+SongSpeed")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifiers_SongSpeed =>
    ""."PlayerSaveData/GameplayModifiers/SongSpeed"
);
#[cfg(feature = "PlayerSaveData+UserAgeCategorySaveData")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSaveData_UserAgeCategorySaveData {
    Adult = 3i32,
    Child = 1i32,
    Teen = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "PlayerSaveData+UserAgeCategorySaveData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveData_UserAgeCategorySaveData => ""
    ."PlayerSaveData/UserAgeCategorySaveData"
);
