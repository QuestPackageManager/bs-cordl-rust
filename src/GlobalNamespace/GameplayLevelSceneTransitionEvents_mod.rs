#[cfg(feature = "GameplayLevelSceneTransitionEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayLevelSceneTransitionEvents {
    __cordl_parent: crate::System::Object,
    pub _standardLevelScenesTransitionSetupData: *mut StandardLevelScenesTransitionSetupDataSO,
    pub _missionLevelScenesTransitionSetupData: *mut MissionLevelScenesTransitionSetupDataSO,
    pub _multiplayerLevelScenesTransitionSetupData: *mut MultiplayerLevelScenesTransitionSetupDataSO,
    pub anyGameplayLevelDidFinishEvent: *mut crate::System::Action,
}
#[cfg(feature = "GameplayLevelSceneTransitionEvents")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameplayLevelSceneTransitionEvents => ""
    ."GameplayLevelSceneTransitionEvents"
);
#[cfg(feature = "GameplayLevelSceneTransitionEvents")]
impl std::ops::Deref for GameplayLevelSceneTransitionEvents {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayLevelSceneTransitionEvents")]
impl std::ops::DerefMut for GameplayLevelSceneTransitionEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayLevelSceneTransitionEvents")]
impl GameplayLevelSceneTransitionEvents {
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
    pub fn HandleMissionLevelDidFinish(
        &mut self,
        missionLevelScenesTransitionSetupData: *mut MissionLevelScenesTransitionSetupDataSO,
        missionCompletionResults: *mut MissionCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMissionLevelDidFinish",
                (missionLevelScenesTransitionSetupData, missionCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelDidDisconnect(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: *mut MultiplayerLevelScenesTransitionSetupDataSO,
        disconnectedReason: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelDidDisconnect",
                (multiplayerLevelScenesTransitionSetupData, disconnectedReason),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerLevelDidFinish(
        &mut self,
        multiplayerLevelScenesTransitionSetupData: *mut MultiplayerLevelScenesTransitionSetupDataSO,
        multiplayerResultsData: *mut MultiplayerResultsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerLevelDidFinish",
                (multiplayerLevelScenesTransitionSetupData, multiplayerResultsData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleStandardLevelDidFinish(
        &mut self,
        standardLevelScenesTransitionSetupData: *mut StandardLevelScenesTransitionSetupDataSO,
        levelCompletionResults: *mut LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleStandardLevelDidFinish",
                (standardLevelScenesTransitionSetupData, levelCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InvokeAnyGameplayLevelDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeAnyGameplayLevelDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        standardLevelScenesTransitionSetupData: *mut StandardLevelScenesTransitionSetupDataSO,
        missionLevelScenesTransitionSetupData: *mut MissionLevelScenesTransitionSetupDataSO,
        multiplayerLevelScenesTransitionSetupData: *mut MultiplayerLevelScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    standardLevelScenesTransitionSetupData,
                    missionLevelScenesTransitionSetupData,
                    multiplayerLevelScenesTransitionSetupData,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        standardLevelScenesTransitionSetupData: *mut StandardLevelScenesTransitionSetupDataSO,
        missionLevelScenesTransitionSetupData: *mut MissionLevelScenesTransitionSetupDataSO,
        multiplayerLevelScenesTransitionSetupData: *mut MultiplayerLevelScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    standardLevelScenesTransitionSetupData,
                    missionLevelScenesTransitionSetupData,
                    multiplayerLevelScenesTransitionSetupData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_anyGameplayLevelDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_anyGameplayLevelDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_anyGameplayLevelDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_anyGameplayLevelDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameplayLevelSceneTransitionEvents")]
impl quest_hook::libil2cpp::ObjectType for GameplayLevelSceneTransitionEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
