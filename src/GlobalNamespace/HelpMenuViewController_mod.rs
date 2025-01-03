#[cfg(feature = "HelpMenuViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct HelpMenuViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _helpMenuSegmentedControl: quest_hook::libil2cpp::Gc<
        crate::HMUI::TextSegmentedControl,
    >,
    pub _howToPlayViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HowToPlayViewController,
    >,
    pub _healthWarningDisplayViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HealthWarningDisplayViewController,
    >,
    pub _privacyPolicyDisplayViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PrivacyPolicyDisplayViewController,
    >,
    pub _eulaDisplayViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EulaDisplayViewController,
    >,
    pub _licensesDisplayViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LicensesDisplayViewController,
    >,
    pub didSelectHelpSubMenuEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::HMUI::ViewController>,
    >,
    pub _viewControllers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::System::ValueTuple_2<
                *mut crate::HMUI::ViewController,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    >,
}
#[cfg(feature = "HelpMenuViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HelpMenuViewController => ""
    ."HelpMenuViewController"
);
#[cfg(feature = "HelpMenuViewController")]
impl std::ops::Deref for crate::GlobalNamespace::HelpMenuViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HelpMenuViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::HelpMenuViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HelpMenuViewController")]
impl crate::GlobalNamespace::HelpMenuViewController {
    pub const kEulaMenu: &'static str = "EULA_MENU";
    pub const kHealthWarningMenu: &'static str = "HEALTH_AND_SAFETY_MENU";
    pub const kHowToPlayMenu: &'static str = "LABEL_HOW_TO_PLAY";
    pub const kLicensesMenu: &'static str = "SOFTWARE_LICENSES";
    pub const kPrivacyPolicyMenu: &'static str = "PRIVACY_POLICY_MENU";
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
    pub fn HandleHelpMenuSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl>,
        cellIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleHelpMenuSegmentedControlDidSelectCell",
                (segmentedControl, cellIdx),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn add_didSelectHelpSubMenuEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::HMUI::ViewController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectHelpSubMenuEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectHelpSubMenuEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::HMUI::ViewController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectHelpSubMenuEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HelpMenuViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HelpMenuViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
