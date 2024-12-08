#[cfg(feature = "SpawnRotationChevronManager+DirectionData")]
#[repr(C)]
#[derive(Debug)]
pub struct SpawnRotationChevronManager_DirectionData {
    __cordl_parent: crate::System::Object,
    pub chevron: *mut crate::GlobalNamespace::SpawnRotationChevron,
    pub fullyLid: bool,
    pub fadeOutStartTime: f32,
    pub fadeInEndTime: f32,
}
#[cfg(feature = "SpawnRotationChevronManager+DirectionData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SpawnRotationChevronManager_DirectionData => ""
    ."SpawnRotationChevronManager/DirectionData"
);
#[cfg(feature = "SpawnRotationChevronManager+DirectionData")]
impl std::ops::Deref
for crate::GlobalNamespace::SpawnRotationChevronManager_DirectionData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationChevronManager+DirectionData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SpawnRotationChevronManager_DirectionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationChevronManager+DirectionData")]
impl crate::GlobalNamespace::SpawnRotationChevronManager_DirectionData {
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
#[cfg(feature = "SpawnRotationChevronManager+DirectionData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SpawnRotationChevronManager_DirectionData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SpawnRotationChevronManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SpawnRotationChevronManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _fadeInTime: f32,
    pub _fadeOutTime: f32,
    pub _jumpStartOffsetTime: f32,
    pub _cutOffsetTime: f32,
    pub _fadeInLightAmountCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _fadeOutLightAmountCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _chevronPool: *mut crate::GlobalNamespace::SpawnRotationChevron_Pool,
    pub _audioTimeSyncController: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _beatmapObjectSpawnController: *mut crate::GlobalNamespace::BeatmapObjectSpawnController,
    pub _beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    pub _spawnRotationDataCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    pub _beatmapObjectDataCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    pub _directionToDataDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::GlobalNamespace::SpawnRotationChevronManager_DirectionData,
    >,
    pub _activeDirections: *mut crate::System::Collections::Generic::HashSet_1<i32>,
    pub _reusableDirectionsList: *mut crate::System::Collections::Generic::List_1<i32>,
    pub _moveDuration: f32,
    pub _halfJumpDuration: f32,
    pub _currentSpawnRotation: f32,
}
#[cfg(feature = "SpawnRotationChevronManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SpawnRotationChevronManager =>
    ""."SpawnRotationChevronManager"
);
#[cfg(feature = "SpawnRotationChevronManager")]
impl std::ops::Deref for crate::GlobalNamespace::SpawnRotationChevronManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationChevronManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::SpawnRotationChevronManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationChevronManager")]
impl crate::GlobalNamespace::SpawnRotationChevronManager {
    #[cfg(feature = "SpawnRotationChevronManager+DirectionData")]
    pub type DirectionData = crate::GlobalNamespace::SpawnRotationChevronManager_DirectionData;
    pub fn ComputeAheadTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ComputeAheadTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapObjectCallback(
        &mut self,
        beatmapObjectData: *mut crate::GlobalNamespace::BeatmapObjectData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapObjectCallback", (beatmapObjectData))?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapObjectSpawnControllerDidInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapObjectSpawnControllerDidInit", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSpawnRotationBeatmapEvent(
        &mut self,
        beatmapEventData: *mut crate::GlobalNamespace::SpawnRotationBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpawnRotationBeatmapEvent", (beatmapEventData))?;
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
#[cfg(feature = "SpawnRotationChevronManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SpawnRotationChevronManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
