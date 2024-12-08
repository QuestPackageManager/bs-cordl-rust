#[cfg(feature = "SaberActivityCounter")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberActivityCounter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _averageWindowDuration: f32,
    pub _valuesPerSecond: f32,
    pub _increaseSpeed: f32,
    pub _decreaseSpeed: f32,
    pub _movementSensitivityThreshold: f32,
    pub _saberManager: *mut SaberManager,
    pub _gamePause: *mut IGamePause,
    pub totalDistanceDidChangeEvent: *mut crate::System::Action_1<f32>,
    pub _prevLeftSaberTipPos: crate::UnityEngine::Vector3,
    pub _prevRightSaberTipPos: crate::UnityEngine::Vector3,
    pub _prevLeftHandPos: crate::UnityEngine::Vector3,
    pub _prevRightHandPos: crate::UnityEngine::Vector3,
    pub _hasPrevPos: bool,
    pub _leftSaberMovementDistance: f32,
    pub _rightSaberMovementDistance: f32,
    pub _leftHandMovementDistance: f32,
    pub _rightHandMovementDistance: f32,
    pub _saberMovementHistoryRecorder: *mut MovementHistoryRecorder,
    pub _handMovementHistoryRecorder: *mut MovementHistoryRecorder,
}
#[cfg(feature = "SaberActivityCounter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SaberActivityCounter => ""."SaberActivityCounter"
);
#[cfg(feature = "SaberActivityCounter")]
impl std::ops::Deref for SaberActivityCounter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberActivityCounter")]
impl std::ops::DerefMut for SaberActivityCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberActivityCounter")]
impl SaberActivityCounter {
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
    pub fn HandleDidPauseEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidPauseEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSaberPositionsWereUpdated(
        &mut self,
        leftSaber: *mut Saber,
        rightSaber: *mut Saber,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSaberPositionsWereUpdated", (leftSaber, rightSaber))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn add_totalDistanceDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_totalDistanceDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_handMovementAveragingValueRecorder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut AveragingValueRecorder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut AveragingValueRecorder = __cordl_object
            .invoke("get_handMovementAveragingValueRecorder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftHandMovementDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_leftHandMovementDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftSaberMovementDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_leftSaberMovementDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightHandMovementDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_rightHandMovementDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightSaberMovementDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_rightSaberMovementDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_saberMovementAveragingValueRecorder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut AveragingValueRecorder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut AveragingValueRecorder = __cordl_object
            .invoke("get_saberMovementAveragingValueRecorder", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_totalDistanceDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_totalDistanceDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SaberActivityCounter")]
impl quest_hook::libil2cpp::ObjectType for SaberActivityCounter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
