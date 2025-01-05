#[cfg(feature = "SettingsFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsFlowCoordinator>,
            crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction,
        >,
    >,
    pub _mainSettingsMenuViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainSettingsMenuViewController,
    >,
    pub _settingsNavigationController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsNavigationController,
    >,
    pub fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _settingsApplicator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsApplicatorSO,
    >,
    pub _controllerProfilesModel: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::GameSettings::ControllerProfilesModel,
    >,
    pub _dropdownDialogPromptViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DropdownDialogPromptViewController,
    >,
    pub _controllerProfilesViewController: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::GameSettings::ControllerProfilesSettingsViewController,
    >,
    pub _undoSettings: crate::BeatSaber::Settings::Settings,
}
#[cfg(feature = "SettingsFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SettingsFlowCoordinator => ""
    ."SettingsFlowCoordinator"
);
#[cfg(feature = "SettingsFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::SettingsFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsFlowCoordinator")]
impl std::ops::DerefMut for crate::GlobalNamespace::SettingsFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsFlowCoordinator")]
impl crate::GlobalNamespace::SettingsFlowCoordinator {
    #[cfg(feature = "SettingsFlowCoordinator+FinishAction")]
    pub type FinishAction = crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction;
    pub fn ApplySettingsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ApplySettingsAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CancelSettingsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("CancelSettingsAsync", ())?;
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
    pub fn HandleDidSelectSettingsSubMenu(
        &mut self,
        settingsSubMenuInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsSubMenuInfo,
        >,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidSelectSettingsSubMenu", (settingsSubMenuInfo, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRequestToSelectProfileToCopyFrom(
        &mut self,
        profilesToCopyFrom: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRequestToSelectProfileToCopyFrom", (profilesToCopyFrom))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReplaceViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplaceViewController", (viewController))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowSecretViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowSecretViewController", (viewController))?;
        Ok(__cordl_ret.into())
    }
    pub fn _HandleRequestToSelectProfileToCopyFrom_b__20_0(
        &mut self,
        arguments: crate::System::ValueTuple_2<i32, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleRequestToSelectProfileToCopyFrom>b__20_0", (arguments))?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SettingsFlowCoordinator,
                >,
                crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SettingsFlowCoordinator,
                >,
                crate::GlobalNamespace::SettingsFlowCoordinator_FinishAction,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SettingsFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SettingsFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SettingsFlowCoordinator+FinishAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsFlowCoordinator_FinishAction {
    #[default]
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
