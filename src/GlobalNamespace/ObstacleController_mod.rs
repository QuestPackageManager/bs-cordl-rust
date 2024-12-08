#[cfg(feature = "ObstacleController")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleController {
    __cordl_parent: ObstacleControllerBase,
    pub _stretchableObstacle: *mut StretchableObstacle,
    pub _endDistanceOffset: f32,
    pub _visualWrappers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::GameObject,
    >,
    pub _playerTransforms: *mut PlayerTransforms,
    pub _audioTimeSyncController: *mut IAudioTimeSource,
    pub _colorManager: *mut ColorManager,
    pub finishedMovementEvent: *mut crate::System::Action_1<*mut ObstacleController>,
    pub passedThreeQuartersOfMove2Event: *mut crate::System::Action_1<
        *mut ObstacleController,
    >,
    pub passedAvoidedMarkEvent: *mut crate::System::Action_1<*mut ObstacleController>,
    pub didDissolveEvent: *mut crate::System::Action_1<*mut ObstacleController>,
    pub didUpdateProgress: *mut crate::System::Action_2<*mut ObstacleController, f32>,
    pub _width: f32,
    pub _height: f32,
    pub _length: f32,
    pub _startPos: crate::UnityEngine::Vector3,
    pub _midPos: crate::UnityEngine::Vector3,
    pub _endPos: crate::UnityEngine::Vector3,
    pub _move1Duration: f32,
    pub _move2Duration: f32,
    pub _startTimeOffset: f32,
    pub _obstacleDuration: f32,
    pub _passedThreeQuartersOfMove2Reported: bool,
    pub _passedAvoidedMarkReported: bool,
    pub _passedAvoidedMarkTime: f32,
    pub _finishMovementTime: f32,
    pub _bounds: crate::UnityEngine::Bounds,
    pub _dissolving: bool,
    pub _obstacleData: *mut ObstacleData,
    pub _color: crate::UnityEngine::Color,
    pub _worldRotation: crate::UnityEngine::Quaternion,
    pub _inverseWorldRotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "ObstacleController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ObstacleController => ""."ObstacleController"
);
#[cfg(feature = "ObstacleController")]
impl std::ops::Deref for ObstacleController {
    type Target = ObstacleControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleController")]
impl std::ops::DerefMut for ObstacleController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleController")]
impl ObstacleController {
    pub const kAvoidMarkTimeOffset: f32 = 0.15f32;
    #[cfg(feature = "ObstacleController+Pool")]
    pub type Pool = crate::GlobalNamespace::ObstacleController_Pool;
    #[cfg(feature = "ObstacleController+_DissolveCoroutine_d__65")]
    pub type _DissolveCoroutine_d__65 = crate::GlobalNamespace::ObstacleController__DissolveCoroutine_d__65;
    pub fn get_length(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_length", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_finishedMovementEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_finishedMovementEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn DissolveCoroutine(
        &mut self,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("DissolveCoroutine", (duration))?;
        Ok(__cordl_ret)
    }
    pub fn remove_passedAvoidedMarkEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_passedAvoidedMarkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didUpdateProgress(
        &mut self,
        value: *mut crate::System::Action_2<*mut ObstacleController, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didUpdateProgress", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Dissolve(
        &mut self,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dissolve", (duration))?;
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
    pub fn get_hasPassedAvoidedMark(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasPassedAvoidedMark", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_finishedMovementEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_finishedMovementEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        obstacleData: *mut ObstacleData,
        worldRotation: f32,
        startPos: crate::UnityEngine::Vector3,
        midPos: crate::UnityEngine::Vector3,
        endPos: crate::UnityEngine::Vector3,
        move1Duration: f32,
        move2Duration: f32,
        singleLineWidth: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    obstacleData,
                    worldRotation,
                    startPos,
                    midPos,
                    endPos,
                    move1Duration,
                    move2Duration,
                    singleLineWidth,
                    height,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_didDissolveEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didDissolveEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didUpdateProgress(
        &mut self,
        value: *mut crate::System::Action_2<*mut ObstacleController, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didUpdateProgress", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didDissolveEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didDissolveEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_height(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_height", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_passedThreeQuartersOfMove2Event(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_passedThreeQuartersOfMove2Event", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("get_bounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn GetPosForTime(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetPosForTime", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn Hide(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", (hide))?;
        Ok(__cordl_ret)
    }
    pub fn get_obstacleData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ObstacleData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ObstacleData = __cordl_object
            .invoke("get_obstacleData", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_passedThreeQuartersOfMove2Event(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_passedThreeQuartersOfMove2Event", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_passedAvoidedMarkEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ObstacleController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_passedAvoidedMarkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_width(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_width", ())?;
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
    pub fn get_move2Duration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_move2Duration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_move1Duration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_move1Duration", ())?;
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
#[cfg(feature = "ObstacleController")]
impl quest_hook::libil2cpp::ObjectType for ObstacleController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ObstacleController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut ObstacleController>,
}
#[cfg(feature = "ObstacleController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ObstacleController_Pool => ""
    ."ObstacleController/Pool"
);
#[cfg(feature = "ObstacleController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::ObstacleController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut ObstacleController>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObstacleController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleController+Pool")]
impl crate::GlobalNamespace::ObstacleController_Pool {
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
#[cfg(feature = "ObstacleController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObstacleController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
