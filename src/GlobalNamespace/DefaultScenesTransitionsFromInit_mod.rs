#[cfg(feature = "DefaultScenesTransitionsFromInit")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultScenesTransitionsFromInit {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _healthWarningScenesTransitionSetupData: *mut HealthWarningScenesTransitionSetupDataSO,
    pub _recordingToolScenesTransitionSetupData: *mut RecordingToolScenesTransitionSetupDataSO,
    pub _mainMenuScenesTransitionSetupData: *mut MenuScenesTransitionSetupDataSO,
    pub _beatmapEditorScenesTransitionSetupData: *mut BeatmapEditorScenesTransitionSetupDataSO,
    pub _shaderWarmupScenesTransitionSetupData: *mut ShaderWarmupScenesTransitionSetupDataSO,
    pub _startupErrorScenesTransitionSetupData: *mut StartupErrorScenesTransitionSetupDataSO,
    pub _gameScenesManager: *mut GameScenesManager,
}
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for DefaultScenesTransitionsFromInit => ""
    ."DefaultScenesTransitionsFromInit"
);
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl std::ops::Deref for DefaultScenesTransitionsFromInit {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl std::ops::DerefMut for DefaultScenesTransitionsFromInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl DefaultScenesTransitionsFromInit {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn TransitionToNextScene(
        &mut self,
        goStraightToMenu: bool,
        goStraightToEditor: bool,
        goToRecordingToolScene: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TransitionToNextScene",
                (goStraightToMenu, goStraightToEditor, goToRecordingToolScene),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TransitionToStartupErrorScene(
        &mut self,
        title: *mut crate::System::String,
        subtitle: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionToStartupErrorScene", (title, subtitle))?;
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
    pub fn get_mainMenuScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MenuScenesTransitionSetupDataSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MenuScenesTransitionSetupDataSO = __cordl_object
            .invoke("get_mainMenuScenesTransitionSetupData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "DefaultScenesTransitionsFromInit")]
impl quest_hook::libil2cpp::ObjectType for DefaultScenesTransitionsFromInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
