#[cfg(feature = "ColorSchemeColorsToggleGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemeColorsToggleGroup {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saberAColorToggleController: *mut crate::GlobalNamespace::ColorSchemeColorToggleController,
    pub _saberBColorToggleController: *mut crate::GlobalNamespace::ColorSchemeColorToggleController,
    pub _environmentColor0ToggleController: *mut crate::GlobalNamespace::ColorSchemeColorToggleController,
    pub _environmentColor1ToggleController: *mut crate::GlobalNamespace::ColorSchemeColorToggleController,
    pub _obstaclesColorToggleController: *mut crate::GlobalNamespace::ColorSchemeColorToggleController,
    pub _environmentColor0BoostToggleController: *mut crate::GlobalNamespace::ColorSchemeColorToggleController,
    pub _environmentColor1BoostToggleController: *mut crate::GlobalNamespace::ColorSchemeColorToggleController,
    pub selectedColorDidChangeEvent: *mut crate::System::Action_1<
        crate::UnityEngine::Color,
    >,
    pub _toggleBinder: *mut crate::HMUI::ToggleBinder,
    pub _selectedColorToggleController: *mut crate::GlobalNamespace::ColorSchemeColorToggleController,
    pub _colorScheme: *mut crate::GlobalNamespace::ColorScheme,
}
#[cfg(feature = "ColorSchemeColorsToggleGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorSchemeColorsToggleGroup =>
    ""."ColorSchemeColorsToggleGroup"
);
#[cfg(feature = "ColorSchemeColorsToggleGroup")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemeColorsToggleGroup {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeColorsToggleGroup")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorSchemeColorsToggleGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeColorsToggleGroup")]
impl crate::GlobalNamespace::ColorSchemeColorsToggleGroup {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateColorSchemeFromEditedColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ColorScheme> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ColorScheme = __cordl_object
            .invoke("CreateColorSchemeFromEditedColors", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleToggleWasSelected(
        &mut self,
        toggleController: *mut crate::GlobalNamespace::ColorSchemeColorToggleController,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleToggleWasSelected", (toggleController, isOn))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
    pub fn SetColorScheme(
        &mut self,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorScheme", (colorScheme))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__17_0(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__17_0", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__17_1(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__17_1", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__17_2(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__17_2", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__17_3(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__17_3", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__17_4(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__17_4", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__17_5(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__17_5", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__17_6(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__17_6", (isOn))?;
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
    pub fn add_selectedColorDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectedColorDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectedColorDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectedColorDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ColorSchemeColorsToggleGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorSchemeColorsToggleGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
