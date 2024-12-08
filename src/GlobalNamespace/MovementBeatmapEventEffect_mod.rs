#[cfg(feature = "MovementBeatmapEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct MovementBeatmapEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _beatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _transitionSpeed: f32,
    pub _movementData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MovementBeatmapEventEffect_MovementData,
    >,
    pub _transforms: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Transform,
    >,
    pub _beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    pub _currentMovementDataIdx: i32,
    pub _currentPositionOffset: crate::UnityEngine::Vector3,
    pub _prevPositionOffset: crate::UnityEngine::Vector3,
    pub _startLocalPositions: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
    pub _beatmapDataCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
}
#[cfg(feature = "MovementBeatmapEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MovementBeatmapEventEffect =>
    ""."MovementBeatmapEventEffect"
);
#[cfg(feature = "MovementBeatmapEventEffect")]
impl std::ops::Deref for crate::GlobalNamespace::MovementBeatmapEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MovementBeatmapEventEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::MovementBeatmapEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MovementBeatmapEventEffect")]
impl crate::GlobalNamespace::MovementBeatmapEventEffect {
    #[cfg(feature = "MovementBeatmapEventEffect+MovementData")]
    pub type MovementData = crate::GlobalNamespace::MovementBeatmapEventEffect_MovementData;
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapEvent(
        &mut self,
        basicBeatmapEventData: *mut crate::GlobalNamespace::BasicBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEvent", (basicBeatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
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
    pub fn SetLocalPositionOffsetsForAllObjects(
        &mut self,
        localPositionOffset: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPositionOffsetsForAllObjects", (localPositionOffset))?;
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
}
#[cfg(feature = "MovementBeatmapEventEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MovementBeatmapEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MovementBeatmapEventEffect+MovementData")]
#[repr(C)]
#[derive(Debug)]
pub struct MovementBeatmapEventEffect_MovementData {
    __cordl_parent: crate::System::Object,
    pub _localPositionOffset: crate::UnityEngine::Vector3,
}
#[cfg(feature = "MovementBeatmapEventEffect+MovementData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MovementBeatmapEventEffect_MovementData => ""
    ."MovementBeatmapEventEffect/MovementData"
);
#[cfg(feature = "MovementBeatmapEventEffect+MovementData")]
impl std::ops::Deref
for crate::GlobalNamespace::MovementBeatmapEventEffect_MovementData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MovementBeatmapEventEffect+MovementData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MovementBeatmapEventEffect_MovementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MovementBeatmapEventEffect+MovementData")]
impl crate::GlobalNamespace::MovementBeatmapEventEffect_MovementData {
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
    pub fn get_localPositionOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_localPositionOffset", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MovementBeatmapEventEffect+MovementData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MovementBeatmapEventEffect_MovementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
