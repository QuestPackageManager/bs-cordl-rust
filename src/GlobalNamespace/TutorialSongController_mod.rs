#[cfg(feature = "TutorialSongController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_InitData {
    __cordl_parent: crate::System::Object,
    pub songBpm: f32,
    pub beatmapData: *mut BeatmapData,
}
#[cfg(feature = "TutorialSongController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TutorialSongController_InitData
    => ""."TutorialSongController/InitData"
);
#[cfg(feature = "TutorialSongController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::TutorialSongController_InitData {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        songBpm: f32,
        beatmapData: *mut BeatmapData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (songBpm, beatmapData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        songBpm: f32,
        beatmapData: *mut BeatmapData,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (songBpm, beatmapData))?;
        Ok(__cordl_object)
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
#[cfg(feature = "TutorialSongController+TutorialBasicNoteSpawnData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_TutorialBasicNoteSpawnData {
    __cordl_parent: crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData,
    pub cutDirection: NoteCutDirection,
    pub colorType: ColorType,
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
    pub fn _ctor(
        &mut self,
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: NoteLineLayer,
        cutDirection: NoteCutDirection,
        colorType: ColorType,
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
        Ok(__cordl_ret)
    }
    pub fn New(
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: NoteLineLayer,
        cutDirection: NoteCutDirection,
        colorType: ColorType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
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
    pub fn _ctor(
        &mut self,
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
            )?;
        Ok(__cordl_object)
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
#[cfg(feature = "TutorialSongController+TutorialJumpingNoteSpawnData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController_TutorialJumpingNoteSpawnData {
    __cordl_parent: crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData,
    pub noteLineLayer: NoteLineLayer,
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
    pub fn _ctor(
        &mut self,
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        noteLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (signal, firstTimeBeatOffset, beatOffset, lineIndex, noteLineLayer),
            )?;
        Ok(__cordl_object)
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
    __cordl_parent: crate::System::Object,
    pub signal: *mut Signal,
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
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signal, firstTimeBeatOffset, beatOffset, lineIndex))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signal, firstTimeBeatOffset, beatOffset, lineIndex))?;
        Ok(__cordl_object)
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
    pub noteLineLayer: NoteLineLayer,
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
    pub fn _ctor(
        &mut self,
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        width: i32,
        height: i32,
        noteLineLayer: NoteLineLayer,
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
        Ok(__cordl_ret)
    }
    pub fn New(
        signal: *mut Signal,
        firstTimeBeatOffset: i32,
        beatOffset: i32,
        lineIndex: i32,
        width: i32,
        height: i32,
        noteLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
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
#[cfg(feature = "TutorialSongController")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSongController {
    __cordl_parent: SongController,
    pub _audioTimeSyncController: *mut AudioTimeSyncController,
    pub _startWaitTimeInBeats: i32,
    pub _numberOfBeatsToSnap: i32,
    pub _obstacleDurationInBeats: i32,
    pub _noteCuttingTutorialPartDidStartSignal: *mut Signal,
    pub _noteCuttingInAnyDirectionDidStartSignal: *mut Signal,
    pub _bombCuttingTutorialPartDidStartSignal: *mut Signal,
    pub _leftObstacleTutorialPartDidStartSignal: *mut Signal,
    pub _rightObstacleTutorialPartDidStartSignal: *mut Signal,
    pub _topObstacleTutorialPartDidStartSignal: *mut Signal,
    pub _noteWasCutOKSignal: *mut Signal,
    pub _noteWasCutTooSoonSignal: *mut Signal,
    pub _noteWasCutWithWrongColorSignal: *mut Signal,
    pub _noteWasCutFromDifferentDirectionSignal: *mut Signal,
    pub _noteWasCutWithSlowSpeedSignal: *mut Signal,
    pub _bombWasCutSignal: *mut Signal,
    pub _initData: *mut crate::GlobalNamespace::TutorialSongController_InitData,
    pub _beatmapObjectManager: *mut BeatmapObjectManager,
    pub _tutorialBeatmapObjectIndex: i32,
    pub _prevSpawnedBeatmapObjectIndex: i32,
    pub _songBpm: f32,
    pub _beatmapData: *mut BeatmapData,
    pub _normalModeTutorialObjectsSpawnData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData,
    >,
}
#[cfg(feature = "TutorialSongController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TutorialSongController => ""."TutorialSongController"
);
#[cfg(feature = "TutorialSongController")]
impl std::ops::Deref for TutorialSongController {
    type Target = SongController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController")]
impl std::ops::DerefMut for TutorialSongController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSongController")]
impl TutorialSongController {
    #[cfg(feature = "TutorialSongController+TutorialObjectSpawnData")]
    pub type TutorialObjectSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialObjectSpawnData;
    #[cfg(feature = "TutorialSongController+TutorialBasicNoteSpawnData")]
    pub type TutorialBasicNoteSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData;
    #[cfg(feature = "TutorialSongController+TutorialBombNoteSpawnData")]
    pub type TutorialBombNoteSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData;
    #[cfg(feature = "TutorialSongController+InitData")]
    pub type InitData = crate::GlobalNamespace::TutorialSongController_InitData;
    #[cfg(feature = "TutorialSongController+TutorialObstacleSpawnData")]
    pub type TutorialObstacleSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData;
    #[cfg(feature = "TutorialSongController+TutorialJumpingNoteSpawnData")]
    pub type TutorialJumpingNoteSpawnData = crate::GlobalNamespace::TutorialSongController_TutorialJumpingNoteSpawnData;
    pub fn CreateBombNoteData(
        &mut self,
        _cordl_time: f32,
        tutorialBombNoteSpawnData: *mut crate::GlobalNamespace::TutorialSongController_TutorialBombNoteSpawnData,
    ) -> quest_hook::libil2cpp::Result<*mut NoteData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut NoteData = __cordl_object
            .invoke("CreateBombNoteData", (_cordl_time, tutorialBombNoteSpawnData))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: *mut NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret)
    }
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
    pub fn HandleObstacleDidPassThreeQuartersOfMove2(
        &mut self,
        obstacleController: *mut ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleDidPassThreeQuartersOfMove2", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextBeatmapObjectTime(
        &mut self,
        beatOffset: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetNextBeatmapObjectTime", (beatOffset))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateBeatmapData(
        &mut self,
        noteTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBeatmapData", (noteTime))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateBasicNoteData(
        &mut self,
        _cordl_time: f32,
        tutorialBasicNoteSpawnData: *mut crate::GlobalNamespace::TutorialSongController_TutorialBasicNoteSpawnData,
    ) -> quest_hook::libil2cpp::Result<*mut NoteData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut NoteData = __cordl_object
            .invoke("CreateBasicNoteData", (_cordl_time, tutorialBasicNoteSpawnData))?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopSong", ())?;
        Ok(__cordl_ret)
    }
    pub fn PauseSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseSong", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResumeSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResumeSong", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasMissed(
        &mut self,
        noteController: *mut NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasMissed", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn CreateObstacleData(
        &mut self,
        _cordl_time: f32,
        tutorialObstacleSpawnData: *mut crate::GlobalNamespace::TutorialSongController_TutorialObstacleSpawnData,
    ) -> quest_hook::libil2cpp::Result<*mut ObstacleData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ObstacleData = __cordl_object
            .invoke("CreateObstacleData", (_cordl_time, tutorialObstacleSpawnData))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "TutorialSongController")]
impl quest_hook::libil2cpp::ObjectType for TutorialSongController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
