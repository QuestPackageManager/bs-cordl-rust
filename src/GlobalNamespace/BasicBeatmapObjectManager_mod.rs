#[cfg(feature = "BasicBeatmapObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicBeatmapObjectManager {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectManager,
    pub _firstBasicNoteTime: crate::System::Nullable_1<f32>,
    pub _initData: *mut crate::GlobalNamespace::BasicBeatmapObjectManager_InitData,
    pub _basicGameNotePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::GameNoteController,
    >,
    pub _burstSliderHeadGameNotePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::GameNoteController,
    >,
    pub _burstSliderGameNotePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::BurstSliderGameNoteController,
    >,
    pub _bombNotePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::BombNoteController,
    >,
    pub _obstaclePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::ObstacleController,
    >,
    pub _sliderNotePoolContainersDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::GlobalNamespace::SliderController_LengthType,
        *mut crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::SliderController,
        >,
    >,
}
#[cfg(feature = "BasicBeatmapObjectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BasicBeatmapObjectManager => ""
    ."BasicBeatmapObjectManager"
);
#[cfg(feature = "BasicBeatmapObjectManager")]
impl std::ops::Deref for crate::GlobalNamespace::BasicBeatmapObjectManager {
    type Target = crate::GlobalNamespace::BeatmapObjectManager;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BasicBeatmapObjectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::BasicBeatmapObjectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BasicBeatmapObjectManager")]
impl crate::GlobalNamespace::BasicBeatmapObjectManager {
    #[cfg(feature = "BasicBeatmapObjectManager+InitData")]
    pub type InitData = crate::GlobalNamespace::BasicBeatmapObjectManager_InitData;
    pub fn DespawnInternal_NoteController1(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnInternal", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn DespawnInternal_ObstacleController0(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
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
        sliderNoteController: *mut crate::GlobalNamespace::SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnInternal", (sliderNoteController))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        initData: *mut crate::GlobalNamespace::BasicBeatmapObjectManager_InitData,
        basicGameNotePool: *mut crate::GlobalNamespace::GameNoteController_Pool,
        burstSliderHeadGameNotePool: *mut crate::GlobalNamespace::GameNoteController_Pool,
        burstSliderGameNotePool: *mut crate::GlobalNamespace::BurstSliderGameNoteController_Pool,
        bombNotePool: *mut crate::GlobalNamespace::BombNoteController_Pool,
        obstaclePool: *mut crate::GlobalNamespace::ObstacleController_Pool,
        sliderPools: *mut crate::GlobalNamespace::SliderController_Pool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    initData,
                    basicGameNotePool,
                    burstSliderHeadGameNotePool,
                    burstSliderGameNotePool,
                    bombNotePool,
                    obstaclePool,
                    sliderPools,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessNoteData(
        &mut self,
        noteData: *mut crate::GlobalNamespace::NoteData,
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
        obstacleData: *mut crate::GlobalNamespace::ObstacleData,
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
        sliderData: *mut crate::GlobalNamespace::SliderData,
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_activeObstacleControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::ObstacleController,
        > = __cordl_object.invoke("get_activeObstacleControllers", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BasicBeatmapObjectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BasicBeatmapObjectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BasicBeatmapObjectManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicBeatmapObjectManager_InitData {
    __cordl_parent: crate::System::Object,
    pub disappearingArrows: bool,
    pub ghostNotes: bool,
    pub cutAngleTolerance: f32,
    pub notesUniformScale: f32,
}
#[cfg(feature = "BasicBeatmapObjectManager+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BasicBeatmapObjectManager_InitData => ""
    ."BasicBeatmapObjectManager/InitData"
);
#[cfg(feature = "BasicBeatmapObjectManager+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BasicBeatmapObjectManager_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BasicBeatmapObjectManager+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BasicBeatmapObjectManager_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BasicBeatmapObjectManager+InitData")]
impl crate::GlobalNamespace::BasicBeatmapObjectManager_InitData {
    pub fn New(
        disappearingArrows: bool,
        ghostNotes: bool,
        cutAngleTolerance: f32,
        notesUniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (disappearingArrows, ghostNotes, cutAngleTolerance, notesUniformScale),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        disappearingArrows: bool,
        ghostNotes: bool,
        cutAngleTolerance: f32,
        notesUniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (disappearingArrows, ghostNotes, cutAngleTolerance, notesUniformScale),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BasicBeatmapObjectManager+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BasicBeatmapObjectManager_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
