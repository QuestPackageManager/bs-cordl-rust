#[cfg(feature = "ColorSchemesSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemesSettings {
    __cordl_parent: crate::System::Object,
    pub overrideDefaultColors: bool,
    pub _colorSchemesList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::ColorScheme,
    >,
    pub _colorSchemesDict: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::ColorScheme,
    >,
    pub _selectedColorSchemeId: *mut crate::System::String,
}
#[cfg(feature = "ColorSchemesSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorSchemesSettings => ""
    ."ColorSchemesSettings"
);
#[cfg(feature = "ColorSchemesSettings")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemesSettings {
    type Target = crate::System::Object;
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
    #[cfg(feature = "ColorSchemesSettings+__c")]
    pub type __c = crate::GlobalNamespace::ColorSchemesSettings___c;
    pub fn GetColorSchemeForId(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ColorScheme> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ColorScheme = __cordl_object
            .invoke("GetColorSchemeForId", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorSchemeForIdx(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ColorScheme> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ColorScheme = __cordl_object
            .invoke("GetColorSchemeForIdx", (idx))?;
        Ok(__cordl_ret)
    }
    pub fn GetNumberOfColorSchemes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNumberOfColorSchemes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOverrideColorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ColorScheme> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ColorScheme = __cordl_object
            .invoke("GetOverrideColorScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSelectedColorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ColorScheme> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ColorScheme = __cordl_object
            .invoke("GetSelectedColorScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSelectedColorSchemeIdx(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSelectedColorSchemeIdx", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_IEnumerable_1_0(
        colorSchemes: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::ColorScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorSchemes))?;
        Ok(__cordl_object)
    }
    pub fn New_IEnumerable_1_1(
        colorSchemeSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::ColorSchemeSO,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorSchemeSOs))?;
        Ok(__cordl_object)
    }
    pub fn SetColorSchemeForId(
        &mut self,
        colorScheme: *mut crate::GlobalNamespace::ColorScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorSchemeForId", (colorScheme))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEnumerable_1_0(
        &mut self,
        colorSchemes: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::ColorScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorSchemes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEnumerable_1_1(
        &mut self,
        colorSchemeSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::ColorSchemeSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorSchemeSOs))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedColorSchemeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_selectedColorSchemeId", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_selectedColorSchemeId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectedColorSchemeId", (value))?;
        Ok(__cordl_ret)
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
