#[cfg(feature = "HydraulicCarJumpEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct HydraulicCarJumpEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _event: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _eventValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub _impulse: crate::UnityEngine::Vector3,
    pub _randomness: f32,
    pub _position: crate::UnityEngine::Vector3,
    pub _minDelayBetweenEvents: f32,
    pub _rigidbody: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rigidbody>,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _lastEventTime: f32,
    pub _eventValuesHashSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub _beatmapDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
}
#[cfg(feature = "HydraulicCarJumpEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HydraulicCarJumpEffect => ""
    ."HydraulicCarJumpEffect"
);
#[cfg(feature = "HydraulicCarJumpEffect")]
impl std::ops::Deref for crate::GlobalNamespace::HydraulicCarJumpEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HydraulicCarJumpEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::HydraulicCarJumpEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HydraulicCarJumpEffect")]
impl crate::GlobalNamespace::HydraulicCarJumpEffect {
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
#[cfg(feature = "HydraulicCarJumpEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HydraulicCarJumpEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
