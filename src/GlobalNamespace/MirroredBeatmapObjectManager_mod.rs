#[cfg(feature = "MirroredBeatmapObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MirroredBeatmapObjectManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapObjectManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectManager,
    >,
    pub _mirroredBasicGameNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MirroredGameNoteController,
        >,
    >,
    pub _mirroredBurstSliderHeadGameNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MirroredGameNoteController,
        >,
    >,
    pub _mirroredBurstSliderGameNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MirroredGameNoteController,
        >,
    >,
    pub _mirroredBombNotePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MirroredBombNoteController,
        >,
    >,
    pub _mirroredObstaclePoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MirroredObstacleController,
        >,
    >,
    pub _mirroredSlidersPoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            *mut crate::GlobalNamespace::MirroredSliderController,
        >,
    >,
    pub _gameNoteControllersToMirroredGameNoteControllers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::GlobalNamespace::IGameNoteMirrorable,
            crate::System::ValueTuple_2<
                *mut crate::GlobalNamespace::MirroredGameNoteController,
                *mut crate::GlobalNamespace::MemoryPoolContainer_1<
                    *mut crate::GlobalNamespace::MirroredGameNoteController,
                >,
            >,
        >,
    >,
    pub _bombNoteControllersToMirroredBombNoteControllers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::GlobalNamespace::INoteMirrorable,
            *mut crate::GlobalNamespace::MirroredBombNoteController,
        >,
    >,
    pub _obstacleControllersToMirroredObstacleControllers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::GlobalNamespace::ObstacleController,
            *mut crate::GlobalNamespace::MirroredObstacleController,
        >,
    >,
    pub _sliderControllersToMirroredSliderControllers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::GlobalNamespace::SliderController,
            *mut crate::GlobalNamespace::MirroredSliderController,
        >,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasDespawned(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasDespawned", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasSpawned", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstacleWasDespawned(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleWasDespawned", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstacleWasSpawned(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleWasSpawned", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSliderWasDespawned(
        &mut self,
        sliderController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderWasDespawned", (sliderController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSliderWasSpawned(
        &mut self,
        sliderController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderWasSpawned", (sliderController))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        beatmapObjectManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectManager,
        >,
        mirroredBasicGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MirroredGameNoteController_Pool,
        >,
        burstSliderHeadGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MirroredGameNoteController_Pool,
        >,
        burstSliderGameNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MirroredGameNoteController_Pool,
        >,
        mirroredBombNotePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MirroredBombNoteController_Pool,
        >,
        mirroredObstaclePool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MirroredObstacleController_Pool,
        >,
        mirroredSlidersPool: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MirroredSliderController_Pool,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InvalidateGameNotePools(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvalidateGameNotePools", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
