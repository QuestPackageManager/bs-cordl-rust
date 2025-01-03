#[cfg(feature = "HealthWarningFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthWarningFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _appInitScenesTransitionSetupDataContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AppInitScenesTransitionSetupDataContainerSO,
    >,
    pub _selectLanguageViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SelectLanguageViewController,
    >,
    pub _selectRegionViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SelectRegionViewController,
    >,
    pub _eulaViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EulaViewController,
    >,
    pub _privacyPolicyViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PrivacyPolicyViewController,
    >,
    pub _healthWarningViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HealthWarningViewController,
    >,
    pub _explicitContentWarningViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ExplicitContentWarningViewController,
    >,
    pub _noUserAgeWarningViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoUserAgeWarningViewController,
    >,
    pub _endOfLifeNoticeViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EndOfLifeNoticeViewController,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _fadeInOut: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FadeInOutController,
    >,
    pub _gameScenesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameScenesManager,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HealthWarningFlowCoordinator_InitData,
    >,
    pub _fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _platformInit: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::IPlatformInit>,
    pub _viewControllerTitles: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::HMUI::ViewController,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _selectedRegion: crate::GlobalNamespace::SelectRegionViewController_Region,
    pub _openedWithLanguage: crate::BGLib::Polyglot::Language,
}
#[cfg(feature = "HealthWarningFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HealthWarningFlowCoordinator =>
    ""."HealthWarningFlowCoordinator"
);
#[cfg(feature = "HealthWarningFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::HealthWarningFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningFlowCoordinator")]
impl std::ops::DerefMut for crate::GlobalNamespace::HealthWarningFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningFlowCoordinator")]
impl crate::GlobalNamespace::HealthWarningFlowCoordinator {
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
    pub fn EndOfLifeNoticeContinueFlow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndOfLifeNoticeContinueFlow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GoToNextScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("GoToNextScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEndOfLifeNoticeViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEndOfLifeNoticeViewControllerDidFinish", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleExplicitContentWarningViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleExplicitContentWarningViewControllerDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleHealthWarningViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHealthWarningViewControllerDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoUserAgeWarningViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoUserAgeWarningViewControllerDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePrivacyPolicyViewControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePrivacyPolicyViewControllerDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSelectLanguageViewControllerDidChangeLanguage(
        &mut self,
        language: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelectLanguageViewControllerDidChangeLanguage", (language))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSelectLanguageViewControllerDidPressContinueButtonAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSelectLanguageViewControllerDidPressContinueButtonAsync",
                (),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleUserAgeCategoryArrived(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleUserAgeCategoryArrived", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveMainViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("ResolveMainViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolvePlayerAgreementsViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("ResolvePlayerAgreementsViewController", ())?;
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
    pub fn WaitForUserAgeCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WaitForUserAgeCategory", ())?;
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
#[cfg(feature = "HealthWarningFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HealthWarningFlowCoordinator {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nextScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    >,
}
#[cfg(feature = "HealthWarningFlowCoordinator+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::HealthWarningFlowCoordinator_InitData => ""
    ."HealthWarningFlowCoordinator/InitData"
);
#[cfg(feature = "HealthWarningFlowCoordinator+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::HealthWarningFlowCoordinator_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        nextScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nextScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_ret.into())
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
