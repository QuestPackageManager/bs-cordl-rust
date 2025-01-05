#[cfg(feature = "ColorsOverrideSettingsPanelController")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorsOverrideSettingsPanelController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _overrideColorsToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _detailsPanelGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _colorSchemeDropDown: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSchemeDropdown,
    >,
    pub _editColorSchemeController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EditColorSchemeController,
    >,
    pub _editColorSchemeModalView: quest_hook::libil2cpp::Gc<crate::HMUI::ModalView>,
    pub _editColorSchemeButton: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Button,
    >,
    pub _colorOverrideTypeDropdown: quest_hook::libil2cpp::Gc<
        crate::HMUI::SimpleTextDropdown,
    >,
    pub _presentPanelAnimation: quest_hook::libil2cpp::Gc<crate::HMUI::PanelAnimationSO>,
    pub _dismissPanelAnimation: quest_hook::libil2cpp::Gc<crate::HMUI::PanelAnimationSO>,
    pub _analyticsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAnalyticsModel,
    >,
    pub _colorSchemesSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSchemesSettings,
    >,
    pub _initialized: bool,
    pub _buttonBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
    pub _isDirty: bool,
}
#[cfg(feature = "ColorsOverrideSettingsPanelController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ColorsOverrideSettingsPanelController => ""
    ."ColorsOverrideSettingsPanelController"
);
#[cfg(feature = "ColorsOverrideSettingsPanelController")]
impl std::ops::Deref for crate::GlobalNamespace::ColorsOverrideSettingsPanelController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorsOverrideSettingsPanelController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ColorsOverrideSettingsPanelController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorsOverrideSettingsPanelController")]
impl crate::GlobalNamespace::ColorsOverrideSettingsPanelController {
    pub fn HandleColorOverrideTypeDropdownDidSelectCellWithIdx(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<crate::HMUI::DropdownWithTableView>,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleColorOverrideTypeDropdownDidSelectCellWithIdx",
                (_cordl__, idx),
            )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn HandleEditColorSchemeButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEditColorSchemeButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEditColorSchemeControllerDidChangeColorScheme(
        &mut self,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleEditColorSchemeControllerDidChangeColorScheme",
                (colorScheme),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEditColorSchemeControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEditColorSchemeControllerDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleOverrideColorsToggleValueChanged(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOverrideColorsToggleValueChanged", (isOn))?;
        Ok(__cordl_ret.into())
    }
    pub fn HideDropdown(
        &mut self,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideDropdown", (animated))?;
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
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
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
        colorSchemesSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (colorSchemesSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn _HandleOverrideColorsToggleValueChanged_b__23_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleOverrideColorsToggleValueChanged>b__23_0", ())?;
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
    pub fn get_editColorSchemeModalView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ModalView>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ModalView> = __cordl_object
            .invoke("get_editColorSchemeModalView", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorsOverrideSettingsPanelController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorsOverrideSettingsPanelController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ColorsOverrideSettingsPanelController")]
impl AsRef<crate::GlobalNamespace::IRefreshable>
for crate::GlobalNamespace::ColorsOverrideSettingsPanelController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IRefreshable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ColorsOverrideSettingsPanelController")]
impl AsMut<crate::GlobalNamespace::IRefreshable>
for crate::GlobalNamespace::ColorsOverrideSettingsPanelController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IRefreshable {
        unsafe { std::mem::transmute(self) }
    }
}
