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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SequenceCutInfo_TutorialSongController_NoteType => ""
    ."TutorialSongController/SequenceCutInfo/NoteType"
);
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
            *mut crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData,
        >,
    >,
}
#[cfg(feature = "TutorialSongController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TutorialSongController => ""
    ."TutorialSongController"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArcData(
        &mut self,
        headData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        tailData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData> = __cordl_object
            .invoke("CreateArcData", (headData, tailData))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData> = __cordl_object
            .invoke(
                "CreateBasicNoteData",
                (_cordl_time, beat, tutorialBasicNoteSpawnData),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData> = __cordl_object
            .invoke(
                "CreateBombNoteData",
                (_cordl_time, beat, tutorialBombNoteSpawnData),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        > = __cordl_object
            .invoke(
                "CreateChainData",
                (_cordl_time, tailTime, beat, tutorialChainSpawnData),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleData,
        > = __cordl_object
            .invoke(
                "CreateObstacleData",
                (_cordl_time, beat, tutorialObstacleSpawnData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextBeatmapObjectBeat(
        &mut self,
        beatOffset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetNextBeatmapObjectBeat", (beatOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeFromBeat(
        &mut self,
        beatNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetTimeFromBeat", (beatNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasMissed(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasMissed", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstacleDidPassThreeQuartersOfMove2(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleDidPassThreeQuartersOfMove2", (obstacleController))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PauseSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseSong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseSignalForIncorrectCutSequence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseSignalForIncorrectCutSequence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseSignalsForIndividualCut(
        &mut self,
        noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseSignalsForIndividualCut", (noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResumeSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResumeSong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartSong(
        &mut self,
        startTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSong", (startTimeOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopSong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBeatmapData(
        &mut self,
        noteTime: f32,
        noteBeat: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBeatmapData", (noteTime, noteBeat))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TutorialSongController_InitData
    => ""."TutorialSongController/InitData"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (songBpm, beatmapData))?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TutorialSongController_SequenceCutInfo => ""
    ."TutorialSongController/SequenceCutInfo"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkCut", (noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkMiss(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkMiss", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cuttableObjectsCount, noteType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allIsOK(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_allIsOK", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFinished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_missedAny(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_missedAny", ())?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TutorialSongController_TutorialArcSpawnData => ""
    ."TutorialSongController/TutorialArcSpawnData"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signal, headNote, tailNote))?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData => ""
    ."TutorialSongController/TutorialBasicNoteSpawnData"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData => ""
    ."TutorialSongController/TutorialBombNoteSpawnData"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
            )?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TutorialSongController_TutorialChainSpawnData => ""
    ."TutorialSongController/TutorialChainSpawnData"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        Ok(__cordl_ret.into())
    }
    pub fn get_headLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_headLineIndex", ())?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData => ""
    ."TutorialSongController/TutorialJumpingNoteSpawnData"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
            )?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData => ""
    ."TutorialSongController/TutorialObjectSpawnData"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signal, firstTimeBeatOffset, beatOffset, lineIndex))?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData => ""
    ."TutorialSongController/TutorialObstacleSpawnData"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
