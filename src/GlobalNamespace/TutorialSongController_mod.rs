#[cfg(feature = "TutorialSongController+SequenceCutInfo+NoteType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SequenceCutInfo_TutorialSongController_NoteType {
    #[default]
    Arc = 2i32,
    Chain = 1i32,
    Normal = 0i32,
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo+NoteType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/SequenceCutInfo/NoteType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo+NoteType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo+NoteType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo+NoteType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo+NoteType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "TutorialSongController")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController {
    __cordl_parent: crate::GlobalNamespace::SongController,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _startWaitTimeInBeats: i32,
    pub _numberOfBeatsToSnap: i32,
    pub _obstacleDurationInBeats: i32,
    pub _noteCuttingTutorialPartDidStartSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _noteCuttingInAnyDirectionDidStartSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _chainCuttingDidStartSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _chainLinkMissedSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _arcCuttingDidStartSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _arcHeadOrTailMissedSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _arcMiddleInfoSignal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
    pub _bombCuttingTutorialPartDidStartSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _leftObstacleTutorialPartDidStartSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _rightObstacleTutorialPartDidStartSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _topObstacleTutorialPartDidStartSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _noteWasCutOKSignal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
    pub _noteWasCutTooSoonSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _noteWasCutWithWrongColorSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _noteWasCutFromDifferentDirectionSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _noteWasCutWithSlowSpeedSignal: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Signal,
    >,
    pub _bombWasCutSignal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TutorialSongController_InitData,
    >,
    pub _beatmapObjectManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectManager,
    >,
    pub _tutorialBeatmapObjectIndex: i32,
    pub _prevSpawnedBeatmapObjectIndex: i32,
    pub _songBpm: f32,
    pub _beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    pub _currentSequenceCutInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TutorialSongController_SequenceCutInfo,
    >,
    pub _normalModeTutorialObjectsSpawnData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData,
            >,
        >,
    >,
}
#[cfg(feature = "TutorialSongController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController";
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
#[cfg(feature = "TutorialSongController")]
impl std::ops::Deref for crate::GlobalNamespace::TutorialSongController {
    type Target = crate::GlobalNamespace::SongController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController")]
impl std::ops::DerefMut for crate::GlobalNamespace::TutorialSongController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController")]
impl crate::GlobalNamespace::TutorialSongController {
    #[cfg(feature = "TutorialSongController+InitData")]
    pub type InitData = crate::GlobalNamespace::TutorialSongController_InitData;
    #[cfg(feature = "TutorialSongController+SequenceCutInfo")]
    pub type SequenceCutInfo = crate::GlobalNamespace::TutorialSongController_SequenceCutInfo;
    #[cfg(feature = "TutorialSongController+TutorialArcSpawnData")]
    pub type TutorialArcSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialArcSpawnData;
    #[cfg(feature = "TutorialSongController+TutorialBasicNoteSpawnData")]
    pub type TutorialBasicNoteSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData;
    #[cfg(feature = "TutorialSongController+TutorialBombNoteSpawnData")]
    pub type TutorialBombNoteSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData;
    #[cfg(feature = "TutorialSongController+TutorialChainSpawnData")]
    pub type TutorialChainSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialChainSpawnData;
    #[cfg(feature = "TutorialSongController+TutorialJumpingNoteSpawnData")]
    pub type TutorialJumpingNoteSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData;
    #[cfg(feature = "TutorialSongController+TutorialObjectSpawnData")]
    pub type TutorialObjectSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData;
    #[cfg(feature = "TutorialSongController+TutorialObstacleSpawnData")]
    pub type TutorialObstacleSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Awake", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateArcData(
        &mut self,
        headData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        tailData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
                2usize,
            >("CreateArcData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateArcData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData> = unsafe {
            method.invoke_unchecked(self, (headData, tailData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateBasicNoteData(
        &mut self,
        _cordl_time: f32,
        beat: i32,
        tutorialBasicNoteSpawnData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                3usize,
            >("CreateBasicNoteData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateBasicNoteData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData> = unsafe {
            method
                .invoke_unchecked(self, (_cordl_time, beat, tutorialBasicNoteSpawnData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateBombNoteData(
        &mut self,
        _cordl_time: f32,
        beat: i32,
        tutorialBombNoteSpawnData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                3usize,
            >("CreateBombNoteData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateBombNoteData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData> = unsafe {
            method.invoke_unchecked(self, (_cordl_time, beat, tutorialBombNoteSpawnData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateChainData(
        &mut self,
        _cordl_time: f32,
        tailTime: f32,
        beat: i32,
        tutorialChainSpawnData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialSongController_TutorialChainSpawnData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::TutorialSongController_TutorialChainSpawnData,
                    >,
                ),
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
                >,
                4usize,
            >("CreateChainData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateChainData", 4usize
                )
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (_cordl_time, tailTime, beat, tutorialChainSpawnData),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateObstacleData(
        &mut self,
        _cordl_time: f32,
        beat: i32,
        tutorialObstacleSpawnData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
                3usize,
            >("CreateObstacleData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateObstacleData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleData,
        > = unsafe {
            method.invoke_unchecked(self, (_cordl_time, beat, tutorialObstacleSpawnData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextBeatmapObjectBeat(
        &mut self,
        beatOffset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetNextBeatmapObjectBeat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNextBeatmapObjectBeat", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (beatOffset)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeFromBeat(
        &mut self,
        beatNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), f32, 1usize>("GetTimeFromBeat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTimeFromBeat", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (beatNumber)) };
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
                    quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleNoteWasCut")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleNoteWasCut", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (noteController, noteCutInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasMissed(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleNoteWasMissed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleNoteWasMissed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (noteController))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstacleDidPassThreeQuartersOfMove2(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleController>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleObstacleDidPassThreeQuartersOfMove2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleObstacleDidPassThreeQuartersOfMove2", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (obstacleController))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDestroy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn PauseSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PauseSong")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PauseSong", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaiseSignalForIncorrectCutSequence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RaiseSignalForIncorrectCutSequence")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RaiseSignalForIncorrectCutSequence", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaiseSignalsForIndividualCut(
        &mut self,
        noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteCutInfo),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RaiseSignalsForIndividualCut")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RaiseSignalsForIndividualCut", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (noteCutInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResumeSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResumeSong")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ResumeSong", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Start", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartSong(
        &mut self,
        startTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("StartSong")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StartSong", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (startTimeOffset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn StopSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("StopSong")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StopSong", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBeatmapData(
        &mut self,
        noteTime: f32,
        noteBeat: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UpdateBeatmapData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateBeatmapData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (noteTime, noteBeat))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialSongController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub songBpm: f32,
    pub beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
}
#[cfg(feature = "TutorialSongController+InitData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController_InitData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/InitData";
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
#[cfg(feature = "TutorialSongController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::TutorialSongController_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::TutorialSongController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+InitData")]
impl crate::GlobalNamespace::TutorialSongController_InitData {
    pub fn New(
        songBpm: f32,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (songBpm, beatmapData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        songBpm: f32,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (songBpm, beatmapData))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_SequenceCutInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub anyWasCutTooSoon: bool,
    pub allSaberTypeOK: bool,
    pub allSpeedOK: bool,
    pub allDirectionOK: bool,
    pub noteType: crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType,
    pub cuttableObjectsCount: i32,
    pub cutObjects: i32,
    pub missedObjects: i32,
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController_SequenceCutInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/SequenceCutInfo";
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
#[cfg(feature = "TutorialSongController+SequenceCutInfo")]
impl std::ops::Deref for crate::GlobalNamespace::TutorialSongController_SequenceCutInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TutorialSongController_SequenceCutInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo")]
impl crate::GlobalNamespace::TutorialSongController_SequenceCutInfo {
    #[cfg(feature = "TutorialSongController+SequenceCutInfo+NoteType")]
    pub type NoteType = crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType;
    pub fn MarkCut(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("MarkCut")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MarkCut", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (noteCutInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn MarkMiss(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("MarkMiss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MarkMiss", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        cuttableObjectsCount: i32,
        noteType: crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cuttableObjectsCount, noteType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        cuttableObjectsCount: i32,
        noteType: crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    crate::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cuttableObjectsCount, noteType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_allIsOK(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_allIsOK")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_allIsOK", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_isFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isFinished")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isFinished", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_missedAny(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_missedAny")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_missedAny", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController+SequenceCutInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController_SequenceCutInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialSongController+TutorialArcSpawnData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_TutorialArcSpawnData {
    __cordl_parent: crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData,
    pub headNote: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
    >,
    pub tailNote: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
    >,
}
#[cfg(feature = "TutorialSongController+TutorialArcSpawnData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController_TutorialArcSpawnData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/TutorialArcSpawnData";
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
#[cfg(feature = "TutorialSongController+TutorialArcSpawnData")]
impl std::ops::Deref
for crate::GlobalNamespace::TutorialSongController_TutorialArcSpawnData {
    type Target = crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialArcSpawnData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TutorialSongController_TutorialArcSpawnData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialArcSpawnData")]
impl crate::GlobalNamespace::TutorialSongController_TutorialArcSpawnData {
    pub fn New(
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        headNote: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
        >,
        tailNote: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signal, headNote, tailNote))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        headNote: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
        >,
        tailNote: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (signal, headNote, tailNote))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController+TutorialArcSpawnData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController_TutorialArcSpawnData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialSongController+TutorialBasicNoteSpawnData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_TutorialBasicNoteSpawnData {
    __cordl_parent: crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData,
    pub cutDirection: crate::GlobalNamespace::NoteCutDirection,
    pub colorType: crate::GlobalNamespace::ColorType,
}
#[cfg(feature = "TutorialSongController+TutorialBasicNoteSpawnData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/TutorialBasicNoteSpawnData";
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
#[cfg(feature = "TutorialSongController+TutorialBasicNoteSpawnData")]
impl std::ops::Deref
for crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData {
    type Target = crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialBasicNoteSpawnData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialBasicNoteSpawnData")]
impl crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData {
    pub fn New(
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    signal,
                    firstTimeBeatOffset,
                    beatOffset,
                    lineIndex,
                    noteLineLayer,
                    cutDirection,
                    colorType,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
                    i32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteCutDirection,
                    crate::GlobalNamespace::ColorType,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        signal,
                        firstTimeBeatOffset,
                        beatOffset,
                        lineIndex,
                        noteLineLayer,
                        cutDirection,
                        colorType,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController+TutorialBasicNoteSpawnData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialSongController+TutorialBombNoteSpawnData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_TutorialBombNoteSpawnData {
    __cordl_parent: crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData,
}
#[cfg(feature = "TutorialSongController+TutorialBombNoteSpawnData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/TutorialBombNoteSpawnData";
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
#[cfg(feature = "TutorialSongController+TutorialBombNoteSpawnData")]
impl std::ops::Deref
for crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData {
    type Target = crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialBombNoteSpawnData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialBombNoteSpawnData")]
impl crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData {
    pub fn New(
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
                    i32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController+TutorialBombNoteSpawnData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialSongController+TutorialChainSpawnData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_TutorialChainSpawnData {
    __cordl_parent: crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData,
    pub colorType: crate::GlobalNamespace::ColorType,
    pub headLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub headCutDirection: crate::GlobalNamespace::NoteCutDirection,
    pub tailLineIndex: i32,
    pub tailLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub sliceCount: i32,
    pub squishAmount: f32,
    pub tailTimeOffset: f32,
}
#[cfg(feature = "TutorialSongController+TutorialChainSpawnData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController_TutorialChainSpawnData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/TutorialChainSpawnData";
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
#[cfg(feature = "TutorialSongController+TutorialChainSpawnData")]
impl std::ops::Deref
for crate::GlobalNamespace::TutorialSongController_TutorialChainSpawnData {
    type Target = crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialChainSpawnData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TutorialSongController_TutorialChainSpawnData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialChainSpawnData")]
impl crate::GlobalNamespace::TutorialSongController_TutorialChainSpawnData {
    pub fn New(
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        colorType: crate::GlobalNamespace::ColorType,
        headLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headCutDirection: crate::GlobalNamespace::NoteCutDirection,
        tailLineIndex: i32,
        tailLineLayer: crate::GlobalNamespace::NoteLineLayer,
        sliceCount: i32,
        squishAmount: f32,
        tailTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    signal,
                    firstTimeBeatOffset,
                    beatOffset,
                    lineIndex,
                    colorType,
                    headLineLayer,
                    headCutDirection,
                    tailLineIndex,
                    tailLineLayer,
                    sliceCount,
                    squishAmount,
                    tailTimeOffset,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        colorType: crate::GlobalNamespace::ColorType,
        headLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headCutDirection: crate::GlobalNamespace::NoteCutDirection,
        tailLineIndex: i32,
        tailLineLayer: crate::GlobalNamespace::NoteLineLayer,
        sliceCount: i32,
        squishAmount: f32,
        tailTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
                    i32,
                    i32,
                    i32,
                    crate::GlobalNamespace::ColorType,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteCutDirection,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    i32,
                    f32,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                12usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 12usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        signal,
                        firstTimeBeatOffset,
                        beatOffset,
                        lineIndex,
                        colorType,
                        headLineLayer,
                        headCutDirection,
                        tailLineIndex,
                        tailLineLayer,
                        sliceCount,
                        squishAmount,
                        tailTimeOffset,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_headLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_headLineIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_headLineIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController+TutorialChainSpawnData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController_TutorialChainSpawnData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialSongController+TutorialJumpingNoteSpawnData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_TutorialJumpingNoteSpawnData {
    __cordl_parent: crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData,
    pub noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
}
#[cfg(feature = "TutorialSongController+TutorialJumpingNoteSpawnData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/TutorialJumpingNoteSpawnData";
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
#[cfg(feature = "TutorialSongController+TutorialJumpingNoteSpawnData")]
impl std::ops::Deref
for crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData {
    type Target = crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialJumpingNoteSpawnData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialJumpingNoteSpawnData")]
impl crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData {
    pub fn New(
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
                    i32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController+TutorialJumpingNoteSpawnData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialSongController+TutorialObjectSpawnData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_TutorialObjectSpawnData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
    pub beatOffset: i32,
    pub firstTimeBeatOffset: i32,
    pub lineIndex: i32,
}
#[cfg(feature = "TutorialSongController+TutorialObjectSpawnData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/TutorialObjectSpawnData";
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
#[cfg(feature = "TutorialSongController+TutorialObjectSpawnData")]
impl std::ops::Deref
for crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialObjectSpawnData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialObjectSpawnData")]
impl crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData {
    pub fn New(
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signal, firstTimeBeatOffset, beatOffset, lineIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
                    i32,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (signal, firstTimeBeatOffset, beatOffset, lineIndex),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController+TutorialObjectSpawnData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialSongController+TutorialObstacleSpawnData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_TutorialObstacleSpawnData {
    __cordl_parent: crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData,
    pub noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub width: i32,
    pub height: i32,
}
#[cfg(feature = "TutorialSongController+TutorialObstacleSpawnData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TutorialSongController/TutorialObstacleSpawnData";
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
#[cfg(feature = "TutorialSongController+TutorialObstacleSpawnData")]
impl std::ops::Deref
for crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData {
    type Target = crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialObstacleSpawnData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController+TutorialObstacleSpawnData")]
impl crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData {
    pub fn New(
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        width: i32,
        height: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    signal,
                    firstTimeBeatOffset,
                    beatOffset,
                    lineIndex,
                    width,
                    height,
                    noteLineLayer,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        width: i32,
        height: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        signal,
                        firstTimeBeatOffset,
                        beatOffset,
                        lineIndex,
                        width,
                        height,
                        noteLineLayer,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSongController+TutorialObstacleSpawnData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
