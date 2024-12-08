#[cfg(feature = "RecordingToolSettingsFlowCoordinator+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolSettingsFlowCoordinator_InitData {
    __cordl_parent: crate::System::Object,
    pub nextScenesTransitionSetupData: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
}
#[cfg(feature = "RecordingToolSettingsFlowCoordinator+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RecordingToolSettingsFlowCoordinator_InitData => ""
    ."RecordingToolSettingsFlowCoordinator/InitData"
);
#[cfg(feature = "RecordingToolSettingsFlowCoordinator+InitData")]
impl std::ops::Deref
for crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolSettingsFlowCoordinator+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolSettingsFlowCoordinator+InitData")]
impl crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator_InitData {
    pub fn New(
        nextScenesTransitionSetupData: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        nextScenesTransitionSetupData: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RecordingToolSettingsFlowCoordinator+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RecordingToolSettingsFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolSettingsFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _recordingToolConfigViewController: *mut crate::GlobalNamespace::RecordingToolConfigViewController,
    pub _recordingToolSettingsViewController: *mut crate::GlobalNamespace::RecordingToolSettingsViewController,
    pub _recordingToolLoggingViewController: *mut crate::GlobalNamespace::RecordingToolLoggingViewController,
    pub _gameScenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _initData: *mut crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator_InitData,
}
#[cfg(feature = "RecordingToolSettingsFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RecordingToolSettingsFlowCoordinator => ""
    ."RecordingToolSettingsFlowCoordinator"
);
#[cfg(feature = "RecordingToolSettingsFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolSettingsFlowCoordinator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolSettingsFlowCoordinator")]
impl crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator {
    #[cfg(feature = "RecordingToolSettingsFlowCoordinator+InitData")]
    pub type InitData = crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator_InitData;
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret)
    }
    pub fn GoToNextScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GoToNextScene", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleRecordingToolSettingsViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRecordingToolSettingsViewControllerDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "RecordingToolSettingsFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingToolSettingsFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
