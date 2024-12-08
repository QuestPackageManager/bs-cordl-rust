#[cfg(feature = "PlayerHeightDetector+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerHeightDetector_InitData {
    __cordl_parent: crate::System::Object,
    pub headPosToPlayerHeightOffset: f32,
    pub startPlayerHeight: f32,
}
#[cfg(feature = "PlayerHeightDetector+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerHeightDetector_InitData
    => ""."PlayerHeightDetector/InitData"
);
#[cfg(feature = "PlayerHeightDetector+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerHeightDetector_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerHeightDetector+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerHeightDetector_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerHeightDetector+InitData")]
impl crate::GlobalNamespace::PlayerHeightDetector_InitData {
    pub fn New(
        headPosToPlayerHeightOffset: f32,
        startPlayerHeight: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (headPosToPlayerHeightOffset, startPlayerHeight))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        headPosToPlayerHeightOffset: f32,
        startPlayerHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (headPosToPlayerHeightOffset, startPlayerHeight))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayerHeightDetector+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerHeightDetector_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerHeightDetector")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerHeightDetector {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _playerTransforms: *mut crate::GlobalNamespace::PlayerTransforms,
    pub _beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    pub _audioTimeSyncController: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _initData: *mut crate::GlobalNamespace::PlayerHeightDetector_InitData,
    pub playerHeightDidChangeEvent: *mut crate::System::Action_1<f32>,
    pub _beatmapObjectCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    pub _noTopObstaclesStartTime: f32,
    pub _computedPlayerHeight: f32,
    pub _changeWeight: f32,
    pub _lastReportedHeight: f32,
}
#[cfg(feature = "PlayerHeightDetector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerHeightDetector => ""
    ."PlayerHeightDetector"
);
#[cfg(feature = "PlayerHeightDetector")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerHeightDetector {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerHeightDetector")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerHeightDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerHeightDetector")]
impl crate::GlobalNamespace::PlayerHeightDetector {
    #[cfg(feature = "PlayerHeightDetector+InitData")]
    pub type InitData = crate::GlobalNamespace::PlayerHeightDetector_InitData;
    pub fn BeatmapObjectSpawnCallback(
        &mut self,
        obstacleData: *mut crate::GlobalNamespace::ObstacleData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeatmapObjectSpawnCallback", (obstacleData))?;
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
    pub fn add_playerHeightDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerHeightDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_playerHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_playerHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerHeightDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerHeightDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayerHeightDetector")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlayerHeightDetector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
