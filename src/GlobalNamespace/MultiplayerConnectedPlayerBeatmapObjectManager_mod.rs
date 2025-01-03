#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerBeatmapObjectManager {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectManager,
    pub _firstBasicNoteTime: crate::System::Nullable_1<f32>,
    pub _gameNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    >,
    pub _burstSliderHeadGameNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    >,
    pub _burstSliderGameNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    >,
    pub _bombNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController,
        >,
    >,
    pub _obstaclePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_2<
            *mut crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController,
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    >,
    pub _beatmapObjectEventManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayerBeatmapObjectEventManager,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData,
    >,
    pub _defaultBeatmapObjectSpawnMovementData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectSpawnMovementData,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager => ""
    ."MultiplayerConnectedPlayerBeatmapObjectManager"
);
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager {
    type Target = crate::GlobalNamespace::BeatmapObjectManager;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager {
    #[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager+InitData")]
    pub type InitData = crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData;
    pub fn AreNotesSame(
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
        noteCutInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreNotesSame", (noteController, noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnInternal_NoteController0(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnInternal", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnInternal_ObstacleController1(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnInternal", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnInternal_SliderController2(
        &mut self,
        sliderNoteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnInternal", (sliderNoteController))?;
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
    pub fn HandleMultiplayerBeatmapObjectEventManagerBeatmapObjectWasCut(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerBeatmapObjectEventManagerBeatmapObjectWasCut",
                (noteCutInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerBeatmapObjectEventManagerBeatmapObjectWasSpawned(
        &mut self,
        noteSpawnInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerBeatmapObjectEventManagerBeatmapObjectWasSpawned",
                (noteSpawnInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerBeatmapObjectEventManagerObstacleWasSpawned(
        &mut self,
        obstacleSpawnInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerBeatmapObjectEventManagerObstacleWasSpawned",
                (obstacleSpawnInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerBeatmapObjectEventManagerSliderWasSpawned(
        &mut self,
        sliderSpawnInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerBeatmapObjectEventManagerSliderWasSpawned",
                (sliderSpawnInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData,
        >,
        beatmapObjectEventManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayerBeatmapObjectEventManager,
        >,
        gameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        >,
        burstSliderHeadGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        >,
        burstSliderGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        >,
        bombNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool,
        >,
        obstaclePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool,
        >,
        variableMovementDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VariableMovementDataProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
                    variableMovementDataProvider,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessNoteData(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteSpawnData,
        >,
        forceIsFirstNoteBehaviour: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessNoteData",
                (noteData, noteSpawnData, forceIsFirstNoteBehaviour),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessObstacleData(
        &mut self,
        obstacleData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
        obstacleSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::ObstacleSpawnData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessObstacleData", (obstacleData, obstacleSpawnData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSliderData(
        &mut self,
        sliderData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        sliderSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::SliderSpawnData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSliderData", (sliderData, sliderSpawnData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager_InitData,
        >,
        beatmapObjectEventManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayerBeatmapObjectEventManager,
        >,
        gameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        >,
        burstSliderHeadGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        >,
        burstSliderGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController_Pool,
        >,
        bombNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController_Pool,
        >,
        obstaclePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool,
        >,
        variableMovementDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VariableMovementDataProvider,
        >,
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
                    variableMovementDataProvider,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeObstacleControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::ObstacleController,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::ObstacleController,
            >,
        > = __cordl_object.invoke("get_activeObstacleControllers", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectManager {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerBeatmapObjectManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerBeatmapObjectManager_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub disappearingArrows: bool,
    pub ghostNotes: bool,
    pub notesUniformScale: f32,
    pub noteJumpMovementSpeed: f32,
    pub bpm: f32,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        noteJumpMovementSpeed: f32,
        bpm: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    disappearingArrows,
                    ghostNotes,
                    notesUniformScale,
                    noteJumpMovementSpeed,
                    bpm,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        disappearingArrows: bool,
        ghostNotes: bool,
        notesUniformScale: f32,
        noteJumpMovementSpeed: f32,
        bpm: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    disappearingArrows,
                    ghostNotes,
                    notesUniformScale,
                    noteJumpMovementSpeed,
                    bpm,
                ),
            )?;
        Ok(__cordl_ret.into())
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
