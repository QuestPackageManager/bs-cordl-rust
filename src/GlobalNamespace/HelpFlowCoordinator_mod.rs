#[cfg(feature = "HelpFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct HelpFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _helpMenuViewController: *mut HelpMenuViewController,
    pub _helpNavigationController: *mut HelpNavigationController,
    pub _howToPlayViewController: *mut HowToPlayViewController,
    pub _healthWarningDisplayViewController: *mut HealthWarningDisplayViewController,
    pub _privacyPolicyDisplayViewController: *mut PrivacyPolicyDisplayViewController,
    pub _eulaDisplayViewController: *mut EulaDisplayViewController,
    pub _licensesDisplayViewController: *mut LicensesDisplayViewController,
    pub _playerStatisticsViewController: *mut PlayerStatisticsViewController,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _menuTransitionsHelper: *mut MenuTransitionsHelper,
    pub didFinishEvent: *mut crate::System::Action_1<*mut HelpFlowCoordinator>,
    pub _viewControllers: *mut crate::System::Collections::Generic::List_1<
        crate::System::ValueTuple_2<
            *mut crate::HMUI::ViewController,
            *mut crate::System::String,
        >,
    >,
}
#[cfg(feature = "HelpFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for HelpFlowCoordinator => ""."HelpFlowCoordinator"
);
#[cfg(feature = "HelpFlowCoordinator")]
impl std::ops::Deref for HelpFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HelpFlowCoordinator")]
impl std::ops::DerefMut for HelpFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HelpFlowCoordinator")]
impl HelpFlowCoordinator {
    pub const kEulaMenu: &'static str = "EULA_MENU";
    pub const kHealthWarningMenu: &'static str = "HEALTH_AND_SAFETY_MENU";
    pub const kHowToPlayMenu: &'static str = "LABEL_HOW_TO_PLAY";
    pub const kLicensesMenu: &'static str = "SOFTWARE_LICENSES";
    pub const kPrivacyPolicyMenu: &'static str = "PRIVACY_POLICY_MENU";
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut HelpFlowCoordinator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
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
    pub fn HandleHowToPlayViewControllerDidFinish(
        &mut self,
        howToPlayOptions: crate::GlobalNamespace::HowToPlayViewController_HowToPlayOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHowToPlayViewControllerDidFinish", (howToPlayOptions))?;
        Ok(__cordl_ret)
    }
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackButtonWasPressed", (topViewController))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut HelpFlowCoordinator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
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
    pub fn HandleDidSelectHelpSubMenu(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidSelectHelpSubMenu", (idx))?;
        Ok(__cordl_ret)
    }
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HelpFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType for HelpFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
