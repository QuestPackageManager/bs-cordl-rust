#[cfg(feature = "ResetPitchOnGameplayFinished")]
#[repr(C)]
#[derive(Debug)]
pub struct ResetPitchOnGameplayFinished {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameplayLevelSceneTransitionEvents: *mut crate::GlobalNamespace::GameplayLevelSceneTransitionEvents,
    pub _audioManager: *mut crate::GlobalNamespace::AudioManagerSO,
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ResetPitchOnGameplayFinished =>
    ""."ResetPitchOnGameplayFinished"
);
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl std::ops::Deref for crate::GlobalNamespace::ResetPitchOnGameplayFinished {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl std::ops::DerefMut for crate::GlobalNamespace::ResetPitchOnGameplayFinished {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl crate::GlobalNamespace::ResetPitchOnGameplayFinished {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn New(
        gameplayLevelSceneTransitionEvents: *mut crate::GlobalNamespace::GameplayLevelSceneTransitionEvents,
        audioManager: *mut crate::GlobalNamespace::AudioManagerSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (gameplayLevelSceneTransitionEvents, audioManager))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        gameplayLevelSceneTransitionEvents: *mut crate::GlobalNamespace::GameplayLevelSceneTransitionEvents,
        audioManager: *mut crate::GlobalNamespace::AudioManagerSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (gameplayLevelSceneTransitionEvents, audioManager))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ResetPitchOnGameplayFinished {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
