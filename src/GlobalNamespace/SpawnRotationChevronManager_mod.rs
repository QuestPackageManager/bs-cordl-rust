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
    pub _variableMovementDataProvider: *mut crate::GlobalNamespace::IVariableMovementDataProvider,
    pub _spawnRotationDataCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    pub _beatmapObjectDataCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    pub _directionToDataDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::GlobalNamespace::SpawnRotationChevronManager_DirectionData,
    >,
    pub _activeDirections: *mut crate::System::Collections::Generic::HashSet_1<i32>,
    pub _reusableDirectionsList: *mut crate::System::Collections::Generic::List_1<i32>,
    pub _queuedDirectionData: *mut crate::System::Collections::Generic::Queue_1<
        crate::GlobalNamespace::SpawnRotationChevronManager_QueuedDirectionData,
    >,
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
    #[cfg(feature = "SpawnRotationChevronManager+QueuedDirectionData")]
    pub type QueuedDirectionData = crate::GlobalNamespace::SpawnRotationChevronManager_QueuedDirectionData;
    pub fn HandleBeatmapObjectCallback(
        &mut self,
        beatmapObjectData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapObjectCallback", (beatmapObjectData))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapObjectSpawnControllerDidInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapObjectSpawnControllerDidInit", ())?;
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
    pub fn SpawnDirectionData(
        &mut self,
        _cordl_time: f32,
        duration: f32,
        rotation: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnDirectionData", (_cordl_time, duration, rotation))?;
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
#[cfg(feature = "SpawnRotationChevronManager+DirectionData")]
#[repr(C)]
#[derive(Debug)]
pub struct SpawnRotationChevronManager_DirectionData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub chevron: *mut crate::GlobalNamespace::SpawnRotationChevron,
    pub fullyLit: bool,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
#[cfg(feature = "SpawnRotationChevronManager+QueuedDirectionData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SpawnRotationChevronManager_QueuedDirectionData {
    pub _cordl_time: f32,
    pub duration: f32,
    pub rotation: i32,
}
#[cfg(feature = "SpawnRotationChevronManager+QueuedDirectionData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SpawnRotationChevronManager_QueuedDirectionData => ""
    ."SpawnRotationChevronManager/QueuedDirectionData"
);
#[cfg(feature = "SpawnRotationChevronManager+QueuedDirectionData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SpawnRotationChevronManager_QueuedDirectionData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SpawnRotationChevronManager+QueuedDirectionData")]
impl crate::GlobalNamespace::SpawnRotationChevronManager_QueuedDirectionData {
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        duration: f32,
        rotation: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time, duration, rotation),
        )?;
        Ok(__cordl_ret.into())
    }
}
