#[cfg(feature = "EnvironmentOverrideSettingsPanelController+Elements")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentOverrideSettingsPanelController_Elements {
    __cordl_parent: crate::System::Object,
    pub label: *mut crate::TMPro::TextMeshProUGUI,
    pub simpleTextDropdown: *mut crate::HMUI::SimpleTextDropdown,
    pub environmentType: EnvironmentType,
    pub localizationKey: *mut crate::System::String,
    pub _environmentInfos_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut EnvironmentInfoSO,
    >,
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController+Elements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EnvironmentOverrideSettingsPanelController_Elements => ""
    ."EnvironmentOverrideSettingsPanelController/Elements"
);
#[cfg(feature = "EnvironmentOverrideSettingsPanelController+Elements")]
impl std::ops::Deref
for crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController_Elements {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController+Elements")]
impl std::ops::DerefMut
for crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController_Elements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController+Elements")]
impl crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController_Elements {
    pub fn set_environmentInfos(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<*mut EnvironmentInfoSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_environmentInfos", (value))?;
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
    pub fn get_environmentInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut EnvironmentInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut EnvironmentInfoSO,
        > = __cordl_object.invoke("get_environmentInfos", ())?;
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
#[cfg(feature = "EnvironmentOverrideSettingsPanelController+Elements")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController_Elements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentOverrideSettingsPanelController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _overrideEnvironmentsToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _elementsGO: *mut crate::UnityEngine::GameObject,
    pub _elements: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController_Elements,
    >,
    pub _presentPanelAnimation: *mut crate::HMUI::PanelAnimationSO,
    pub _dismissPanelAnimation: *mut crate::HMUI::PanelAnimationSO,
    pub _environmentListModel: *mut EnvironmentsListModel,
    pub _analyticsModel: *mut IAnalyticsModel,
    pub _overrideEnvironmentSettings: *mut OverrideEnvironmentSettings,
    pub _initialized: bool,
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EnvironmentOverrideSettingsPanelController => ""
    ."EnvironmentOverrideSettingsPanelController"
);
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl std::ops::Deref for EnvironmentOverrideSettingsPanelController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl std::ops::DerefMut for EnvironmentOverrideSettingsPanelController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl EnvironmentOverrideSettingsPanelController {
    #[cfg(feature = "EnvironmentOverrideSettingsPanelController+Elements")]
    pub type Elements = crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController_Elements;
    #[cfg(feature = "EnvironmentOverrideSettingsPanelController+__c")]
    pub type __c = crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController___c;
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleOverrideEnvironmentsToggleValueChanged(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOverrideEnvironmentsToggleValueChanged", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        overrideEnvironmentSettings: *mut OverrideEnvironmentSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (overrideEnvironmentSettings))?;
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
    pub fn _HandleOverrideEnvironmentsToggleValueChanged_b__16_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleOverrideEnvironmentsToggleValueChanged>b__16_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_overrideEnvironmentSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut OverrideEnvironmentSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OverrideEnvironmentSettings = __cordl_object
            .invoke("get_overrideEnvironmentSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleDropDownDidSelectCellWithIdx(
        &mut self,
        dropDownWithTableView: *mut crate::HMUI::DropdownWithTableView,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDropDownDidSelectCellWithIdx", (dropDownWithTableView, idx))?;
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
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl quest_hook::libil2cpp::ObjectType for EnvironmentOverrideSettingsPanelController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
