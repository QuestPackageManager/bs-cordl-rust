#[cfg(feature = "RecordingSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingSettings {
    __cordl_parent: crate::System::Object,
    pub gameMode: *mut crate::System::String,
    pub pack: *mut BeatmapLevelPack,
    pub level: *mut BeatmapLevel,
    pub difficulty: BeatmapDifficulty,
    pub characteristic: *mut BeatmapCharacteristicSO,
    pub runLevel: bool,
    pub recordPerformance: bool,
    pub recordingMode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
    pub recordingPath: *mut crate::System::String,
    pub cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
    pub addDateTimeSuffixToRecordingName: bool,
    pub screenshotRecording: bool,
    pub screenshotWidth: i32,
    pub screenshotHeight: i32,
    pub framerate: i32,
    pub playbackScreenshots: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
    >,
    pub practice: bool,
    pub startSongTime: f32,
    pub songSpeedMultiplier: f32,
    pub overrideEnvironments: bool,
    pub environmentType: crate::System::Nullable_1<EnvironmentType>,
    pub environmentInfo: *mut EnvironmentInfoSO,
    pub saveToOldFormat: bool,
}
#[cfg(feature = "RecordingSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RecordingSettings => ""."RecordingSettings"
);
#[cfg(feature = "RecordingSettings")]
impl std::ops::Deref for RecordingSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingSettings")]
impl std::ops::DerefMut for RecordingSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingSettings")]
impl RecordingSettings {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        gameMode: *mut crate::System::String,
        pack: *mut BeatmapLevelPack,
        level: *mut BeatmapLevel,
        difficulty: BeatmapDifficulty,
        characteristic: *mut BeatmapCharacteristicSO,
        runLevel: bool,
        recordPerformance: bool,
        recordingMode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
        recordingPath: *mut crate::System::String,
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        addDateTimeSuffixToRecordingName: bool,
        screenshotRecording: bool,
        screenshotWidth: i32,
        screenshotHeight: i32,
        framerate: i32,
        playbackScreenshots: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
        >,
        practice: bool,
        startSongTime: f32,
        songSpeedMultiplier: f32,
        overrideEnvironments: bool,
        environmentType: crate::System::Nullable_1<EnvironmentType>,
        environmentInfo: *mut EnvironmentInfoSO,
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
        Ok(__cordl_ret)
    }
    pub fn New(
        gameMode: *mut crate::System::String,
        pack: *mut BeatmapLevelPack,
        level: *mut BeatmapLevel,
        difficulty: BeatmapDifficulty,
        characteristic: *mut BeatmapCharacteristicSO,
        runLevel: bool,
        recordPerformance: bool,
        recordingMode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
        recordingPath: *mut crate::System::String,
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        addDateTimeSuffixToRecordingName: bool,
        screenshotRecording: bool,
        screenshotWidth: i32,
        screenshotHeight: i32,
        framerate: i32,
        playbackScreenshots: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
        >,
        practice: bool,
        startSongTime: f32,
        songSpeedMultiplier: f32,
        overrideEnvironments: bool,
        environmentType: crate::System::Nullable_1<EnvironmentType>,
        environmentInfo: *mut EnvironmentInfoSO,
        saveToOldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
    }
}
#[cfg(feature = "RecordingSettings")]
impl quest_hook::libil2cpp::ObjectType for RecordingSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
