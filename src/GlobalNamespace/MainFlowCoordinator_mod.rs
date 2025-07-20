#[cfg(feature = "MainFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct MainFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _defaultLightsPreset: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuLightsPresetSO,
    >,
    pub _soloFreePlayFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SoloFreePlayFlowCoordinator,
    >,
    pub _partyFreePlayFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PartyFreePlayFlowCoordinator,
    >,
    pub _campaignFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CampaignFlowCoordinator,
    >,
    pub _settingsFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsFlowCoordinator,
    >,
    pub _multiplayerModeSelectionFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
    >,
    pub _helpFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HelpFlowCoordinator,
    >,
    pub _editAvatarFlowCoordinatorHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper,
    >,
    pub _simpleDialogPromptViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SimpleDialogPromptViewController,
    >,
    pub _mainMenuViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainMenuViewController,
    >,
    pub _playerOptionsViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerOptionsViewController,
    >,
    pub _optionsViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OptionsViewController,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _menuLightsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuLightsManager,
    >,
    pub _fadeInOut: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FadeInOutController,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _menuTransitionsHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuTransitionsHelper,
    >,
    pub _analyticsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAnalyticsModel,
    >,
    pub _avatarSystemCollection: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    >,
    pub _additionalContentModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAdditionalContentModel,
    >,
    pub _menuDestinationRequest: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuDestination,
    >,
    pub _afterDialogPromptFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::HMUI::FlowCoordinator,
    >,
    pub _goToMultiplayerAfterAvatarCreation: bool,
}
#[cfg(feature = "MainFlowCoordinator")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::MainFlowCoordinator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MainFlowCoordinator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BackButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(), "BackButtonWasPressed",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (topViewController))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DidActivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(), "DidActivate", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (firstActivation, addedToHierarchy, screenSystemEnabling),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DidDeactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(), "DidDeactivate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (removedFromHierarchy, screenSystemDisabling))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleCampaignFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::CampaignFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::CampaignFlowCoordinator,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleCampaignFlowCoordinatorDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleCampaignFlowCoordinatorDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (flowCoordinator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleEditAvatarFlowCoordinatorHelperDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
        finishAction: crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                    crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleEditAvatarFlowCoordinatorHelperDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleEditAvatarFlowCoordinatorHelperDidFinish", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (flowCoordinator, finishAction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleHelpFlowCoordinatorDidFinish(
        &mut self,
        helpFlowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HelpFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HelpFlowCoordinator>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleHelpFlowCoordinatorDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleHelpFlowCoordinatorDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (helpFlowCoordinator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleHowToPlayViewControllerDidFinish(
        &mut self,
        howToPlayOptions: crate::GlobalNamespace::HowToPlayViewController_HowToPlayOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::HowToPlayViewController_HowToPlayOptions),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleHowToPlayViewControllerDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleHowToPlayViewControllerDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (howToPlayOptions))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMainMenuViewControllerDidFinish(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MainMenuViewController,
        >,
        subMenuType: crate::GlobalNamespace::MainMenuViewController_MenuButton,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::MainMenuViewController,
                    >,
                    crate::GlobalNamespace::MainMenuViewController_MenuButton,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleMainMenuViewControllerDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleMainMenuViewControllerDidFinish", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, subMenuType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMainMenuViewControllerPromoButtonWasPressed(
        &mut self,
        promoInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMainMenuViewControllerPromoButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleMainMenuViewControllerPromoButtonWasPressed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (promoInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerDisclaimerDidFinishAction(
        &mut self,
        buttonNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMultiplayerDisclaimerDidFinishAction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleMultiplayerDisclaimerDidFinishAction", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (buttonNumber))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerModeSelectionFlowCoordinatorDidFinish(
        &mut self,
        multiplayerModeSelectionFlowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerModeSelectionFlowCoordinator,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleMultiplayerModeSelectionFlowCoordinatorDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleMultiplayerModeSelectionFlowCoordinatorDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (multiplayerModeSelectionFlowCoordinator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleOptionsViewControllerDidFinish(
        &mut self,
        optionsType: crate::GlobalNamespace::OptionsViewController_OptionsButton,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::OptionsViewController_OptionsButton),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleOptionsViewControllerDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleOptionsViewControllerDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (optionsType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandlePartyFreePlayFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionFlowCoordinator,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandlePartyFreePlayFlowCoordinatorDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandlePartyFreePlayFlowCoordinatorDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (flowCoordinator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerOptionsViewControllerDidFinish(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandlePlayerOptionsViewControllerDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandlePlayerOptionsViewControllerDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSettingsFlowCoordinatorDidFinish(
        &mut self,
        settingsFlowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsFlowCoordinator,
        >,
        finishAction: crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::SettingsFlowCoordinator,
                    >,
                    crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleSettingsFlowCoordinatorDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleSettingsFlowCoordinatorDidFinish", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (settingsFlowCoordinator, finishAction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSoloFreePlayFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionFlowCoordinator,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleSoloFreePlayFlowCoordinatorDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleSoloFreePlayFlowCoordinatorDidFinish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (flowCoordinator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitialViewControllerWasPresented(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("InitialViewControllerWasPresented")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "InitialViewControllerWasPresented", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PresentFlowCoordinatorOrAskForTutorial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "PresentFlowCoordinatorOrAskForTutorial", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (flowCoordinator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator(
        &mut self,
        presentImmediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >(
                "PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (presentImmediately))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMenuDestinationRequest(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuDestination>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuDestination>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessMenuDestinationRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ProcessMenuDestinationRequest", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (destination))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMenuDestinationRequestAfterFrameCoroutine(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuDestination>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuDestination>),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                1usize,
            >("ProcessMenuDestinationRequestAfterFrameCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ProcessMenuDestinationRequestAfterFrameCoroutine", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, (destination))? };
        Ok(__cordl_ret.into())
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TopViewControllerWillChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "TopViewControllerWillChange", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (oldViewController, newViewController, animationType),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _HandleMainMenuViewControllerDidFinish_b__29_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("<HandleMainMenuViewControllerDidFinish>b__29_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "<HandleMainMenuViewControllerDidFinish>b__29_0", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _PresentFlowCoordinatorOrAskForTutorial_b__28_0(
        &mut self,
        buttonNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("<PresentFlowCoordinatorOrAskForTutorial>b__28_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "<PresentFlowCoordinatorOrAskForTutorial>b__28_0", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (buttonNumber))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _PresentFlowCoordinatorOrAskForTutorial_b__28_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("<PresentFlowCoordinatorOrAskForTutorial>b__28_1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "<PresentFlowCoordinatorOrAskForTutorial>b__28_1", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator_b__43_0(
        &mut self,
        buttonNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >(
                "<PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator>b__43_0",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "<PresentMultiplayerModeSelectionFlowCoordinatorWithDisclaimerAndAvatarCreator>b__43_0",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (buttonNumber))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MainFlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MainFlowCoordinator as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
