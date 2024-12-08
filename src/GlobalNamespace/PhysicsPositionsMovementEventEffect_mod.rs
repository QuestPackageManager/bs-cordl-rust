#[cfg(feature = "PhysicsPositionsMovementEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct PhysicsPositionsMovementEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _event: BasicBeatmapEventType,
    pub _movementVector: crate::UnityEngine::Vector3,
    pub _stepSize: f32,
    pub _elasticity: f32,
    pub _friction: f32,
    pub _minMaxSpeed: f32,
    pub _maxMaxSpeed: f32,
    pub _maxAcceleration: f32,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _songTimeFixedUpdateController: *mut SongTimeFixedUpdateController,
    pub _transform: *mut crate::UnityEngine::Transform,
    pub _startPos: crate::UnityEngine::Vector3,
    pub _velocity: crate::UnityEngine::Vector3,
    pub _prevPosition: crate::UnityEngine::Vector3,
    pub _position: crate::UnityEngine::Vector3,
    pub _targetPosition: crate::UnityEngine::Vector3,
    pub _maxSpeed: f32,
    pub _sqrMaxSpeed: f32,
    pub _beatmapDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
}
#[cfg(feature = "PhysicsPositionsMovementEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PhysicsPositionsMovementEventEffect => ""
    ."PhysicsPositionsMovementEventEffect"
);
#[cfg(feature = "PhysicsPositionsMovementEventEffect")]
impl std::ops::Deref for PhysicsPositionsMovementEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PhysicsPositionsMovementEventEffect")]
impl std::ops::DerefMut for PhysicsPositionsMovementEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PhysicsPositionsMovementEventEffect")]
impl PhysicsPositionsMovementEventEffect {
    pub fn HandleSongTimeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSongTimeUpdate", ())?;
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
    pub fn HandleBeatmapEvent(
        &mut self,
        basicBeatmapEventData: *mut BasicBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEvent", (basicBeatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSongTimeFixedUpdate(
        &mut self,
        fixedDeltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSongTimeFixedUpdate", (fixedDeltaTime))?;
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
#[cfg(feature = "PhysicsPositionsMovementEventEffect")]
impl quest_hook::libil2cpp::ObjectType for PhysicsPositionsMovementEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
