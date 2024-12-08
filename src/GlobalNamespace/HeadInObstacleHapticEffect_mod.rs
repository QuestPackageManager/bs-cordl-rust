#[cfg(feature = "HeadInObstacleHapticEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct HeadInObstacleHapticEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _headHapticPreset: *mut crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    pub _minimalHapticDuration: f32,
    pub _hapticFeedbackManager: *mut crate::GlobalNamespace::HapticFeedbackManager,
    pub _playerHeadAndObstacleInteraction: *mut crate::GlobalNamespace::PlayerHeadAndObstacleInteraction,
    pub _gamePause: *mut crate::GlobalNamespace::IGamePause,
    pub _gameplayLevelSceneTransitionEvents: *mut crate::GlobalNamespace::GameplayLevelSceneTransitionEvents,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _isGamePaused: bool,
    pub _isLevelFinished: bool,
    pub _minimumTimeUntilHapticEnd: f32,
    pub _wasHeadInWallLastFrame: bool,
}
#[cfg(feature = "HeadInObstacleHapticEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HeadInObstacleHapticEffect =>
    ""."HeadInObstacleHapticEffect"
);
#[cfg(feature = "HeadInObstacleHapticEffect")]
impl std::ops::Deref for crate::GlobalNamespace::HeadInObstacleHapticEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HeadInObstacleHapticEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::HeadInObstacleHapticEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HeadInObstacleHapticEffect")]
impl crate::GlobalNamespace::HeadInObstacleHapticEffect {
    pub const kInvalidTime: f32 = -1f32;
    pub fn HandleAnyGameplayLevelDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAnyGameplayLevelDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleDidPauseEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidPauseEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleDidResumeEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidResumeEvent", ())?;
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
#[cfg(feature = "HeadInObstacleHapticEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HeadInObstacleHapticEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
