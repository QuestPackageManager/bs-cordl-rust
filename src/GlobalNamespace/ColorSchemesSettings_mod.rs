#[cfg(feature = "ColorSchemesSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemesSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _colorOverrideType_k__BackingField: crate::GlobalNamespace::ColorSchemesSettings_ColorOverrideType,
    pub didChangeOverrideSettingsEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _colorSchemesList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::ColorScheme,
        >,
    >,
    pub _colorSchemesDict: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::ColorScheme,
        >,
    >,
    pub _selectedColorSchemeId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _overrideDefaultColors: bool,
}
#[cfg(feature = "ColorSchemesSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorSchemesSettings => ""
    ."ColorSchemesSettings"
);
#[cfg(feature = "ColorSchemesSettings")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemesSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemesSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorSchemesSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemesSettings")]
impl crate::GlobalNamespace::ColorSchemesSettings {
    #[cfg(feature = "ColorSchemesSettings+ColorOverrideType")]
    pub type ColorOverrideType = crate::GlobalNamespace::ColorSchemesSettings_ColorOverrideType;
    pub fn GetColorSchemeForId(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = __cordl_object.invoke("GetColorSchemeForId", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColorSchemeForIdx(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = __cordl_object.invoke("GetColorSchemeForIdx", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNumberOfColorSchemes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNumberOfColorSchemes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOverrideColorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = __cordl_object.invoke("GetOverrideColorScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedColorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = __cordl_object.invoke("GetSelectedColorScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedColorSchemeIdx(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSelectedColorSchemeIdx", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_IEnumerable_1_0(
        colorSchemes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ColorScheme,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorSchemes))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEnumerable_1_1(
        colorSchemeSOs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ColorSchemeSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorSchemeSOs))?;
        Ok(__cordl_object.into())
    }
    pub fn SetColorSchemeForId(
        &mut self,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorSchemeForId", (colorScheme))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldOverrideLightshowColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldOverrideLightshowColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_0(
        &mut self,
        colorSchemes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ColorScheme,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorSchemes))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_1(
        &mut self,
        colorSchemeSOs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ColorSchemeSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorSchemeSOs))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeOverrideSettingsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeOverrideSettingsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorOverrideType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ColorSchemesSettings_ColorOverrideType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ColorSchemesSettings_ColorOverrideType = __cordl_object
            .invoke("get_colorOverrideType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideDefaultColors(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_overrideDefaultColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedColorSchemeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_selectedColorSchemeId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeOverrideSettingsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeOverrideSettingsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colorOverrideType(
        &mut self,
        value: crate::GlobalNamespace::ColorSchemesSettings_ColorOverrideType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorOverrideType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overrideDefaultColors(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overrideDefaultColors", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectedColorSchemeId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectedColorSchemeId", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorSchemesSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorSchemesSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ColorSchemesSettings+ColorOverrideType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorSchemesSettings_ColorOverrideType {
    #[default]
    All = 0i32,
    NotesOnly = 1i32,
}
#[cfg(feature = "ColorSchemesSettings+ColorOverrideType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ColorSchemesSettings_ColorOverrideType => ""
    ."ColorSchemesSettings/ColorOverrideType"
);
