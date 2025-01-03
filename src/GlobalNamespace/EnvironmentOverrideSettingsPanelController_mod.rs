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
    pub _environmentListModel: *mut crate::GlobalNamespace::EnvironmentsListModel,
    pub _analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
    pub _overrideEnvironmentSettings: *mut crate::GlobalNamespace::OverrideEnvironmentSettings,
    pub _initialized: bool,
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EnvironmentOverrideSettingsPanelController => ""
    ."EnvironmentOverrideSettingsPanelController"
);
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl std::ops::Deref
for crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController {
    #[cfg(feature = "EnvironmentOverrideSettingsPanelController+Elements")]
    pub type Elements = crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController_Elements;
    pub fn HandleDropDownDidSelectCellWithIdx(
        &mut self,
        dropDownWithTableView: quest_hook::libil2cpp::Gc<
            crate::HMUI::DropdownWithTableView,
        >,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDropDownDidSelectCellWithIdx", (dropDownWithTableView, idx))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        overrideEnvironmentSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (overrideEnvironmentSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn _HandleOverrideEnvironmentsToggleValueChanged_b__14_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleOverrideEnvironmentsToggleValueChanged>b__14_0", ())?;
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
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl AsRef<crate::GlobalNamespace::IRefreshable>
for crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IRefreshable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController")]
impl AsMut<crate::GlobalNamespace::IRefreshable>
for crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IRefreshable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EnvironmentOverrideSettingsPanelController+Elements")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentOverrideSettingsPanelController_Elements {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub label: *mut crate::TMPro::TextMeshProUGUI,
    pub simpleTextDropdown: *mut crate::HMUI::SimpleTextDropdown,
    pub environmentType: crate::GlobalNamespace::EnvironmentType,
    pub localizationKey: *mut quest_hook::libil2cpp::Il2CppString,
    pub _environmentInfos_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::EnvironmentInfoSO,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn get_environmentInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::EnvironmentInfoSO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::EnvironmentInfoSO,
            >,
        > = __cordl_object.invoke("get_environmentInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_environmentInfos(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::EnvironmentInfoSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_environmentInfos", (value))?;
        Ok(__cordl_ret.into())
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
