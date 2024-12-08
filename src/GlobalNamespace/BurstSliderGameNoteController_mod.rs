#[cfg(feature = "BurstSliderGameNoteController")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderGameNoteController {
    __cordl_parent: NoteController,
    pub _bigCuttableBySaberList: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BoxCuttableBySaber,
    >,
    pub _smallCuttableBySaberList: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BoxCuttableBySaber,
    >,
    pub _wrapperGO: *mut crate::UnityEngine::GameObject,
    pub _audioTimeSyncController: *mut AudioTimeSyncController,
    pub cubeNoteControllerDidInitEvent: *mut crate::System::Action_1<
        *mut BurstSliderGameNoteController,
    >,
    pub _noteVisualModifierType: NoteVisualModifierType,
    pub _gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
    pub _bigCuttableBySaber: *mut BoxCuttableBySaber,
    pub _originalColliderSize: crate::UnityEngine::Vector3,
    pub _originalColliderCenter: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BurstSliderGameNoteController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BurstSliderGameNoteController => ""
    ."BurstSliderGameNoteController"
);
#[cfg(feature = "BurstSliderGameNoteController")]
impl std::ops::Deref for BurstSliderGameNoteController {
    type Target = NoteController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl std::ops::DerefMut for BurstSliderGameNoteController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderGameNoteController")]
impl BurstSliderGameNoteController {
    #[cfg(feature = "BurstSliderGameNoteController+Pool")]
    pub type Pool = crate::GlobalNamespace::BurstSliderGameNoteController_Pool;
    pub fn Init(
        &mut self,
        noteData: *mut NoteData,
        worldRotation: f32,
        moveStartPos: crate::UnityEngine::Vector3,
        moveEndPos: crate::UnityEngine::Vector3,
        jumpEndPos: crate::UnityEngine::Vector3,
        moveDuration: f32,
        jumpDuration: f32,
        jumpGravity: f32,
        noteVisualModifierType: NoteVisualModifierType,
        uniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    noteData,
                    worldRotation,
                    moveStartPos,
                    moveEndPos,
                    jumpEndPos,
                    moveDuration,
                    jumpDuration,
                    jumpGravity,
                    noteVisualModifierType,
                    uniformScale,
                ),
            )?;
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
    pub fn NoteDidStartJump(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteDidStartJump", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleCut(
        &mut self,
        saber: *mut Saber,
        cutPoint: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        cutDirVec: crate::UnityEngine::Vector3,
        allowBadCut: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCut",
                (saber, cutPoint, orientation, cutDirVec, allowBadCut),
            )?;
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
    pub fn NoteDidPassMissedMarker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteDidPassMissedMarker", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBigWasCutBySaber(
        &mut self,
        saber: *mut Saber,
        cutPoint: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        cutDirVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBigWasCutBySaber",
                (saber, cutPoint, orientation, cutDirVec),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HiddenStateDidChange(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HiddenStateDidChange", (hide))?;
        Ok(__cordl_ret)
    }
    pub fn Pause(
        &mut self,
        pause: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", (pause))?;
        Ok(__cordl_ret)
    }
    pub fn get_noteMovement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut NoteMovement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut NoteMovement = __cordl_object
            .invoke("get_noteMovement", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_cubeNoteControllerDidInitEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut BurstSliderGameNoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cubeNoteControllerDidInitEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSmallWasCutBySaber(
        &mut self,
        saber: *mut Saber,
        cutPoint: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        cutDirVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSmallWasCutBySaber",
                (saber, cutPoint, orientation, cutDirVec),
            )?;
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
    pub fn remove_cubeNoteControllerDidInitEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut BurstSliderGameNoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cubeNoteControllerDidInitEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn NoteDidStartDissolving(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteDidStartDissolving", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_noteVisualModifierType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<NoteVisualModifierType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteVisualModifierType = __cordl_object
            .invoke("get_noteVisualModifierType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameplayType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteData_GameplayType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteData_GameplayType = __cordl_object
            .invoke("get_gameplayType", ())?;
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
#[cfg(feature = "BurstSliderGameNoteController")]
impl quest_hook::libil2cpp::ObjectType for BurstSliderGameNoteController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BurstSliderGameNoteController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderGameNoteController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut BurstSliderGameNoteController>,
}
#[cfg(feature = "BurstSliderGameNoteController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BurstSliderGameNoteController_Pool => ""
    ."BurstSliderGameNoteController/Pool"
);
#[cfg(feature = "BurstSliderGameNoteController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::BurstSliderGameNoteController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut BurstSliderGameNoteController>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderGameNoteController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::BurstSliderGameNoteController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderGameNoteController+Pool")]
impl crate::GlobalNamespace::BurstSliderGameNoteController_Pool {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BurstSliderGameNoteController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BurstSliderGameNoteController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
