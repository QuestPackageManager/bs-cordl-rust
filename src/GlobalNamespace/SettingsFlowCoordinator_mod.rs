#[cfg(feature = "SettingsFlowCoordinator+FinishAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsFlowCoordinator_FinishAction {
    Apply = 2i32,
    Cancel = 0i32,
    _cordl_Ok = 1i32,
}
#[cfg(feature = "SettingsFlowCoordinator+FinishAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SettingsFlowCoordinator_FinishAction => ""
    ."SettingsFlowCoordinator/FinishAction"
);
#[cfg(feature = "SettingsFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _mainSettingsMenuViewController: *mut MainSettingsMenuViewController,
    pub _settingsNavigationController: *mut SettingsNavigationController,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _flushingService: *mut crate::BGLib::SaveDataCore::SaveDataFlushingService,
    pub _settingsApplicator: *mut SettingsApplicatorSO,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut SettingsFlowCoordinator,
        crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction,
    >,
}
#[cfg(feature = "SettingsFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SettingsFlowCoordinator => ""."SettingsFlowCoordinator"
);
#[cfg(feature = "SettingsFlowCoordinator")]
impl std::ops::Deref for SettingsFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsFlowCoordinator")]
impl std::ops::DerefMut for SettingsFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsFlowCoordinator")]
impl SettingsFlowCoordinator {
    #[cfg(feature = "SettingsFlowCoordinator+_CancelSettingsAsync_d__18")]
    pub type _CancelSettingsAsync_d__18 = crate::GlobalNamespace::SettingsFlowCoordinator__CancelSettingsAsync_d__18;
    #[cfg(feature = "SettingsFlowCoordinator+_ApplySettingsAsync_d__17")]
    pub type _ApplySettingsAsync_d__17 = crate::GlobalNamespace::SettingsFlowCoordinator__ApplySettingsAsync_d__17;
    #[cfg(feature = "SettingsFlowCoordinator+FinishAction")]
    pub type FinishAction = crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction;
    #[cfg(
        feature = "SettingsFlowCoordinator+_HandleSettingsNavigationControllerDidFinishAsync_d__16"
    )]
    pub type _HandleSettingsNavigationControllerDidFinishAsync_d__16 = crate::GlobalNamespace::SettingsFlowCoordinator__HandleSettingsNavigationControllerDidFinishAsync_d__16;
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
    pub fn ApplySettingsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("ApplySettingsAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplaceViewController", (viewController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleDidSelectSettingsSubMenu(
        &mut self,
        settingsSubMenuInfo: *mut SettingsSubMenuInfo,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidSelectSettingsSubMenu", (settingsSubMenuInfo, idx))?;
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
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut SettingsFlowCoordinator,
            crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut SettingsFlowCoordinator,
            crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CancelSettingsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("CancelSettingsAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShowSecretViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowSecretViewController", (viewController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSettingsNavigationControllerDidFinishAsync(
        &mut self,
        finishAction: crate::GlobalNamespace::SettingsNavigationController_FinishAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSettingsNavigationControllerDidFinishAsync", (finishAction))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SettingsFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType for SettingsFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
