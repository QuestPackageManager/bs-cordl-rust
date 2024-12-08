#[cfg(feature = "PlayerSaveDataV1_0_1+AchievementsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_AchievementsData {
    __cordl_parent: crate::System::Object,
    pub unlockedAchievements: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub unlockedAchievementsToUpload: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+AchievementsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData => ""
    ."PlayerSaveDataV1_0_1/AchievementsData"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+AchievementsData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+AchievementsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+AchievementsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData {
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
#[cfg(feature = "PlayerSaveDataV1_0_1+AchievementsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplayModifiers_EnabledObstacleType {
    All = 0i32,
    FullHeightOnly = 1i32,
    None = 2i32,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayModifiers_EnabledObstacleType => ""
    ."PlayerSaveDataV1_0_1/GameplayModifiers/EnabledObstacleType"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplayModifiers_EnergyType {
    Bar = 0i32,
    Battery = 1i32,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifiers_EnergyType =>
    ""."PlayerSaveDataV1_0_1/GameplayModifiers/EnergyType"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_GameplayModifiers {
    __cordl_parent: crate::System::Object,
    pub energyType: crate::GlobalNamespace::GameplayModifiers_EnergyType,
    pub noFail: bool,
    pub instaFail: bool,
    pub failOnSaberClash: bool,
    pub enabledObstacleType: crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
    pub fastNotes: bool,
    pub strictAngles: bool,
    pub disappearingArrows: bool,
    pub noBombs: bool,
    pub songSpeed: crate::GlobalNamespace::GameplayModifiers_SongSpeed,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers => ""
    ."PlayerSaveDataV1_0_1/GameplayModifiers"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers {
    #[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
    pub type SongSpeed = crate::GlobalNamespace::GameplayModifiers_SongSpeed;
    #[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+EnergyType")]
    pub type EnergyType = crate::GlobalNamespace::GameplayModifiers_EnergyType;
    #[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+EnabledObstacleType")]
    pub type EnabledObstacleType = crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType;
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
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GuestPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_GuestPlayer {
    __cordl_parent: crate::System::Object,
    pub playerName: *mut crate::System::String,
    pub playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GuestPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer => ""
    ."PlayerSaveDataV1_0_1/GuestPlayer"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+GuestPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GuestPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GuestPlayer")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer {
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
#[cfg(feature = "PlayerSaveDataV1_0_1+GuestPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+LocalPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_LocalPlayer {
    __cordl_parent: crate::System::Object,
    pub playerId: *mut crate::System::String,
    pub playerName: *mut crate::System::String,
    pub shouldShowTutorialPrompt: bool,
    pub gameplayModifiers: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers,
    pub playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings,
    pub playerAllOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData,
    pub levelsStatsData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData,
    >,
    pub missionsStatsData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData,
    >,
    pub showedMissionHelpIds: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub achievementsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+LocalPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer => ""
    ."PlayerSaveDataV1_0_1/LocalPlayer"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+LocalPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+LocalPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+LocalPlayer")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer {
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
#[cfg(feature = "PlayerSaveDataV1_0_1+LocalPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    __cordl_parent: crate::System::Object,
    pub campaignOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
    pub soloFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
    pub partyFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData => ""
    ."PlayerSaveDataV1_0_1/PlayerAllOverallStatsData"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
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
    pub fn _ctor_PlayerSaveDataV1_0_1_PlayerOverallStatsData_PlayerSaveDataV1_0_1_PlayerOverallStatsData_PlayerSaveDataV1_0_1_PlayerOverallStatsData1(
        &mut self,
        campaignOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        soloFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        partyFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
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
    pub fn New_PlayerSaveDataV1_0_1_PlayerOverallStatsData_PlayerSaveDataV1_0_1_PlayerOverallStatsData_PlayerSaveDataV1_0_1_PlayerOverallStatsData1(
        campaignOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        soloFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        partyFreePlayOverallStatsData: *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
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
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerLevelStatsData {
    __cordl_parent: crate::System::Object,
    pub levelId: *mut crate::System::String,
    pub difficulty: BeatmapDifficulty,
    pub highScore: i32,
    pub maxCombo: i32,
    pub fullCombo: bool,
    pub maxRank: crate::GlobalNamespace::RankModel_Rank,
    pub validScore: bool,
    pub playCount: i32,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData => ""
    ."PlayerSaveDataV1_0_1/PlayerLevelStatsData"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData {
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
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerMissionStatsData {
    __cordl_parent: crate::System::Object,
    pub missionId: *mut crate::System::String,
    pub cleared: bool,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData => ""
    ."PlayerSaveDataV1_0_1/PlayerMissionStatsData"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData {
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
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerOverallStatsData {
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
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData => ""
    ."PlayerSaveDataV1_0_1/PlayerOverallStatsData"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData {
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
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1 {
    __cordl_parent: crate::System::Object,
    pub version: *mut crate::System::String,
    pub localPlayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer,
    >,
    pub guestPlayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer,
    >,
    pub lastSelectedBeatmapDifficulty: BeatmapDifficulty,
}
#[cfg(feature = "PlayerSaveDataV1_0_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerSaveDataV1_0_1 => ""."PlayerSaveDataV1_0_1"
);
#[cfg(feature = "PlayerSaveDataV1_0_1")]
impl std::ops::Deref for PlayerSaveDataV1_0_1 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1")]
impl std::ops::DerefMut for PlayerSaveDataV1_0_1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1")]
impl PlayerSaveDataV1_0_1 {
    pub const kCurrentVersion: &'static str = "";
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerLevelStatsData")]
    pub type PlayerLevelStatsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerLevelStatsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+LocalPlayer")]
    pub type LocalPlayer = crate::GlobalNamespace::PlayerSaveDataV1_0_1_LocalPlayer;
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
    pub type PlayerSpecificSettings = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings;
    #[cfg(feature = "PlayerSaveDataV1_0_1+AchievementsData")]
    pub type AchievementsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_AchievementsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerOverallStatsData")]
    pub type PlayerOverallStatsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers")]
    pub type GameplayModifiers = crate::GlobalNamespace::PlayerSaveDataV1_0_1_GameplayModifiers;
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerAllOverallStatsData")]
    pub type PlayerAllOverallStatsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+PlayerMissionStatsData")]
    pub type PlayerMissionStatsData = crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerMissionStatsData;
    #[cfg(feature = "PlayerSaveDataV1_0_1+GuestPlayer")]
    pub type GuestPlayer = crate::GlobalNamespace::PlayerSaveDataV1_0_1_GuestPlayer;
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
#[cfg(feature = "PlayerSaveDataV1_0_1")]
impl quest_hook::libil2cpp::ObjectType for PlayerSaveDataV1_0_1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataV1_0_1_PlayerSpecificSettings {
    __cordl_parent: crate::System::Object,
    pub staticLights: bool,
    pub leftHanded: bool,
    pub swapColors: bool,
    pub playerHeight: f32,
    pub disableSFX: bool,
    pub reduceDebris: bool,
    pub advancedHud: bool,
    pub noTextsAndHuds: bool,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings => ""
    ."PlayerSaveDataV1_0_1/PlayerSpecificSettings"
);
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
impl crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings {
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
#[cfg(feature = "PlayerSaveDataV1_0_1+PlayerSpecificSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerSpecificSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplayModifiers_SongSpeed {
    Faster = 1i32,
    Normal = 0i32,
    Slower = 2i32,
}
#[cfg(feature = "PlayerSaveDataV1_0_1+GameplayModifiers+SongSpeed")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifiers_SongSpeed =>
    ""."PlayerSaveDataV1_0_1/GameplayModifiers/SongSpeed"
);
