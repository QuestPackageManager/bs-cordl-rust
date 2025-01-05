#[cfg(feature = "BasicBeatmapObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicBeatmapObjectManager {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectManager,
    pub _sliderControllerPool: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SliderController_Pool,
    >,
    pub _variableMovementDataProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VariableMovementDataProvider,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BasicBeatmapObjectManager_InitData,
    >,
    pub _random: quest_hook::libil2cpp::Gc<crate::System::Random>,
    pub _basicGameNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameNoteController>,
        >,
    >,
    pub _burstSliderHeadGameNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameNoteController>,
        >,
    >,
    pub _burstSliderGameNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BurstSliderGameNoteController,
            >,
        >,
    >,
    pub _bombNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BombNoteController>,
        >,
    >,
    pub _obstaclePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleController>,
        >,
    >,
    pub _sliderNotePoolContainersDictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::SliderController_LengthType,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MemoryPoolContainer_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderController>,
                >,
            >,
        >,
    >,
    pub _firstBasicNoteTime: crate::System::Nullable_1<f32>,
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
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnInternal", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnInternal_ObstacleController0(
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
    pub fn Init(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapObjectManager_InitData,
        >,
        random: quest_hook::libil2cpp::Gc<crate::System::Random>,
        variableMovementDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VariableMovementDataProvider,
        >,
        basicGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameNoteController_Pool,
        >,
        burstSliderHeadGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameNoteController_Pool,
        >,
        burstSliderGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BurstSliderGameNoteController_Pool,
        >,
        bombNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BombNoteController_Pool,
        >,
        obstaclePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController_Pool,
        >,
        sliderPools: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController_Pool,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    initData,
                    random,
                    variableMovementDataProvider,
                    basicGameNotePool,
                    burstSliderHeadGameNotePool,
                    burstSliderGameNotePool,
                    bombNotePool,
                    obstaclePool,
                    sliderPools,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    pub fn __InvalidateBombNotePool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__InvalidateBombNotePool", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __InvalidateGameNotePools(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__InvalidateGameNotePools", ())?;
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
    pub fn get_activeObstacleControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleController>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleController>,
            >,
        > = __cordl_object.invoke("get_activeObstacleControllers", ())?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (disappearingArrows, ghostNotes, cutAngleTolerance, notesUniformScale),
            )?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
