#[cfg(feature = "BeatmapObjectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn AddSpawnedObstacleController(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn AddSpawnedSliderController(
        &mut self,
        sliderController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
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
    pub fn Despawn_NoteController0(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Despawn", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn Despawn_ObstacleController1(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Despawn", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn Despawn_SliderController2(
        &mut self,
        sliderNoteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Despawn", (sliderNoteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn DissolveAllObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DissolveAllObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteControllerNoteDidDissolve(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteDidDissolve", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteControllerNoteDidFinishJump(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteDidFinishJump", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteControllerNoteDidStartDissolving(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteControllerBase,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteControllerNoteDidStartJump(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteDidStartJump", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteControllerNoteWasCut(
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
            .invoke("HandleNoteControllerNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteControllerNoteWasMissed(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerNoteWasMissed", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstacleDidDissolve(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleDidDissolve", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstacleFinishedMovement(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleFinishedMovement", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstaclePassedAvoidedMark(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstaclePassedAvoidedMark", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstaclePassedThreeQuartersOfMove2(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstaclePassedThreeQuartersOfMove2", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSliderDidDissolve(
        &mut self,
        sliderController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderDidDissolve", (sliderController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSliderDidFinishJump(
        &mut self,
        sliderController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderDidFinishJump", (sliderController))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn INoteControllerNoteWasCutEvent_HandleNoteControllerNoteWasCut(
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
            .invoke(
                "INoteControllerNoteWasCutEvent.HandleNoteControllerNoteWasCut",
                (noteController, noteCutInfo),
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
    pub fn PauseAllBeatmapObjects(
        &mut self,
        pause: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseAllBeatmapObjects", (pause))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessNoteData(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessObstacleData(
        &mut self,
        obstacleData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSliderData(
        &mut self,
        sliderData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
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
        Ok(__cordl_ret.into())
    }
    pub fn RemoveNoteControllerEventCallbacks(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveNoteControllerEventCallbacks", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveObstacleEventCallbacks(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveObstacleEventCallbacks", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSliderNoteControllerEventCallbacks(
        &mut self,
        sliderNoteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSliderNoteControllerEventCallbacks", (sliderNoteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNoteControllerEventCallbacks(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNoteControllerEventCallbacks", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetObstacleEventCallbacks(
        &mut self,
        obstacleController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetObstacleEventCallbacks", (obstacleController))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSliderNoteControllerEventCallbacks(
        &mut self,
        sliderNoteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSliderNoteControllerEventCallbacks", (sliderNoteController))?;
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
    pub fn add_didHideAllBeatmapObjectsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didHideAllBeatmapObjectsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteDidStartDissolvingEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteControllerBase>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteDidStartDissolvingEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteDidStartJumpEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteDidStartJumpEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteWasAddedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::GlobalNamespace::NoteData,
                crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasAddedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteWasCutEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasCutEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteWasDespawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasDespawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteWasMissedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasMissedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_obstacleDidPassAvoidedMarkEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleDidPassAvoidedMarkEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_obstacleDidPassThreeQuartersOfMove2Event(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleDidPassThreeQuartersOfMove2Event", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_obstacleWasAddedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::GlobalNamespace::ObstacleData,
                crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleWasAddedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_obstacleWasDespawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleWasDespawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_obstacleWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sliderWasAddedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::GlobalNamespace::SliderData,
                crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sliderWasAddedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sliderWasDespawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::SliderController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sliderWasDespawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sliderWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::SliderController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sliderWasSpawnedEvent", (value))?;
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
    pub fn get_spawnHidden(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_spawnHidden", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didHideAllBeatmapObjectsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didHideAllBeatmapObjectsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteDidStartDissolvingEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteControllerBase>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteDidStartDissolvingEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteDidStartJumpEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteDidStartJumpEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteWasAddedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::GlobalNamespace::NoteData,
                crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasAddedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteWasCutEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectManager_NoteWasCutDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasCutEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteWasDespawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasDespawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteWasMissedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasMissedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::NoteController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_obstacleDidPassAvoidedMarkEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleDidPassAvoidedMarkEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_obstacleDidPassThreeQuartersOfMove2Event(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleDidPassThreeQuartersOfMove2Event", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_obstacleWasAddedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::GlobalNamespace::ObstacleData,
                crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleWasAddedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_obstacleWasDespawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleWasDespawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_obstacleWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sliderWasAddedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::GlobalNamespace::SliderData,
                crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sliderWasAddedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sliderWasDespawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::SliderController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sliderWasDespawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sliderWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::SliderController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BeatmapObjectManager")]
impl AsRef<crate::GlobalNamespace::IBeatmapObjectSpawner>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatmapObjectSpawner {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsMut<crate::GlobalNamespace::IBeatmapObjectSpawner>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatmapObjectSpawner {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsRef<crate::GlobalNamespace::INoteControllerNoteDidDissolveEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteControllerNoteDidDissolveEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsMut<crate::GlobalNamespace::INoteControllerNoteDidDissolveEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteControllerNoteDidDissolveEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsRef<crate::GlobalNamespace::INoteControllerNoteDidFinishJumpEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteControllerNoteDidFinishJumpEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsMut<crate::GlobalNamespace::INoteControllerNoteDidFinishJumpEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteControllerNoteDidFinishJumpEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsRef<crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsMut<crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsRef<crate::GlobalNamespace::INoteControllerNoteDidStartJumpEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteControllerNoteDidStartJumpEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsMut<crate::GlobalNamespace::INoteControllerNoteDidStartJumpEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteControllerNoteDidStartJumpEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsRef<crate::GlobalNamespace::INoteControllerNoteWasCutEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteControllerNoteWasCutEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsMut<crate::GlobalNamespace::INoteControllerNoteWasCutEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INoteControllerNoteWasCutEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsRef<crate::GlobalNamespace::INoteControllerNoteWasMissedEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteControllerNoteWasMissedEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsMut<crate::GlobalNamespace::INoteControllerNoteWasMissedEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteControllerNoteWasMissedEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsRef<crate::GlobalNamespace::ISliderDidDissolveEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::ISliderDidDissolveEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsMut<crate::GlobalNamespace::ISliderDidDissolveEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ISliderDidDissolveEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsRef<crate::GlobalNamespace::ISliderDidFinishJumpEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::ISliderDidFinishJumpEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapObjectManager")]
impl AsMut<crate::GlobalNamespace::ISliderDidFinishJumpEvent>
for crate::GlobalNamespace::BeatmapObjectManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ISliderDidFinishJumpEvent {
        unsafe { std::mem::transmute(self) }
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
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteCutInfo,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (noteController, noteCutInfo, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteCutInfo,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (noteCutInfo, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
            .invoke("Invoke", (noteController, noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
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
