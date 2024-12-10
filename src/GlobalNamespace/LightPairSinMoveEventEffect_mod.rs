#[cfg(feature = "LightPairSinMoveEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct LightPairSinMoveEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _eventL: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _eventR: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _switchOverrideRandomValuesEvent: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _overrideRandomValues: bool,
    pub _startValueOffset: f32,
    pub _startPositionOffset: crate::UnityEngine::Vector3,
    pub _endPositionOffset: crate::UnityEngine::Vector3,
    pub _transformL: *mut crate::UnityEngine::Transform,
    pub _transformR: *mut crate::UnityEngine::Transform,
    pub _beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    pub _audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _movementDataL: *mut crate::GlobalNamespace::LightPairSinMoveEventEffect_MovementData,
    pub _movementDataR: *mut crate::GlobalNamespace::LightPairSinMoveEventEffect_MovementData,
    pub _randomGenerationFrameNum: i32,
    pub _randomStartOffset: f32,
    pub _beatmapDataCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
}
#[cfg(feature = "LightPairSinMoveEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightPairSinMoveEventEffect =>
    ""."LightPairSinMoveEventEffect"
);
#[cfg(feature = "LightPairSinMoveEventEffect")]
impl std::ops::Deref for crate::GlobalNamespace::LightPairSinMoveEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairSinMoveEventEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightPairSinMoveEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairSinMoveEventEffect")]
impl crate::GlobalNamespace::LightPairSinMoveEventEffect {
    pub const kSpeedMultiplier: f32 = 1f32;
    #[cfg(feature = "LightPairSinMoveEventEffect+MovementData")]
    pub type MovementData = crate::GlobalNamespace::LightPairSinMoveEventEffect_MovementData;
    pub fn HandleBeatmapEvent(
        &mut self,
        basicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEvent", (basicBeatmapEventData))?;
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMovementData(
        &mut self,
        beatmapEventDataValue: i32,
        movementData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightPairSinMoveEventEffect_MovementData,
        >,
        movementValueOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateMovementData",
                (beatmapEventDataValue, movementData, movementValueOffset),
            )?;
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
}
#[cfg(feature = "LightPairSinMoveEventEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightPairSinMoveEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightPairSinMoveEventEffect+MovementData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightPairSinMoveEventEffect_MovementData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub enabled: bool,
    pub speed: f32,
    pub startPosition: crate::UnityEngine::Vector3,
    pub transform: *mut crate::UnityEngine::Transform,
    pub startMovementValue: f32,
    pub movementValue: f32,
    pub side: f32,
}
#[cfg(feature = "LightPairSinMoveEventEffect+MovementData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightPairSinMoveEventEffect_MovementData => ""
    ."LightPairSinMoveEventEffect/MovementData"
);
#[cfg(feature = "LightPairSinMoveEventEffect+MovementData")]
impl std::ops::Deref
for crate::GlobalNamespace::LightPairSinMoveEventEffect_MovementData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairSinMoveEventEffect+MovementData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightPairSinMoveEventEffect_MovementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairSinMoveEventEffect+MovementData")]
impl crate::GlobalNamespace::LightPairSinMoveEventEffect_MovementData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "LightPairSinMoveEventEffect+MovementData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightPairSinMoveEventEffect_MovementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
