#[cfg(feature = "BeatmapObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectManager {
    __cordl_parent: crate::System::Object,
    pub noteWasAddedEvent: *mut crate::System::Action_3<
        *mut crate::GlobalNamespace::NoteData,
        crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
        f32,
    >,
    pub noteWasSpawnedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::NoteController,
    >,
    pub noteWasDespawnedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::NoteController,
    >,
    pub noteWasMissedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::NoteController,
    >,
    pub noteWasCutEvent: *mut crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate,
    pub noteDidStartJumpEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::NoteController,
    >,
    pub noteDidStartDissolvingEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::NoteControllerBase,
    >,
    pub obstacleWasAddedEvent: *mut crate::System::Action_3<
        *mut crate::GlobalNamespace::ObstacleData,
        crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData,
        f32,
    >,
    pub obstacleWasSpawnedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::ObstacleController,
    >,
    pub obstacleWasDespawnedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::ObstacleController,
    >,
    pub obstacleDidPassThreeQuartersOfMove2Event: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::ObstacleController,
    >,
    pub obstacleDidPassAvoidedMarkEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::ObstacleController,
    >,
    pub sliderWasAddedEvent: *mut crate::System::Action_3<
        *mut crate::GlobalNamespace::SliderData,
        crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData,
        f32,
    >,
    pub sliderWasSpawnedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::SliderController,
    >,
    pub sliderWasDespawnedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::SliderController,
    >,
    pub didHideAllBeatmapObjectsEvent: *mut crate::System::Action_1<bool>,
    pub _allBeatmapObjects: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::IBeatmapObjectController,
    >,
    pub _spawnHidden_k__BackingField: bool,
}
#[cfg(feature = "BeatmapObjectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapObjectManager => ""
    ."BeatmapObjectManager"
);
#[cfg(feature = "BeatmapObjectManager")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl crate::GlobalNamespace::BeatmapObjectManager {
    #[cfg(feature = "BeatmapObjectManager+NoteWasCutDelegate")]
    pub type NoteWasCutDelegate = crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate;
    pub fn AddSpawnedNoteController(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
        noteSpawnData: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSpawnedNoteController",
                (noteController, noteSpawnData, rotation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSpawnedObstacleController(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
        obstacleSpawnData: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSpawnedObstacleController",
                (obstacleController, obstacleSpawnData, rotation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSpawnedSliderController(
        &mut self,
        sliderController: *mut crate::GlobalNamespace::SliderController,
        sliderSpawnData: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSpawnedSliderController",
                (sliderController, sliderSpawnData, rotation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DespawnInternal_NoteController0(
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
    pub fn DespawnInternal_ObstacleController1(
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
    pub fn Despawn_NoteController0(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Despawn", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn Despawn_ObstacleController1(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Despawn", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn Despawn_SliderController2(
        &mut self,
        sliderNoteController: *mut crate::GlobalNamespace::SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Despawn", (sliderNoteController))?;
        Ok(__cordl_ret)
    }
    pub fn DissolveAllObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DissolveAllObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteControllerNoteDidDissolve(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteDidDissolve", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteControllerNoteDidFinishJump(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteDidFinishJump", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteControllerNoteDidStartDissolving(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteControllerBase,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteControllerNoteDidStartDissolving",
                (noteController, duration),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteControllerNoteDidStartJump(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteDidStartJump", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteControllerNoteWasCut(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteControllerNoteWasMissed(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteWasMissed", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstacleDidDissolve(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleDidDissolve", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstacleFinishedMovement(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleFinishedMovement", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstaclePassedAvoidedMark(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstaclePassedAvoidedMark", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstaclePassedThreeQuartersOfMove2(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstaclePassedThreeQuartersOfMove2", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSliderDidDissolve(
        &mut self,
        sliderController: *mut crate::GlobalNamespace::SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderDidDissolve", (sliderController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSliderDidFinishJump(
        &mut self,
        sliderController: *mut crate::GlobalNamespace::SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderDidFinishJump", (sliderController))?;
        Ok(__cordl_ret)
    }
    pub fn HideAllBeatmapObjects(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideAllBeatmapObjects", (hide))?;
        Ok(__cordl_ret)
    }
    pub fn INoteControllerNoteWasCutEvent_HandleNoteControllerNoteWasCut(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "INoteControllerNoteWasCutEvent.HandleNoteControllerNoteWasCut",
                (noteController, noteCutInfo),
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
    pub fn PauseAllBeatmapObjects(
        &mut self,
        pause: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseAllBeatmapObjects", (pause))?;
        Ok(__cordl_ret)
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
    pub fn RemoveNoteControllerEventCallbacks(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveNoteControllerEventCallbacks", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveObstacleEventCallbacks(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveObstacleEventCallbacks", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveSliderNoteControllerEventCallbacks(
        &mut self,
        sliderNoteController: *mut crate::GlobalNamespace::SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSliderNoteControllerEventCallbacks", (sliderNoteController))?;
        Ok(__cordl_ret)
    }
    pub fn SetNoteControllerEventCallbacks(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNoteControllerEventCallbacks", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn SetObstacleEventCallbacks(
        &mut self,
        obstacleController: *mut crate::GlobalNamespace::ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetObstacleEventCallbacks", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn SetSliderNoteControllerEventCallbacks(
        &mut self,
        sliderNoteController: *mut crate::GlobalNamespace::SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSliderNoteControllerEventCallbacks", (sliderNoteController))?;
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
    pub fn add_didHideAllBeatmapObjectsEvent(
        &mut self,
        value: *mut crate::System::Action_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didHideAllBeatmapObjectsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteDidStartDissolvingEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::NoteControllerBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteDidStartDissolvingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteDidStartJumpEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteDidStartJumpEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteWasAddedEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::NoteData,
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
            f32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasAddedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteWasCutEvent(
        &mut self,
        value: *mut crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasCutEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteWasDespawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasDespawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteWasMissedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasMissedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_obstacleDidPassAvoidedMarkEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleDidPassAvoidedMarkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_obstacleDidPassThreeQuartersOfMove2Event(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleDidPassThreeQuartersOfMove2Event", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_obstacleWasAddedEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::ObstacleData,
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData,
            f32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleWasAddedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_obstacleWasDespawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleWasDespawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_obstacleWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_sliderWasAddedEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::SliderData,
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData,
            f32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sliderWasAddedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_sliderWasDespawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sliderWasDespawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_sliderWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sliderWasSpawnedEvent", (value))?;
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
    pub fn get_spawnHidden(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_spawnHidden", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didHideAllBeatmapObjectsEvent(
        &mut self,
        value: *mut crate::System::Action_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didHideAllBeatmapObjectsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteDidStartDissolvingEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::NoteControllerBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteDidStartDissolvingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteDidStartJumpEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteDidStartJumpEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteWasAddedEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::NoteData,
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
            f32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasAddedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteWasCutEvent(
        &mut self,
        value: *mut crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasCutEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteWasDespawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasDespawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteWasMissedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasMissedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_obstacleDidPassAvoidedMarkEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleDidPassAvoidedMarkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_obstacleDidPassThreeQuartersOfMove2Event(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleDidPassThreeQuartersOfMove2Event", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_obstacleWasAddedEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::ObstacleData,
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData,
            f32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleWasAddedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_obstacleWasDespawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleWasDespawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_obstacleWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_sliderWasAddedEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::SliderData,
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData,
            f32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sliderWasAddedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_sliderWasDespawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sliderWasDespawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_sliderWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_spawnHidden(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spawnHidden", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapObjectManager+NoteWasCutDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectManager_NoteWasCutDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "BeatmapObjectManager+NoteWasCutDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate => ""
    ."BeatmapObjectManager/NoteWasCutDelegate"
);
#[cfg(feature = "BeatmapObjectManager+NoteWasCutDelegate")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectManager+NoteWasCutDelegate")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectManager+NoteWasCutDelegate")]
impl crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate {
    pub fn BeginInvoke(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteCutInfo,
        >,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (noteController, noteCutInfo, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteCutInfo,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (noteCutInfo, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (noteController, noteCutInfo))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectManager+NoteWasCutDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
