#[cfg(feature = "SaberActivityCounter")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberActivityCounter {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _averageWindowDuration: f32,
    pub _valuesPerSecond: f32,
    pub _increaseSpeed: f32,
    pub _decreaseSpeed: f32,
    pub _movementSensitivityThreshold: f32,
    pub _saberManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SaberManager>,
    pub _gamePause: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IGamePause>,
    pub totalDistanceDidChangeEvent: quest_hook::libil2cpp::Gc<f32>,
    pub _prevLeftSaberTipPos: crate::UnityEngine::Vector3,
    pub _prevRightSaberTipPos: crate::UnityEngine::Vector3,
    pub _prevLeftHandPos: crate::UnityEngine::Vector3,
    pub _prevRightHandPos: crate::UnityEngine::Vector3,
    pub _hasPrevPos: bool,
    pub _leftSaberMovementDistance: f32,
    pub _rightSaberMovementDistance: f32,
    pub _leftHandMovementDistance: f32,
    pub _rightHandMovementDistance: f32,
    pub _saberMovementHistoryRecorder: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MovementHistoryRecorder,
    >,
    pub _handMovementHistoryRecorder: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MovementHistoryRecorder,
    >,
}
#[cfg(feature = "SaberActivityCounter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberActivityCounter => ""
    ."SaberActivityCounter"
);
#[cfg(feature = "SaberActivityCounter")]
impl std::ops::Deref for crate::GlobalNamespace::SaberActivityCounter {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberActivityCounter")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberActivityCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberActivityCounter")]
impl crate::GlobalNamespace::SaberActivityCounter {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDidPauseEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidPauseEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSaberPositionsWereUpdated(
        &mut self,
        leftSaber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
        rightSaber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSaberPositionsWereUpdated", (leftSaber, rightSaber))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn add_totalDistanceDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_totalDistanceDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_handMovementAveragingValueRecorder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AveragingValueRecorder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AveragingValueRecorder,
        > = __cordl_object.invoke("get_handMovementAveragingValueRecorder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftHandMovementDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_leftHandMovementDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftSaberMovementDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_leftSaberMovementDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightHandMovementDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_rightHandMovementDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightSaberMovementDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_rightSaberMovementDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_saberMovementAveragingValueRecorder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AveragingValueRecorder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AveragingValueRecorder,
        > = __cordl_object.invoke("get_saberMovementAveragingValueRecorder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_totalDistanceDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_totalDistanceDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SaberActivityCounter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SaberActivityCounter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
