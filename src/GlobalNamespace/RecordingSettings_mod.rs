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
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
            >,
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
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::RecordingSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RecordingSettings";
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
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
                >,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::RecordingSettings as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::RecordingSettings as
                    quest_hook::libil2cpp::Type > ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
                >,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::RecordingSettings as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    crate::GlobalNamespace::BeatmapDifficulty,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BeatmapCharacteristicSO,
                    >,
                    bool,
                    bool,
                    crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
                    bool,
                    bool,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
                            >,
                        >,
                    >,
                    bool,
                    f32,
                    f32,
                    bool,
                    crate::System::Nullable_1<crate::GlobalNamespace::EnvironmentType>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                23usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::RecordingSettings as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 23usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
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
