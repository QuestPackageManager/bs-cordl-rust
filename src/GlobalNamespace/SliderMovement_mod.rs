#[cfg(feature = "SliderMovement")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderMovement {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioTimeSyncController: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _playerTransforms: *mut crate::GlobalNamespace::PlayerTransforms,
    pub movementDidFinishEvent: *mut crate::System::Action,
    pub movementDidMoveEvent: *mut crate::System::Action_1<f32>,
    pub headDidMovePastCutMarkEvent: *mut crate::System::Action,
    pub tailDidMovePastCutMarkEvent: *mut crate::System::Action,
    pub _headNoteJumpStartPos: crate::UnityEngine::Vector3,
    pub _headNoteJumpEndPos: crate::UnityEngine::Vector3,
    pub _headNoteTime: f32,
    pub _tailNoteTime: f32,
    pub _localPosition: crate::UnityEngine::Vector3,
    pub _worldRotation: crate::UnityEngine::Quaternion,
    pub _inverseWorldRotation: crate::UnityEngine::Quaternion,
    pub _jumpDuration: f32,
    pub _headNoteGravity: f32,
    pub _tailNoteGravity: f32,
    pub _transform: *mut crate::UnityEngine::Transform,
    pub _movementEndReported: bool,
    pub _headDidMovePastCutMarkReported: bool,
    pub _tailDidMovePastCutMarkReported: bool,
    pub _timeSinceHeadNoteJump: f32,
}
#[cfg(feature = "SliderMovement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderMovement => ""
    ."SliderMovement"
);
#[cfg(feature = "SliderMovement")]
impl std::ops::Deref for crate::GlobalNamespace::SliderMovement {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMovement")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderMovement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMovement")]
impl crate::GlobalNamespace::SliderMovement {
    pub fn Init(
        &mut self,
        headNoteTime: f32,
        tailNoteTime: f32,
        worldRotation: f32,
        headNoteJumpStartPos: crate::UnityEngine::Vector3,
        headNoteJumpEndPos: crate::UnityEngine::Vector3,
        jumpDuration: f32,
        headNoteGravity: f32,
        tailNoteGravity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    headNoteTime,
                    tailNoteTime,
                    worldRotation,
                    headNoteJumpStartPos,
                    headNoteJumpEndPos,
                    jumpDuration,
                    headNoteGravity,
                    tailNoteGravity,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ManualUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn StartMovement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartMovement", ())?;
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
    pub fn add_headDidMovePastCutMarkEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_headDidMovePastCutMarkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_movementDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_movementDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_movementDidMoveEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_movementDidMoveEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_tailDidMovePastCutMarkEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_tailDidMovePastCutMarkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_headNoteGravity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_headNoteGravity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_jumpDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_tailNoteGravity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_tailNoteGravity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_timeSinceHeadNoteJump(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeSinceHeadNoteJump", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_headDidMovePastCutMarkEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_headDidMovePastCutMarkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_movementDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_movementDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_movementDidMoveEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_movementDidMoveEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_tailDidMovePastCutMarkEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_tailDidMovePastCutMarkEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SliderMovement")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SliderMovement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
