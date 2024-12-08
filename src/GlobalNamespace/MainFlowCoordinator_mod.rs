#[cfg(feature = "MainFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct MainFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _defaultLightsPreset: *mut MenuLightsPresetSO,
    pub _soloFreePlayFlowCoordinator: *mut SoloFreePlayFlowCoordinator,
    pub _partyFreePlayFlowCoordinator: *mut PartyFreePlayFlowCoordinator,
    pub _campaignFlowCoordinator: *mut CampaignFlowCoordinator,
    pub _settingsFlowCoordinator: *mut SettingsFlowCoordinator,
    pub _multiplayerModeSelectionFlowCoordinator: *mut MultiplayerModeSelectionFlowCoordinator,
    pub _helpFlowCoordinator: *mut HelpFlowCoordinator,
    pub _editAvatarFlowCoordinatorHelper: *mut EditAvatarFlowCoordinatorHelper,
    pub _simpleDialogPromptViewController: *mut SimpleDialogPromptViewController,
    pub _mainMenuViewController: *mut MainMenuViewController,
    pub _playerOptionsViewController: *mut PlayerOptionsViewController,
    pub _optionsViewController: *mut OptionsViewController,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _menuLightsManager: *mut MenuLightsManager,
    pub _fadeInOut: *mut FadeInOutController,
    pub _beatmapLevelsModel: *mut BeatmapLevelsModel,
    pub _menuTransitionsHelper: *mut MenuTransitionsHelper,
    pub _analyticsModel: *mut IAnalyticsModel,
    pub _avatarSystemCollection: *mut crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    pub _menuDestinationRequest: *mut MenuDestination,
    pub _afterDialogPromptFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
    pub _goToMultiplayerAfterAvatarCreation: bool,
}
#[cfg(feature = "MainFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MainFlowCoordinator => ""."MainFlowCoordinator"
);
#[cfg(feature = "MainFlowCoordinator")]
impl std::ops::Deref for MainFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainFlowCoordinator")]
impl std::ops::DerefMut for MainFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainFlowCoordinator")]
impl MainFlowCoordinator {
    #[cfg(
        feature = "MainFlowCoordinator+_HandleMultiplayerDisclaimerDidFinishAction_d__29"
    )]
    pub type _HandleMultiplayerDisclaimerDidFinishAction_d__29 = crate::GlobalNamespace::MainFlowCoordinator__HandleMultiplayerDisclaimerDidFinishAction_d__29;
    #[cfg(
        feature = "MainFlowCoordinator+_ProcessMenuDestinationRequestAfterFrameCoroutine_d__43"
    )]
    pub type _ProcessMenuDestinationRequestAfterFrameCoroutine_d__43 = crate::GlobalNamespace::MainFlowCoordinator__ProcessMenuDestinationRequestAfterFrameCoroutine_d__43;
    #[cfg(
        feature = "MainFlowCoordinator+_PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator_d__42"
    )]
    pub type _PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator_d__42 = crate::GlobalNamespace::MainFlowCoordinator__PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator_d__42;
    #[cfg(
        feature = "MainFlowCoordinator+_HandleEditAvatarFlowCoordinatorHelperDidFinish_d__34"
    )]
    pub type _HandleEditAvatarFlowCoordinatorHelperDidFinish_d__34 = crate::GlobalNamespace::MainFlowCoordinator__HandleEditAvatarFlowCoordinatorHelperDidFinish_d__34;
    #[cfg(feature = "MainFlowCoordinator+_ProcessMenuDestinationRequest_d__41")]
    pub type _ProcessMenuDestinationRequest_d__41 = crate::GlobalNamespace::MainFlowCoordinator__ProcessMenuDestinationRequest_d__41;
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
    pub fn HandleCampaignFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: *mut CampaignFlowCoordinator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCampaignFlowCoordinatorDidFinish", (flowCoordinator))?;
        Ok(__cordl_ret)
    }
    pub fn HandleEditAvatarFlowCoordinatorHelperDidFinish(
        &mut self,
        flowCoordinator: *mut crate::HMUI::FlowCoordinator,
        finishAction: crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleEditAvatarFlowCoordinatorHelperDidFinish",
                (flowCoordinator, finishAction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleHelpFlowCoordinatorDidFinish(
        &mut self,
        helpFlowCoordinator: *mut HelpFlowCoordinator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHelpFlowCoordinatorDidFinish", (helpFlowCoordinator))?;
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
    pub fn HandleMainMenuViewControllerDidFinish(
        &mut self,
        viewController: *mut MainMenuViewController,
        subMenuType: crate::GlobalNamespace::MainMenuViewController_MenuButton,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMainMenuViewControllerDidFinish",
                (viewController, subMenuType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMainMenuViewControllerPromoButtonWasPressed(
        &mut self,
        promoInfo: *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMainMenuViewControllerPromoButtonWasPressed", (promoInfo))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerDisclaimerDidFinishAction(
        &mut self,
        buttonNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplayerDisclaimerDidFinishAction", (buttonNumber))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplayerModeSelectionFlowCoordinatorDidFinish(
        &mut self,
        multiplayerModeSelectionFlowCoordinator: *mut MultiplayerModeSelectionFlowCoordinator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerModeSelectionFlowCoordinatorDidFinish",
                (multiplayerModeSelectionFlowCoordinator),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleOptionsViewControllerDidFinish(
        &mut self,
        optionsType: crate::GlobalNamespace::OptionsViewController_OptionsButton,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOptionsViewControllerDidFinish", (optionsType))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePartyFreePlayFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: *mut LevelSelectionFlowCoordinator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePartyFreePlayFlowCoordinatorDidFinish", (flowCoordinator))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerOptionsViewControllerDidFinish(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerOptionsViewControllerDidFinish", (viewController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSettingsFlowCoordinatorDidFinish(
        &mut self,
        settingsFlowCoordinator: *mut SettingsFlowCoordinator,
        finishAction: crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSettingsFlowCoordinatorDidFinish",
                (settingsFlowCoordinator, finishAction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSoloFreePlayFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: *mut LevelSelectionFlowCoordinator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSoloFreePlayFlowCoordinatorDidFinish", (flowCoordinator))?;
        Ok(__cordl_ret)
    }
    pub fn InitialViewControllerWasPresented(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitialViewControllerWasPresented", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PresentFlowCoordinatorOrAskForTutorial(
        &mut self,
        flowCoordinator: *mut crate::HMUI::FlowCoordinator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentFlowCoordinatorOrAskForTutorial", (flowCoordinator))?;
        Ok(__cordl_ret)
    }
    pub fn PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator(
        &mut self,
        presentImmediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator",
                (presentImmediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMenuDestinationRequest(
        &mut self,
        destination: *mut MenuDestination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMenuDestinationRequest", (destination))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMenuDestinationRequestAfterFrameCoroutine(
        &mut self,
        destination: *mut MenuDestination,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("ProcessMenuDestinationRequestAfterFrameCoroutine", (destination))?;
        Ok(__cordl_ret)
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: *mut crate::HMUI::ViewController,
        newViewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TopViewControllerWillChange",
                (oldViewController, newViewController, animationType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _HandleMainMenuViewControllerDidFinish_b__28_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleMainMenuViewControllerDidFinish>b__28_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _PresentFlowCoordinatorOrAskForTutorial_b__27_0(
        &mut self,
        buttonNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<PresentFlowCoordinatorOrAskForTutorial>b__27_0", (buttonNumber))?;
        Ok(__cordl_ret)
    }
    pub fn _PresentFlowCoordinatorOrAskForTutorial_b__27_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<PresentFlowCoordinatorOrAskForTutorial>b__27_1", ())?;
        Ok(__cordl_ret)
    }
    pub fn _PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator_b__42_0(
        &mut self,
        buttonNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "<PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator>b__42_0",
                (buttonNumber),
            )?;
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
#[cfg(feature = "MainFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType for MainFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
