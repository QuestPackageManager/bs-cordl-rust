#[cfg(feature = "RecordingSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub gameMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    pub level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    pub difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub characteristic: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
    pub runLevel: bool,
    pub recordPerformance: bool,
    pub recordingMode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
    pub recordingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
    pub addDateTimeSuffixToRecordingName: bool,
    pub screenshotRecording: bool,
    pub screenshotWidth: i32,
    pub screenshotHeight: i32,
    pub framerate: i32,
    pub playbackScreenshots: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
        >,
    >,
    pub practice: bool,
    pub startSongTime: f32,
    pub songSpeedMultiplier: f32,
    pub overrideEnvironments: bool,
    pub environmentType: crate::System::Nullable_1<
        crate::GlobalNamespace::EnvironmentType,
    >,
    pub environmentInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentInfoSO,
    >,
    pub saveToOldFormat: bool,
}
#[cfg(feature = "RecordingSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RecordingSettings => ""
    ."RecordingSettings"
);
#[cfg(feature = "RecordingSettings")]
impl std::ops::Deref for crate::GlobalNamespace::RecordingSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::RecordingSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingSettings")]
impl crate::GlobalNamespace::RecordingSettings {
    pub fn New(
        gameMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        characteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        runLevel: bool,
        recordPerformance: bool,
        recordingMode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
        recordingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        addDateTimeSuffixToRecordingName: bool,
        screenshotRecording: bool,
        screenshotWidth: i32,
        screenshotHeight: i32,
        framerate: i32,
        playbackScreenshots: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
            >,
        >,
        practice: bool,
        startSongTime: f32,
        songSpeedMultiplier: f32,
        overrideEnvironments: bool,
        environmentType: crate::System::Nullable_1<
            crate::GlobalNamespace::EnvironmentType,
        >,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        saveToOldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    gameMode,
                    pack,
                    level,
                    difficulty,
                    characteristic,
                    runLevel,
                    recordPerformance,
                    recordingMode,
                    recordingPath,
                    cameraView,
                    addDateTimeSuffixToRecordingName,
                    screenshotRecording,
                    screenshotWidth,
                    screenshotHeight,
                    framerate,
                    playbackScreenshots,
                    practice,
                    startSongTime,
                    songSpeedMultiplier,
                    overrideEnvironments,
                    environmentType,
                    environmentInfo,
                    saveToOldFormat,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        gameMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        characteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        runLevel: bool,
        recordPerformance: bool,
        recordingMode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
        recordingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        addDateTimeSuffixToRecordingName: bool,
        screenshotRecording: bool,
        screenshotWidth: i32,
        screenshotHeight: i32,
        framerate: i32,
        playbackScreenshots: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
            >,
        >,
        practice: bool,
        startSongTime: f32,
        songSpeedMultiplier: f32,
        overrideEnvironments: bool,
        environmentType: crate::System::Nullable_1<
            crate::GlobalNamespace::EnvironmentType,
        >,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        saveToOldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    gameMode,
                    pack,
                    level,
                    difficulty,
                    characteristic,
                    runLevel,
                    recordPerformance,
                    recordingMode,
                    recordingPath,
                    cameraView,
                    addDateTimeSuffixToRecordingName,
                    screenshotRecording,
                    screenshotWidth,
                    screenshotHeight,
                    framerate,
                    playbackScreenshots,
                    practice,
                    startSongTime,
                    songSpeedMultiplier,
                    overrideEnvironments,
                    environmentType,
                    environmentInfo,
                    saveToOldFormat,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RecordingSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RecordingSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
