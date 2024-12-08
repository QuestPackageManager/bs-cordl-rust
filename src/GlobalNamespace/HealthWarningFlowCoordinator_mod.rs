#[cfg(feature = "HealthWarningFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthWarningFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _appInitScenesTransitionSetupDataContainer: *mut AppInitScenesTransitionSetupDataContainerSO,
    pub _selectLanguageViewController: *mut SelectLanguageViewController,
    pub _selectRegionViewController: *mut SelectRegionViewController,
    pub _eulaViewController: *mut EulaViewController,
    pub _privacyPolicyViewController: *mut PrivacyPolicyViewController,
    pub _healthWarningViewController: *mut HealthWarningViewController,
    pub _explicitContentWarningViewController: *mut ExplicitContentWarningViewController,
    pub _noUserAgeWarningViewController: *mut NoUserAgeWarningViewController,
    pub _endOfLifeNoticeViewController: *mut EndOfLifeNoticeViewController,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _fadeInOut: *mut FadeInOutController,
    pub _gameScenesManager: *mut GameScenesManager,
    pub _initData: *mut crate::GlobalNamespace::HealthWarningFlowCoordinator_InitData,
    pub _flushingService: *mut crate::BGLib::SaveDataCore::SaveDataFlushingService,
    pub _platformInit: *mut IPlatformInit,
    pub _viewControllerTitles: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::HMUI::ViewController,
        *mut crate::System::String,
    >,
    pub _selectedRegion: crate::GlobalNamespace::SelectRegionViewController_Region,
}
#[cfg(feature = "HealthWarningFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for HealthWarningFlowCoordinator => ""
    ."HealthWarningFlowCoordinator"
);
#[cfg(feature = "HealthWarningFlowCoordinator")]
impl std::ops::Deref for HealthWarningFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningFlowCoordinator")]
impl std::ops::DerefMut for HealthWarningFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningFlowCoordinator")]
impl HealthWarningFlowCoordinator {
    #[cfg(
        feature = "HealthWarningFlowCoordinator+_HandleSelectLanguageViewControllerDidChangeLanguageAsync_d__21"
    )]
    pub type _HandleSelectLanguageViewControllerDidChangeLanguageAsync_d__21 = crate::GlobalNamespace::HealthWarningFlowCoordinator__HandleSelectLanguageViewControllerDidChangeLanguageAsync_d__21;
    #[cfg(feature = "HealthWarningFlowCoordinator+_WaitForUserAgeCategory_d__29")]
    pub type _WaitForUserAgeCategory_d__29 = crate::GlobalNamespace::HealthWarningFlowCoordinator__WaitForUserAgeCategory_d__29;
    #[cfg(feature = "HealthWarningFlowCoordinator+InitData")]
    pub type InitData = crate::GlobalNamespace::HealthWarningFlowCoordinator_InitData;
    pub fn CheckPlayerSensitivityFlagAndContinueFlow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckPlayerSensitivityFlagAndContinueFlow", ())?;
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
    pub fn EndOfLifeNoticeContinueFlow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndOfLifeNoticeContinueFlow", ())?;
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
    pub fn HandleEndOfLifeNoticeViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEndOfLifeNoticeViewControllerDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleEulaViewControllerDidFinish(
        &mut self,
        buttonType: crate::GlobalNamespace::EulaViewController_ButtonType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEulaViewControllerDidFinish", (buttonType))?;
        Ok(__cordl_ret)
    }
    pub fn HandleExplicitContentWarningViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleExplicitContentWarningViewControllerDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleHealthWarningViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHealthWarningViewControllerDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoUserAgeWarningViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoUserAgeWarningViewControllerDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePrivacyPolicyViewControllerDidFinish(
        &mut self,
        buttonType: crate::GlobalNamespace::PrivacyPolicyViewController_ButtonType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePrivacyPolicyViewControllerDidFinish", (buttonType))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectLanguageViewControllerDidChangeLanguageAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelectLanguageViewControllerDidChangeLanguageAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectLanguageViewControllerDidPressContinueButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelectLanguageViewControllerDidPressContinueButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectRegionViewControllerDidPressContinueButton(
        &mut self,
        region: crate::GlobalNamespace::SelectRegionViewController_Region,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelectRegionViewControllerDidPressContinueButton", (region))?;
        Ok(__cordl_ret)
    }
    pub fn HandleUserAgeCategoryArrived(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleUserAgeCategoryArrived", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ResolveMainViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("ResolveMainViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolvePlayerAgreementsViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("ResolvePlayerAgreementsViewController", ())?;
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
    pub fn WaitForUserAgeCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WaitForUserAgeCategory", ())?;
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
#[cfg(feature = "HealthWarningFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType for HealthWarningFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HealthWarningFlowCoordinator+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthWarningFlowCoordinator_InitData {
    __cordl_parent: crate::System::Object,
    pub nextScenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
}
#[cfg(feature = "HealthWarningFlowCoordinator+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::HealthWarningFlowCoordinator_InitData => ""
    ."HealthWarningFlowCoordinator/InitData"
);
#[cfg(feature = "HealthWarningFlowCoordinator+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::HealthWarningFlowCoordinator_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningFlowCoordinator+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::HealthWarningFlowCoordinator_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningFlowCoordinator+InitData")]
impl crate::GlobalNamespace::HealthWarningFlowCoordinator_InitData {
    pub fn New(
        nextScenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        nextScenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HealthWarningFlowCoordinator+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HealthWarningFlowCoordinator_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
