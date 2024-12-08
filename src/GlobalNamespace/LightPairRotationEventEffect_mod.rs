#[cfg(feature = "LightPairRotationEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct LightPairRotationEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _eventL: BasicBeatmapEventType,
    pub _eventR: BasicBeatmapEventType,
    pub _switchOverrideRandomValuesEvent: BasicBeatmapEventType,
    pub _rotationVector: crate::UnityEngine::Vector3,
    pub _overrideRandomValues: bool,
    pub _useZPositionForAngleOffset: bool,
    pub _zPositionAngleOffsetScale: f32,
    pub _startRotation: f32,
    pub _transformL: *mut crate::UnityEngine::Transform,
    pub _transformR: *mut crate::UnityEngine::Transform,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _audioTimeSource: *mut IAudioTimeSource,
    pub _rotationDataL: *mut crate::GlobalNamespace::LightPairRotationEventEffect_RotationData,
    pub _rotationDataR: *mut crate::GlobalNamespace::LightPairRotationEventEffect_RotationData,
    pub _randomGenerationFrameNum: i32,
    pub _randomStartRotation: f32,
    pub _randomDirection: f32,
    pub _beatmapDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
}
#[cfg(feature = "LightPairRotationEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightPairRotationEventEffect => ""
    ."LightPairRotationEventEffect"
);
#[cfg(feature = "LightPairRotationEventEffect")]
impl std::ops::Deref for LightPairRotationEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairRotationEventEffect")]
impl std::ops::DerefMut for LightPairRotationEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairRotationEventEffect")]
impl LightPairRotationEventEffect {
    pub const kSpeedMultiplier: f32 = 20f32;
    #[cfg(feature = "LightPairRotationEventEffect+RotationData")]
    pub type RotationData = crate::GlobalNamespace::LightPairRotationEventEffect_RotationData;
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
    pub fn UpdateRotationData(
        &mut self,
        beatmapEventDataValue: i32,
        rotationData: *mut crate::GlobalNamespace::LightPairRotationEventEffect_RotationData,
        startRotationOffset: f32,
        direction: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateRotationData",
                (beatmapEventDataValue, rotationData, startRotationOffset, direction),
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
}
#[cfg(feature = "LightPairRotationEventEffect")]
impl quest_hook::libil2cpp::ObjectType for LightPairRotationEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightPairRotationEventEffect+RotationData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightPairRotationEventEffect_RotationData {
    __cordl_parent: crate::System::Object,
    pub enabled: bool,
    pub rotationSpeed: f32,
    pub startRotation: crate::UnityEngine::Quaternion,
    pub transform: *mut crate::UnityEngine::Transform,
    pub startRotationAngle: f32,
    pub rotationAngle: f32,
}
#[cfg(feature = "LightPairRotationEventEffect+RotationData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightPairRotationEventEffect_RotationData => ""
    ."LightPairRotationEventEffect/RotationData"
);
#[cfg(feature = "LightPairRotationEventEffect+RotationData")]
impl std::ops::Deref
for crate::GlobalNamespace::LightPairRotationEventEffect_RotationData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairRotationEventEffect+RotationData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightPairRotationEventEffect_RotationData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairRotationEventEffect+RotationData")]
impl crate::GlobalNamespace::LightPairRotationEventEffect_RotationData {
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
}
#[cfg(feature = "LightPairRotationEventEffect+RotationData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightPairRotationEventEffect_RotationData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
