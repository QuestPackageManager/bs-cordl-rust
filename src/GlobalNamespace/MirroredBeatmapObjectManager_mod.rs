#[cfg(feature = "MirroredBeatmapObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MirroredBeatmapObjectManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
    pub _mirroredBasicGameNotePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::MirroredGameNoteController,
    >,
    pub _mirroredBurstSliderHeadGameNotePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::MirroredGameNoteController,
    >,
    pub _mirroredBurstSliderGameNotePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::MirroredGameNoteController,
    >,
    pub _mirroredBombNotePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::MirroredBombNoteController,
    >,
    pub _mirroredObstaclePoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::MirroredObstacleController,
    >,
    pub _mirroredSlidersPoolContainer: *mut crate::GlobalNamespace::MemoryPoolContainer_1<
        *mut crate::GlobalNamespace::MirroredSliderController,
    >,
    pub _gameNoteControllersToMirroredGameNoteControllers: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::GlobalNamespace::IGameNoteMirrorable,
        crate::System::ValueTuple_2<
            *mut crate::GlobalNamespace::MirroredGameNoteController,
            *mut crate::GlobalNamespace::MemoryPoolContainer_1<
                *mut crate::GlobalNamespace::MirroredGameNoteController,
            >,
        >,
    >,
    pub _bombNoteControllersToMirroredBombNoteControllers: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::GlobalNamespace::INoteMirrorable,
        *mut crate::GlobalNamespace::MirroredBombNoteController,
    >,
    pub _obstacleControllersToMirroredObstacleControllers: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::GlobalNamespace::ObstacleController,
        *mut crate::GlobalNamespace::MirroredObstacleController,
    >,
    pub _sliderControllersToMirroredSliderControllers: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::GlobalNamespace::SliderController,
        *mut crate::GlobalNamespace::MirroredSliderController,
    >,
}
#[cfg(feature = "MirroredBeatmapObjectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MirroredBeatmapObjectManager =>
    ""."MirroredBeatmapObjectManager"
);
#[cfg(feature = "MirroredBeatmapObjectManager")]
impl std::ops::Deref for crate::GlobalNamespace::MirroredBeatmapObjectManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredBeatmapObjectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MirroredBeatmapObjectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredBeatmapObjectManager")]
impl crate::GlobalNamespace::MirroredBeatmapObjectManager {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleDidHideAllBeatmapObjects(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidHideAllBeatmapObjects", (hide))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasDespawned(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasDespawned", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasSpawned", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstacleWasDespawned(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleWasDespawned", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstacleWasSpawned(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleWasSpawned", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSliderWasDespawned(
        &mut self,
        sliderController: *mut crate::GlobalNamespace::SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderWasDespawned", (sliderController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSliderWasSpawned(
        &mut self,
        sliderController: *mut crate::GlobalNamespace::SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderWasSpawned", (sliderController))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
        mirroredBasicGameNotePool: *mut crate::GlobalNamespace::MirroredGameNoteController_Pool,
        burstSliderHeadGameNotePool: *mut crate::GlobalNamespace::MirroredGameNoteController_Pool,
        burstSliderGameNotePool: *mut crate::GlobalNamespace::MirroredGameNoteController_Pool,
        mirroredBombNotePool: *mut crate::GlobalNamespace::MirroredBombNoteController_Pool,
        mirroredObstaclePool: *mut crate::GlobalNamespace::MirroredObstacleController_Pool,
        mirroredSlidersPool: *mut crate::GlobalNamespace::MirroredSliderController_Pool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    beatmapObjectManager,
                    mirroredBasicGameNotePool,
                    burstSliderHeadGameNotePool,
                    burstSliderGameNotePool,
                    mirroredBombNotePool,
                    mirroredObstaclePool,
                    mirroredSlidersPool,
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
}
#[cfg(feature = "MirroredBeatmapObjectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MirroredBeatmapObjectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
