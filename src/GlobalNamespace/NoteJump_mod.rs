#[cfg(feature = "NoteJump")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteJump {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _rotatedObject: *mut crate::UnityEngine::Transform,
    pub _yAvoidanceUp: f32,
    pub _yAvoidanceDown: f32,
    pub _endDistanceOffset: f32,
    pub _playerTransforms: *mut PlayerTransforms,
    pub _playerSpaceConvertor: *mut PlayerSpaceConvertor,
    pub _audioTimeSyncController: *mut IAudioTimeSource,
    pub noteJumpDidFinishEvent: *mut crate::System::Action,
    pub noteJumpDidPassMissedMarkerEvent: *mut crate::System::Action,
    pub noteJumpDidPassThreeQuartersEvent: *mut crate::System::Action_1<*mut NoteJump>,
    pub noteJumpDidPassHalfEvent: *mut crate::System::Action,
    pub noteJumpDidUpdateProgressEvent: *mut crate::System::Action_1<f32>,
    pub _startPos: crate::UnityEngine::Vector3,
    pub _endPos: crate::UnityEngine::Vector3,
    pub _jumpDuration: f32,
    pub _moveVec: crate::UnityEngine::Vector3,
    pub _beatTime: f32,
    pub _startVerticalVelocity: f32,
    pub _startRotation: crate::UnityEngine::Quaternion,
    pub _middleRotation: crate::UnityEngine::Quaternion,
    pub _endRotation: crate::UnityEngine::Quaternion,
    pub _gravity: f32,
    pub _yAvoidance: f32,
    pub _missedTime: f32,
    pub _missedMarkReported: bool,
    pub _threeQuartersMarkReported: bool,
    pub _halfJumpMarkReported: bool,
    pub _localPosition: crate::UnityEngine::Vector3,
    pub _randomRotations: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
    pub _worldRotation: crate::UnityEngine::Quaternion,
    pub _inverseWorldRotation: crate::UnityEngine::Quaternion,
    pub _rotateTowardsPlayer: bool,
}
#[cfg(feature = "NoteJump")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NoteJump => ""."NoteJump"
);
#[cfg(feature = "NoteJump")]
impl std::ops::Deref for NoteJump {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJump")]
impl std::ops::DerefMut for NoteJump {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJump")]
impl NoteJump {
    pub const kMissedTimeOffset: f32 = 0.15f32;
    pub fn Init(
        &mut self,
        beatTime: f32,
        worldRotation: f32,
        startPos: crate::UnityEngine::Vector3,
        endPos: crate::UnityEngine::Vector3,
        jumpDuration: f32,
        gravity: f32,
        flipYSide: f32,
        endRotation: f32,
        rotateTowardsPlayer: bool,
        useRandomRotation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    beatTime,
                    worldRotation,
                    startPos,
                    endPos,
                    jumpDuration,
                    gravity,
                    flipYSide,
                    endRotation,
                    rotateTowardsPlayer,
                    useRandomRotation,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ManualUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
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
    pub fn add_noteJumpDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteJumpDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteJumpDidPassHalfEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteJumpDidPassHalfEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteJumpDidPassMissedMarkerEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteJumpDidPassMissedMarkerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteJumpDidPassThreeQuartersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteJump>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteJumpDidPassThreeQuartersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteJumpDidUpdateProgressEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteJumpDidUpdateProgressEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_beatPos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_distanceToPlayer(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_distanceToPlayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_jumpDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_localPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_moveVec(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_moveVec", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteJumpDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteJumpDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteJumpDidPassHalfEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteJumpDidPassHalfEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteJumpDidPassMissedMarkerEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteJumpDidPassMissedMarkerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteJumpDidPassThreeQuartersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteJump>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteJumpDidPassThreeQuartersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteJumpDidUpdateProgressEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteJumpDidUpdateProgressEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NoteJump")]
impl quest_hook::libil2cpp::ObjectType for NoteJump {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
