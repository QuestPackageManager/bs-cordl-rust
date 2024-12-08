#[cfg(feature = "TutorialScenesTransitionSetupDataSO+TutorialEndStateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TutorialScenesTransitionSetupDataSO_TutorialEndStateType {
    Completed = 0i32,
    Restart = 2i32,
    ReturnToMenu = 1i32,
}
#[cfg(feature = "TutorialScenesTransitionSetupDataSO+TutorialEndStateType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TutorialScenesTransitionSetupDataSO_TutorialEndStateType => ""
    ."TutorialScenesTransitionSetupDataSO/TutorialEndStateType"
);
#[cfg(feature = "TutorialScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialScenesTransitionSetupDataSO {
    __cordl_parent: ScenesTransitionSetupDataSO,
    pub _environmentInfo: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut EnvironmentInfoSO,
    >,
    pub _tutorialSceneInfo: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut SceneInfo,
    >,
    pub _gameCoreSceneInfo: *mut SceneInfo,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut TutorialScenesTransitionSetupDataSO,
        crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO_TutorialEndStateType,
    >,
    pub _playerSpecificSettings_k__BackingField: *mut PlayerSpecificSettings,
    pub _loadedEnvironmentInfo: *mut EnvironmentInfoSO,
    pub _loadedTutorialSceneInfo: *mut SceneInfo,
}
#[cfg(feature = "TutorialScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TutorialScenesTransitionSetupDataSO => ""
    ."TutorialScenesTransitionSetupDataSO"
);
#[cfg(feature = "TutorialScenesTransitionSetupDataSO")]
impl std::ops::Deref for TutorialScenesTransitionSetupDataSO {
    type Target = ScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for TutorialScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialScenesTransitionSetupDataSO")]
impl TutorialScenesTransitionSetupDataSO {
    #[cfg(feature = "TutorialScenesTransitionSetupDataSO+TutorialEndStateType")]
    pub type TutorialEndStateType = crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO_TutorialEndStateType;
    pub fn Finish(
        &mut self,
        endState: crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO_TutorialEndStateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (endState))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        playerSpecificSettings: *mut PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (playerSpecificSettings))?;
        Ok(__cordl_ret)
    }
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut TutorialScenesTransitionSetupDataSO,
            crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO_TutorialEndStateType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_playerSpecificSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerSpecificSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerSpecificSettings = __cordl_object
            .invoke("get_playerSpecificSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut TutorialScenesTransitionSetupDataSO,
            crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO_TutorialEndStateType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerSpecificSettings(
        &mut self,
        value: *mut PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerSpecificSettings", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TutorialScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType for TutorialScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
