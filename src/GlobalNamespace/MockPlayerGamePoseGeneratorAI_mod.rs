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
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
        >,
    >,
    pub _rightNotes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
        >,
    >,
    pub _bombNotes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
        >,
    >,
    pub _obstacles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockObstacleData>,
        >,
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MockPlayerGamePoseGeneratorAI {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MockPlayerGamePoseGeneratorAI";
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
#[cfg(feature = "MockPlayerGamePoseGeneratorAI")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlayerGamePoseGeneratorAI {
    type Target = crate::GlobalNamespace::MockPlayerGamePoseGenerator;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorAI")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlayerGamePoseGeneratorAI {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                        ),
                        crate::UnityEngine::Vector3,
                        5usize,
                    >("Blerp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blerp",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (prevStart, prevEnd, currStart, currEnd, t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCutDirection(
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::NoteCutDirection),
                        crate::UnityEngine::Vector3,
                        1usize,
                    >("GetCutDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCutDirection", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (cutDirection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNotePosition(
        lineCount: i32,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MockNoteData,
                            >,
                        ),
                        crate::UnityEngine::Vector3,
                        2usize,
                    >("GetNotePosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNotePosition", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (lineCount, noteData))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i64,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("HandleNoteWasSpawned")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleNoteWasSpawned", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (userId, syncTime, songTime, noteSpawnInfoNetSerializable),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i64,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("HandleObstacleWasSpawned")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleObstacleWasSpawned", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (userId, syncTime, songTime, obstacleSpawnInfoNetSerializable),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i64,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("HandleSliderWasSpawned")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleSliderWasSpawned", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (userId, syncTime, songTime, sliderSpawnInfoNetSerializable),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i64,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MockBeatmapData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (introStartTime, beatmapData, gameplayModifiers, onSongFinished),
                )?
        };
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
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
            >,
        >,
        handDirection: crate::UnityEngine::Vector3,
        noteIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        prevHitScore: quest_hook::libil2cpp::ByRefMut<i32>,
        nextHitScore: quest_hook::libil2cpp::ByRefMut<i32>,
        bombs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
            >,
        >,
        bombIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        lineCount: i32,
        songTime: f32,
        wasHitOrMiss: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MockNoteData,
                                    >,
                                >,
                            >,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MockNoteData,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        crate::UnityEngine::Pose,
                        10usize,
                    >("ProcessNotes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessNotes", 10usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Pose = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessObstacles(
        &mut self,
        obstacles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockObstacleData>,
            >,
        >,
        obstacleIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        lineCount: i32,
        prevHeadPose: crate::UnityEngine::Pose,
        leftHandPose: crate::UnityEngine::Pose,
        rightHandPose: crate::UnityEngine::Pose,
        songTime: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::MockObstacleData,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            crate::UnityEngine::Pose,
                            crate::UnityEngine::Pose,
                            crate::UnityEngine::Pose,
                            f32,
                        ),
                        crate::UnityEngine::Pose,
                        7usize,
                    >("ProcessObstacles")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessObstacles", 7usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Pose = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        obstacles,
                        obstacleIndex,
                        lineCount,
                        prevHeadPose,
                        leftHandPose,
                        rightHandPose,
                        songTime,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SimulateFail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("SimulateFail")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SimulateFail", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Tick")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Tick",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            i32,
                            crate::UnityEngine::Pose,
                            crate::UnityEngine::Pose,
                            f32,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MockNoteData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MockNoteData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        11usize,
                    >("UpdateScore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateScore", 11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IMultiplayerSessionManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IGameplayRpcManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IMockPlayerScoreCalculator,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        multiplayerSessionManager,
                        gameplayRpcManager,
                        scoreCalculator,
                        leftHanded,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_songTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_songTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_songTime", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
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
