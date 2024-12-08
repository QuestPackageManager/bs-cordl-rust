#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerBeatmapObjectManager_InitData {
    __cordl_parent: crate::System::Object,
    pub disappearingArrows: bool,
    pub ghostNotes: bool,
    pub notesUniformScale: f32,
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData => ""
    ."MultiplayerConnectedPlayerBeatmapObjectManager/InitData"
);
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager+InitData")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager+InitData")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData {
    pub fn New(
        disappearingArrows: bool,
        ghostNotes: bool,
        notesUniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (disappearingArrows, ghostNotes, notesUniformScale))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        disappearingArrows: bool,
        ghostNotes: bool,
        notesUniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (disappearingArrows, ghostNotes, notesUniformScale))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerBeatmapObjectManager {
    __cordl_parent: BeatmapObjectManager,
    pub _firstBasicNoteTime: crate::System::Nullable_1<f32>,
    pub _gameNotePoolContainer: *mut MemoryPoolContainer_1<
        *mut MultiplayerConnectedPlayerGameNoteController,
    >,
    pub _burstSliderHeadGameNotePoolContainer: *mut MemoryPoolContainer_1<
        *mut MultiplayerConnectedPlayerGameNoteController,
    >,
    pub _burstSliderGameNotePoolContainer: *mut MemoryPoolContainer_1<
        *mut MultiplayerConnectedPlayerGameNoteController,
    >,
    pub _bombNotePoolContainer: *mut MemoryPoolContainer_1<
        *mut MultiplayerConnectedPlayerBombNoteController,
    >,
    pub _obstaclePoolContainer: *mut MemoryPoolContainer_2<
        *mut MultiplayerConnectedPlayerObstacleController,
        *mut ObstacleController,
    >,
    pub _beatmapObjectEventManager: *mut IConnectedPlayerBeatmapObjectEventManager,
    pub _initData: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData,
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerConnectedPlayerBeatmapObjectManager => ""
    ."MultiplayerConnectedPlayerBeatmapObjectManager"
);
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl std::ops::Deref for MultiplayerConnectedPlayerBeatmapObjectManager {
    type Target = BeatmapObjectManager;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl std::ops::DerefMut for MultiplayerConnectedPlayerBeatmapObjectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl MultiplayerConnectedPlayerBeatmapObjectManager {
    #[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager+InitData")]
    pub type InitData = crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData;
    pub fn DespawnInternal_NoteController0(
        &mut self,
        noteController: *mut NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnInternal", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn DespawnInternal_ObstacleController1(
        &mut self,
        obstacleController: *mut ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnInternal", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn DespawnInternal_SliderController2(
        &mut self,
        sliderNoteController: *mut SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnInternal", (sliderNoteController))?;
        Ok(__cordl_ret)
    }
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
    pub fn HandleMultiplayerBeatmapObjectEventManagerBeatmapObjectWasCut(
        &mut self,
        noteCutInfo: *mut NoteCutInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerBeatmapObjectEventManagerBeatmapObjectWasCut",
                (noteCutInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerBeatmapObjectEventManagerBeatmapObjectWasSpawned(
        &mut self,
        noteSpawnInfo: *mut NoteSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerBeatmapObjectEventManagerBeatmapObjectWasSpawned",
                (noteSpawnInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerBeatmapObjectEventManagerObstacleWasSpawned(
        &mut self,
        obstacleSpawnInfo: *mut ObstacleSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerBeatmapObjectEventManagerObstacleWasSpawned",
                (obstacleSpawnInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerBeatmapObjectEventManagerSliderWasSpawned(
        &mut self,
        sliderSpawnInfo: *mut SliderSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerBeatmapObjectEventManagerSliderWasSpawned",
                (sliderSpawnInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initData: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData,
        beatmapObjectEventManager: *mut IConnectedPlayerBeatmapObjectEventManager,
        gameNotePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        burstSliderHeadGameNotePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        burstSliderGameNotePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        bombNotePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool,
        obstaclePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    initData,
                    beatmapObjectEventManager,
                    gameNotePool,
                    burstSliderHeadGameNotePool,
                    burstSliderGameNotePool,
                    bombNotePool,
                    obstaclePool,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn ProcessNoteData(
        &mut self,
        noteData: *mut NoteData,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
        >,
        rotation: f32,
        forceIsFirstNoteBehaviour: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessNoteData",
                (noteData, noteSpawnData, rotation, forceIsFirstNoteBehaviour),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessObstacleData(
        &mut self,
        obstacleData: *mut ObstacleData,
        obstacleSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData,
        >,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessObstacleData", (obstacleData, obstacleSpawnData, rotation))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessSliderData(
        &mut self,
        sliderData: *mut SliderData,
        sliderSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData,
        >,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSliderData", (sliderData, sliderSpawnData, rotation))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        initData: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData,
        beatmapObjectEventManager: *mut IConnectedPlayerBeatmapObjectEventManager,
        gameNotePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        burstSliderHeadGameNotePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        burstSliderGameNotePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        bombNotePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool,
        obstaclePool: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    initData,
                    beatmapObjectEventManager,
                    gameNotePool,
                    burstSliderHeadGameNotePool,
                    burstSliderGameNotePool,
                    bombNotePool,
                    obstaclePool,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_activeObstacleControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut ObstacleController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut ObstacleController,
        > = __cordl_object.invoke("get_activeObstacleControllers", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl quest_hook::libil2cpp::ObjectType
for MultiplayerConnectedPlayerBeatmapObjectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}