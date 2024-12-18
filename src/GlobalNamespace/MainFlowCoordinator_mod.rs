#[cfg(feature = "MainFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct MainFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _defaultLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _soloFreePlayFlowCoordinator: *mut crate::GlobalNamespace::SoloFreePlayFlowCoordinator,
    pub _partyFreePlayFlowCoordinator: *mut crate::GlobalNamespace::PartyFreePlayFlowCoordinator,
    pub _campaignFlowCoordinator: *mut crate::GlobalNamespace::CampaignFlowCoordinator,
    pub _settingsFlowCoordinator: *mut crate::GlobalNamespace::SettingsFlowCoordinator,
    pub _multiplayerModeSelectionFlowCoordinator: *mut crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
    pub _helpFlowCoordinator: *mut crate::GlobalNamespace::HelpFlowCoordinator,
    pub _editAvatarFlowCoordinatorHelper: *mut crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper,
    pub _simpleDialogPromptViewController: *mut crate::GlobalNamespace::SimpleDialogPromptViewController,
    pub _mainMenuViewController: *mut crate::GlobalNamespace::MainMenuViewController,
    pub _playerOptionsViewController: *mut crate::GlobalNamespace::PlayerOptionsViewController,
    pub _optionsViewController: *mut crate::GlobalNamespace::OptionsViewController,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _menuLightsManager: *mut crate::GlobalNamespace::MenuLightsManager,
    pub _fadeInOut: *mut crate::GlobalNamespace::FadeInOutController,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _menuTransitionsHelper: *mut crate::GlobalNamespace::MenuTransitionsHelper,
    pub _analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
    pub _avatarSystemCollection: *mut crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    pub _menuDestinationRequest: *mut crate::GlobalNamespace::MenuDestination,
    pub _afterDialogPromptFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
    pub _goToMultiplayerAfterAvatarCreation: bool,
}
#[cfg(feature = "MainFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MainFlowCoordinator => ""
    ."MainFlowCoordinator"
);
#[cfg(feature = "MainFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::MainFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainFlowCoordinator")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainFlowCoordinator")]
impl crate::GlobalNamespace::MainFlowCoordinator {
    #[cfg(
        feature = "MainFlowCoordinator+_HandleEditAvatarFlowCoordinatorHelperDidFinish_d__34"
    )]
    pub type _HandleEditAvatarFlowCoordinatorHelperDidFinish_d__34 = crate::GlobalNamespace::MainFlowCoordinator__HandleEditAvatarFlowCoordinatorHelperDidFinish_d__34;
    #[cfg(
        feature = "MainFlowCoordinator+_HandleMultiplayerDisclaimerDidFinishAction_d__29"
    )]
    pub type _HandleMultiplayerDisclaimerDidFinishAction_d__29 = crate::GlobalNamespace::MainFlowCoordinator__HandleMultiplayerDisclaimerDidFinishAction_d__29;
    #[cfg(
        feature = "MainFlowCoordinator+_PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator_d__42"
    )]
    pub type _PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator_d__42 = crate::GlobalNamespace::MainFlowCoordinator__PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator_d__42;
    #[cfg(
        feature = "MainFlowCoordinator+_ProcessMenuDestinationRequestAfterFrameCoroutine_d__43"
    )]
    pub type _ProcessMenuDestinationRequestAfterFrameCoroutine_d__43 = crate::GlobalNamespace::MainFlowCoordinator__ProcessMenuDestinationRequestAfterFrameCoroutine_d__43;
    #[cfg(feature = "MainFlowCoordinator+_ProcessMenuDestinationRequest_d__41")]
    pub type _ProcessMenuDestinationRequest_d__41 = crate::GlobalNamespace::MainFlowCoordinator__ProcessMenuDestinationRequest_d__41;
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackButtonWasPressed", (topViewController))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleCampaignFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CampaignFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCampaignFlowCoordinatorDidFinish", (flowCoordinator))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEditAvatarFlowCoordinatorHelperDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleHelpFlowCoordinatorDidFinish(
        &mut self,
        helpFlowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HelpFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHelpFlowCoordinatorDidFinish", (helpFlowCoordinator))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleMainMenuViewControllerDidFinish(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MainMenuViewController,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleMainMenuViewControllerPromoButtonWasPressed(
        &mut self,
        promoInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMainMenuViewControllerPromoButtonWasPressed", (promoInfo))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerModeSelectionFlowCoordinatorDidFinish(
        &mut self,
        multiplayerModeSelectionFlowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerModeSelectionFlowCoordinatorDidFinish",
                (multiplayerModeSelectionFlowCoordinator),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandlePartyFreePlayFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePartyFreePlayFlowCoordinatorDidFinish", (flowCoordinator))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerOptionsViewControllerDidFinish(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerOptionsViewControllerDidFinish", (viewController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSettingsFlowCoordinatorDidFinish(
        &mut self,
        settingsFlowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsFlowCoordinator,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleSoloFreePlayFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSoloFreePlayFlowCoordinatorDidFinish", (flowCoordinator))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitialViewControllerWasPresented(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitialViewControllerWasPresented", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PresentFlowCoordinatorOrAskForTutorial(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentFlowCoordinatorOrAskForTutorial", (flowCoordinator))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMenuDestinationRequest(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuDestination>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMenuDestinationRequest", (destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMenuDestinationRequestAfterFrameCoroutine(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuDestination>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke("ProcessMenuDestinationRequestAfterFrameCoroutine", (destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _HandleMainMenuViewControllerDidFinish_b__28_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleMainMenuViewControllerDidFinish>b__28_0", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _PresentFlowCoordinatorOrAskForTutorial_b__27_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<PresentFlowCoordinatorOrAskForTutorial>b__27_1", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MainFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MainFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
