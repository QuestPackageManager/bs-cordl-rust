#[cfg(feature = "RunLevelMenuDestination")]
#[repr(C)]
#[derive(Debug)]
pub struct RunLevelMenuDestination {
    __cordl_parent: MenuDestination,
    pub beatmapLevelPack: *mut BeatmapLevelPack,
    pub beatmapLevel: *mut BeatmapLevel,
    pub beatmapDifficulty: BeatmapDifficulty,
    pub beatmapCharacteristic: *mut BeatmapCharacteristicSO,
    pub gameplayModifiers: *mut GameplayModifiers,
    pub practice: bool,
    pub startSongTime: f32,
    pub songSpeedMultiplier: f32,
    pub overrideEnvironments: bool,
    pub environmentType: *mut crate::System::String,
    pub environmentName: *mut crate::System::String,
    pub quitAppAfterRun: bool,
}
#[cfg(feature = "RunLevelMenuDestination")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RunLevelMenuDestination => ""."RunLevelMenuDestination"
);
#[cfg(feature = "RunLevelMenuDestination")]
impl std::ops::Deref for RunLevelMenuDestination {
    type Target = MenuDestination;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RunLevelMenuDestination")]
impl std::ops::DerefMut for RunLevelMenuDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RunLevelMenuDestination")]
impl RunLevelMenuDestination {
    pub fn New(
        beatmapLevelPack: *mut BeatmapLevelPack,
        beatmapLevel: *mut BeatmapLevel,
        beatmapDifficulty: BeatmapDifficulty,
        beatmapCharacteristic: *mut BeatmapCharacteristicSO,
        gameplayModifiers: *mut GameplayModifiers,
        practice: bool,
        startSongTime: f32,
        songSpeedMultiplier: f32,
        overrideEnvironments: bool,
        environmentType: *mut crate::System::String,
        environmentName: *mut crate::System::String,
        quitAppAfterRun: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapLevelPack,
                    beatmapLevel,
                    beatmapDifficulty,
                    beatmapCharacteristic,
                    gameplayModifiers,
                    practice,
                    startSongTime,
                    songSpeedMultiplier,
                    overrideEnvironments,
                    environmentType,
                    environmentName,
                    quitAppAfterRun,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        beatmapLevelPack: *mut BeatmapLevelPack,
        beatmapLevel: *mut BeatmapLevel,
        beatmapDifficulty: BeatmapDifficulty,
        beatmapCharacteristic: *mut BeatmapCharacteristicSO,
        gameplayModifiers: *mut GameplayModifiers,
        practice: bool,
        startSongTime: f32,
        songSpeedMultiplier: f32,
        overrideEnvironments: bool,
        environmentType: *mut crate::System::String,
        environmentName: *mut crate::System::String,
        quitAppAfterRun: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapLevelPack,
                    beatmapLevel,
                    beatmapDifficulty,
                    beatmapCharacteristic,
                    gameplayModifiers,
                    practice,
                    startSongTime,
                    songSpeedMultiplier,
                    overrideEnvironments,
                    environmentType,
                    environmentName,
                    quitAppAfterRun,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RunLevelMenuDestination")]
impl quest_hook::libil2cpp::ObjectType for RunLevelMenuDestination {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
