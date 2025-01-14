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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ColorsOverrideSettingsPanelController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ColorsOverrideSettingsPanelController";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::DropdownWithTableView>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleColorOverrideTypeDropdownDidSelectCellWithIdx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleColorOverrideTypeDropdownDidSelectCellWithIdx", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl__, idx))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleDropDownDidSelectCellWithIdx(
        &mut self,
        dropDownWithTableView: quest_hook::libil2cpp::Gc<
            crate::HMUI::DropdownWithTableView,
        >,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::DropdownWithTableView>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleDropDownDidSelectCellWithIdx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleDropDownDidSelectCellWithIdx", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dropDownWithTableView, idx))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleEditColorSchemeButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleEditColorSchemeButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleEditColorSchemeButtonWasPressed", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleEditColorSchemeControllerDidChangeColorScheme(
        &mut self,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleEditColorSchemeControllerDidChangeColorScheme")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleEditColorSchemeControllerDidChangeColorScheme", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (colorScheme))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleEditColorSchemeControllerDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleEditColorSchemeControllerDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleEditColorSchemeControllerDidFinish", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleOverrideColorsToggleValueChanged(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleOverrideColorsToggleValueChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleOverrideColorsToggleValueChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (isOn))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HideDropdown(
        &mut self,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("HideDropdown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HideDropdown", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (animated))
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
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDestroy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDisable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnEnable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Refresh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Refresh", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        colorSchemesSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ColorSchemesSettings,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (colorSchemesSettings))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _HandleOverrideColorsToggleValueChanged_b__23_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("<HandleOverrideColorsToggleValueChanged>b__23_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<HandleOverrideColorsToggleValueChanged>b__23_0", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_editColorSchemeModalView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ModalView>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::HMUI::ModalView>,
                0usize,
            >("get_editColorSchemeModalView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_editColorSchemeModalView", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ModalView> = unsafe {
            method.invoke_unchecked(self, ())
        };
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
