#[cfg(feature = "TrackLaneRingsPositionStepEffectSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackLaneRingsPositionStepEffectSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _trackLaneRingsManager: *mut TrackLaneRingsManager,
    pub _beatmapEventType: BasicBeatmapEventType,
    pub _minPositionStep: f32,
    pub _maxPositionStep: f32,
    pub _moveSpeed: f32,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _beatmapDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
}
#[cfg(feature = "TrackLaneRingsPositionStepEffectSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TrackLaneRingsPositionStepEffectSpawner => ""
    ."TrackLaneRingsPositionStepEffectSpawner"
);
#[cfg(feature = "TrackLaneRingsPositionStepEffectSpawner")]
impl std::ops::Deref for TrackLaneRingsPositionStepEffectSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRingsPositionStepEffectSpawner")]
impl std::ops::DerefMut for TrackLaneRingsPositionStepEffectSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRingsPositionStepEffectSpawner")]
impl TrackLaneRingsPositionStepEffectSpawner {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TrackLaneRingsPositionStepEffectSpawner")]
impl quest_hook::libil2cpp::ObjectType for TrackLaneRingsPositionStepEffectSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
