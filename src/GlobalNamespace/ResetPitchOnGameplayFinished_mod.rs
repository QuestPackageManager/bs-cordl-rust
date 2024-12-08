#[cfg(feature = "ResetPitchOnGameplayFinished")]
#[repr(C)]
#[derive(Debug)]
pub struct ResetPitchOnGameplayFinished {
    __cordl_parent: crate::System::Object,
    pub _gameplayLevelSceneTransitionEvents: *mut GameplayLevelSceneTransitionEvents,
    pub _audioManager: *mut AudioManagerSO,
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ResetPitchOnGameplayFinished => ""
    ."ResetPitchOnGameplayFinished"
);
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl std::ops::Deref for ResetPitchOnGameplayFinished {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl std::ops::DerefMut for ResetPitchOnGameplayFinished {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl ResetPitchOnGameplayFinished {
    pub fn _ctor(
        &mut self,
        gameplayLevelSceneTransitionEvents: *mut GameplayLevelSceneTransitionEvents,
        audioManager: *mut AudioManagerSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (gameplayLevelSceneTransitionEvents, audioManager))?;
        Ok(__cordl_ret)
    }
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
        gameplayLevelSceneTransitionEvents: *mut GameplayLevelSceneTransitionEvents,
        audioManager: *mut AudioManagerSO,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (gameplayLevelSceneTransitionEvents, audioManager))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ResetPitchOnGameplayFinished")]
impl quest_hook::libil2cpp::ObjectType for ResetPitchOnGameplayFinished {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
