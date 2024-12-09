#[cfg(feature = "MockPlayerGamePoseGeneratorAI")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerGamePoseGeneratorAI {
    __cordl_parent: crate::GlobalNamespace::MockPlayerGamePoseGenerator,
    pub _scoreCalculator: *mut crate::GlobalNamespace::IMockPlayerScoreCalculator,
    pub _lastKnowScore: i32,
    pub _songStartTime: i64,
    pub _timeScale: f32,
    pub _onSongFinished: *mut crate::System::Action,
    pub _gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    pub _lastEventTime: f32,
    pub _lastHeadPose: crate::UnityEngine::Pose,
    pub _lastLeftHandPose: crate::UnityEngine::Pose,
    pub _lastRightHandPose: crate::UnityEngine::Pose,
    pub _lastSongTime: f32,
    pub _lineCount: i32,
    pub _leftNotes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MockNoteData,
    >,
    pub _rightNotes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MockNoteData,
    >,
    pub _bombNotes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MockNoteData,
    >,
    pub _obstacles: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MockObstacleData,
    >,
    pub _leftNoteIndex: i32,
    pub _rightNoteIndex: i32,
    pub _bombNoteIndex: i32,
    pub _obstacleIndex: i32,
    pub _prevLeftScore: i32,
    pub _prevRightScore: i32,
    pub _nextLeftHitScore: i32,
    pub _nextRightHitScore: i32,
    pub _score: i32,
    pub _combo: i32,
    pub _multiplier: i32,
    pub _fullCombo: bool,
    pub _hasFinishedLevel: bool,
    pub _isInited: bool,
}
#[cfg(feature = "MockPlayerGamePoseGeneratorAI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockPlayerGamePoseGeneratorAI
    => ""."MockPlayerGamePoseGeneratorAI"
);
#[cfg(feature = "MockPlayerGamePoseGeneratorAI")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlayerGamePoseGeneratorAI {
    type Target = crate::GlobalNamespace::MockPlayerGamePoseGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorAI")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlayerGamePoseGeneratorAI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorAI")]
impl crate::GlobalNamespace::MockPlayerGamePoseGeneratorAI {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        syncTime: i64,
        songTime: f32,
        noteSpawnInfoNetSerializable: *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteWasSpawned",
                (userId, syncTime, songTime, noteSpawnInfoNetSerializable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstacleWasSpawned(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        syncTime: i64,
        songTime: f32,
        obstacleSpawnInfoNetSerializable: *mut crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleObstacleWasSpawned",
                (userId, syncTime, songTime, obstacleSpawnInfoNetSerializable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSliderWasSpawned(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        syncTime: i64,
        songTime: f32,
        sliderSpawnInfoNetSerializable: *mut crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSliderWasSpawned",
                (userId, syncTime, songTime, sliderSpawnInfoNetSerializable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        introStartTime: i64,
        beatmapData: *mut crate::GlobalNamespace::MockBeatmapData,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        onSongFinished: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (introStartTime, beatmapData, gameplayModifiers, onSongFinished),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
        gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
        scoreCalculator: *mut crate::GlobalNamespace::IMockPlayerScoreCalculator,
        leftHanded: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    multiplayerSessionManager,
                    gameplayRpcManager,
                    scoreCalculator,
                    leftHanded,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn ProcessNotes(
        &mut self,
        notes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MockNoteData,
        >,
        handDirection: crate::UnityEngine::Vector3,
        noteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        prevHitScore: quest_hook::libil2cpp::ByRefMut<i32>,
        nextHitScore: quest_hook::libil2cpp::ByRefMut<i32>,
        bombs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MockNoteData,
        >,
        bombIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        lineCount: i32,
        songTime: f32,
        wasHitOrMiss: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke(
                "ProcessNotes",
                (
                    notes,
                    handDirection,
                    noteIndex,
                    prevHitScore,
                    nextHitScore,
                    bombs,
                    bombIndex,
                    lineCount,
                    songTime,
                    wasHitOrMiss,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessObstacles(
        &mut self,
        obstacles: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MockObstacleData,
        >,
        obstacleIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        lineCount: i32,
        prevHeadPose: crate::UnityEngine::Pose,
        leftHandPose: crate::UnityEngine::Pose,
        rightHandPose: crate::UnityEngine::Pose,
        songTime: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke(
                "ProcessObstacles",
                (
                    obstacles,
                    obstacleIndex,
                    lineCount,
                    prevHeadPose,
                    leftHandPose,
                    rightHandPose,
                    songTime,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SimulateFail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SimulateFail", ())?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateScore(
        &mut self,
        currentScore: quest_hook::libil2cpp::ByRefMut<i32>,
        currentCombo: quest_hook::libil2cpp::ByRefMut<i32>,
        currentMultiplier: quest_hook::libil2cpp::ByRefMut<i32>,
        hitScore: i32,
        lineCount: i32,
        lastPose: crate::UnityEngine::Pose,
        currentPose: crate::UnityEngine::Pose,
        lastSongTime: f32,
        songTime: f32,
        noteData: *mut crate::GlobalNamespace::MockNoteData,
        nextNoteData: *mut crate::GlobalNamespace::MockNoteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateScore",
                (
                    currentScore,
                    currentCombo,
                    currentMultiplier,
                    hitScore,
                    lineCount,
                    lastPose,
                    currentPose,
                    lastSongTime,
                    songTime,
                    noteData,
                    nextNoteData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
        gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
        scoreCalculator: *mut crate::GlobalNamespace::IMockPlayerScoreCalculator,
        leftHanded: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    multiplayerSessionManager,
                    gameplayRpcManager,
                    scoreCalculator,
                    leftHanded,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_songTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTime", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorAI")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlayerGamePoseGeneratorAI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
