#[cfg(feature = "MockPlayerGamePoseGeneratorAI")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerGamePoseGeneratorAI {
    __cordl_parent: crate::GlobalNamespace::MockPlayerGamePoseGenerator,
    pub _scoreCalculator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMockPlayerScoreCalculator,
    >,
    pub _lastKnowScore: i32,
    pub _songStartTime: i64,
    pub _timeScale: f32,
    pub _onSongFinished: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
    pub _lastEventTime: f32,
    pub _lastHeadPose: crate::UnityEngine::Pose,
    pub _lastLeftHandPose: crate::UnityEngine::Pose,
    pub _lastRightHandPose: crate::UnityEngine::Pose,
    pub _lastSongTime: f32,
    pub _lineCount: i32,
    pub _leftNotes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::MockNoteData>,
    >,
    pub _rightNotes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::MockNoteData>,
    >,
    pub _bombNotes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::MockNoteData>,
    >,
    pub _obstacles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::MockObstacleData>,
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
    pub fn Blerp(
        prevStart: crate::UnityEngine::Vector3,
        prevEnd: crate::UnityEngine::Vector3,
        currStart: crate::UnityEngine::Vector3,
        currEnd: crate::UnityEngine::Vector3,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blerp", (prevStart, prevEnd, currStart, currEnd, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCutDirection(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCutDirection", (cutDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNotePosition(
        lineCount: i32,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNotePosition", (lineCount, noteData))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        noteSpawnInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteWasSpawned",
                (userId, syncTime, songTime, noteSpawnInfoNetSerializable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstacleWasSpawned(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        obstacleSpawnInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleObstacleWasSpawned",
                (userId, syncTime, songTime, obstacleSpawnInfoNetSerializable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSliderWasSpawned(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        sliderSpawnInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSliderWasSpawned",
                (userId, syncTime, songTime, sliderSpawnInfoNetSerializable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        introStartTime: i64,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockBeatmapData>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        onSongFinished: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (introStartTime, beatmapData, gameplayModifiers, onSongFinished),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        scoreCalculator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMockPlayerScoreCalculator,
        >,
        leftHanded: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn ProcessNotes(
        &mut self,
        notes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::MockNoteData>,
        >,
        handDirection: crate::UnityEngine::Vector3,
        noteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        prevHitScore: quest_hook::libil2cpp::ByRefMut<i32>,
        nextHitScore: quest_hook::libil2cpp::ByRefMut<i32>,
        bombs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::MockNoteData>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessObstacles(
        &mut self,
        obstacles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::MockObstacleData,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn SimulateFail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SimulateFail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret.into())
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
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
        nextNoteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        scoreCalculator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMockPlayerScoreCalculator,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_songTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTime", ())?;
        Ok(__cordl_ret.into())
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
