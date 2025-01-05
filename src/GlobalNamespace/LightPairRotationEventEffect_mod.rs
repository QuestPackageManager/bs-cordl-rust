#[cfg(feature = "LightPairRotationEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct LightPairRotationEventEffect {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _eventL: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _eventR: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _switchOverrideRandomValuesEvent: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _rotationVector: crate::UnityEngine::Vector3,
    pub _overrideRandomValues: bool,
    pub _useZPositionForAngleOffset: bool,
    pub _zPositionAngleOffsetScale: f32,
    pub _startRotation: f32,
    pub _transformL: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _transformR: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _audioTimeSource: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAudioTimeSource,
    >,
    pub _random: quest_hook::libil2cpp::Gc<crate::System::Random>,
    pub _rotationDataL: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightPairRotationEventEffect_RotationData,
    >,
    pub _rotationDataR: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightPairRotationEventEffect_RotationData,
    >,
    pub _randomGenerationFrameNum: i32,
    pub _randomStartRotation: f32,
    pub _randomDirection: f32,
    pub _beatmapDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
}
#[cfg(feature = "LightPairRotationEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightPairRotationEventEffect =>
    ""."LightPairRotationEventEffect"
);
#[cfg(feature = "LightPairRotationEventEffect")]
impl std::ops::Deref for crate::GlobalNamespace::LightPairRotationEventEffect {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairRotationEventEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightPairRotationEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightPairRotationEventEffect")]
impl crate::GlobalNamespace::LightPairRotationEventEffect {
    pub const kSpeedMultiplier: f32 = 20f32;
    #[cfg(feature = "LightPairRotationEventEffect+RotationData")]
    pub type RotationData = crate::GlobalNamespace::LightPairRotationEventEffect_RotationData;
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
    pub fn UpdateRotationData(
        &mut self,
        beatmapEventDataValue: i32,
        rotationData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightPairRotationEventEffect_RotationData,
        >,
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
#[cfg(feature = "LightPairRotationEventEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightPairRotationEventEffect {
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub enabled: bool,
    pub rotationSpeed: f32,
    pub startRotation: crate::UnityEngine::Quaternion,
    pub transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
