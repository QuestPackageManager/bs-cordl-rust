#[cfg(feature = "MirroredBeatmapObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MirroredBeatmapObjectManager {
    __cordl_parent: crate::System::Object,
    pub _beatmapObjectManager: *mut BeatmapObjectManager,
    pub _mirroredBasicGameNotePoolContainer: *mut MemoryPoolContainer_1<
        *mut MirroredGameNoteController,
    >,
    pub _mirroredBurstSliderHeadGameNotePoolContainer: *mut MemoryPoolContainer_1<
        *mut MirroredGameNoteController,
    >,
    pub _mirroredBurstSliderGameNotePoolContainer: *mut MemoryPoolContainer_1<
        *mut MirroredGameNoteController,
    >,
    pub _mirroredBombNotePoolContainer: *mut MemoryPoolContainer_1<
        *mut MirroredBombNoteController,
    >,
    pub _mirroredObstaclePoolContainer: *mut MemoryPoolContainer_1<
        *mut MirroredObstacleController,
    >,
    pub _mirroredSlidersPoolContainer: *mut MemoryPoolContainer_1<
        *mut MirroredSliderController,
    >,
    pub _gameNoteControllersToMirroredGameNoteControllers: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut IGameNoteMirrorable,
        crate::System::ValueTuple_2<
            *mut MirroredGameNoteController,
            *mut MemoryPoolContainer_1<*mut MirroredGameNoteController>,
        >,
    >,
    pub _bombNoteControllersToMirroredBombNoteControllers: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut INoteMirrorable,
        *mut MirroredBombNoteController,
    >,
    pub _obstacleControllersToMirroredObstacleControllers: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut ObstacleController,
        *mut MirroredObstacleController,
    >,
    pub _sliderControllersToMirroredSliderControllers: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut SliderController,
        *mut MirroredSliderController,
    >,
}
#[cfg(feature = "MirroredBeatmapObjectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MirroredBeatmapObjectManager => ""
    ."MirroredBeatmapObjectManager"
);
#[cfg(feature = "MirroredBeatmapObjectManager")]
impl std::ops::Deref for MirroredBeatmapObjectManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredBeatmapObjectManager")]
impl std::ops::DerefMut for MirroredBeatmapObjectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredBeatmapObjectManager")]
impl MirroredBeatmapObjectManager {
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
        noteController: *mut NoteController,
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
        noteController: *mut NoteController,
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
        obstacleController: *mut ObstacleController,
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
        obstacleController: *mut ObstacleController,
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
        sliderController: *mut SliderController,
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
        sliderController: *mut SliderController,
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
        beatmapObjectManager: *mut BeatmapObjectManager,
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
impl quest_hook::libil2cpp::ObjectType for MirroredBeatmapObjectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}