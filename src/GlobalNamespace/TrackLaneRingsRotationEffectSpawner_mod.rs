#[cfg(feature = "TrackLaneRingsRotationEffectSpawner+RotationStepType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackLaneRingsRotationEffectSpawner_RotationStepType {
    MaxOr0 = 2i32,
    Range = 1i32,
    Range0ToMax = 0i32,
}
#[cfg(feature = "TrackLaneRingsRotationEffectSpawner+RotationStepType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TrackLaneRingsRotationEffectSpawner_RotationStepType => ""
    ."TrackLaneRingsRotationEffectSpawner/RotationStepType"
);
#[cfg(feature = "TrackLaneRingsRotationEffectSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackLaneRingsRotationEffectSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _trackLaneRingsRotationEffect: *mut TrackLaneRingsRotationEffect,
    pub _beatmapEventType: BasicBeatmapEventType,
    pub _rotation: f32,
    pub _rotationStep: f32,
    pub _rotationStepType: crate::GlobalNamespace::TrackLaneRingsRotationEffectSpawner_RotationStepType,
    pub _rotationPropagationSpeed: i32,
    pub _rotationFlexySpeed: f32,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _beatmapDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
}
#[cfg(feature = "TrackLaneRingsRotationEffectSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TrackLaneRingsRotationEffectSpawner => ""
    ."TrackLaneRingsRotationEffectSpawner"
);
#[cfg(feature = "TrackLaneRingsRotationEffectSpawner")]
impl std::ops::Deref for TrackLaneRingsRotationEffectSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffectSpawner")]
impl std::ops::DerefMut for TrackLaneRingsRotationEffectSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffectSpawner")]
impl TrackLaneRingsRotationEffectSpawner {
    #[cfg(feature = "TrackLaneRingsRotationEffectSpawner+RotationStepType")]
    pub type RotationStepType = crate::GlobalNamespace::TrackLaneRingsRotationEffectSpawner_RotationStepType;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffectSpawner")]
impl quest_hook::libil2cpp::ObjectType for TrackLaneRingsRotationEffectSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
