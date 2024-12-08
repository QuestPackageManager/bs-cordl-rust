#[cfg(feature = "BGLib+Polyglot+LocalizationModel")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizationModel {
    __cordl_parent: crate::System::Object,
    pub localization: *mut crate::BGLib::Polyglot::Localization,
    pub inputFiles: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BGLib::Polyglot::LocalizationAsset,
    >,
    pub selectedCulture: *mut crate::System::Globalization::CultureInfo,
    pub _onChangeLanguage: *mut crate::System::Action_1<
        *mut crate::BGLib::Polyglot::LocalizationModel,
    >,
}
#[cfg(feature = "BGLib+Polyglot+LocalizationModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LocalizationModel =>
    "BGLib.Polyglot"."LocalizationModel"
);
#[cfg(feature = "BGLib+Polyglot+LocalizationModel")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizationModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationModel")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LocalizationModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationModel")]
impl crate::BGLib::Polyglot::LocalizationModel {
    #[cfg(feature = "BGLib+Polyglot+LocalizationModel+__c__DisplayClass30_0")]
    pub type __c__DisplayClass30_0 = crate::BGLib::Polyglot::LocalizationModel___c__DisplayClass30_0;
    pub fn get_fallbackLanguage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::Language = __cordl_object
            .invoke("get_fallbackLanguage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalizedLanguageNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_LocalizedLanguageNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFormatOrKey(
        &mut self,
        key: *mut crate::System::String,
        arguments: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetFormatOrKey", (key, arguments))?;
        Ok(__cordl_ret)
    }
    pub fn AddOnLocalizeEvent(
        &mut self,
        localize: *mut crate::BGLib::Polyglot::ILocalize,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOnLocalizeEvent", (localize))?;
        Ok(__cordl_ret)
    }
    pub fn set_SelectedCultureInfo(
        &mut self,
        value: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SelectedCultureInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn InputFilesContains(
        &mut self,
        doc: *mut crate::BGLib::Polyglot::LocalizationDocument,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InputFilesContains", (doc))?;
        Ok(__cordl_ret)
    }
    pub fn get_SelectedCultureInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::CultureInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::CultureInfo = __cordl_object
            .invoke("get_SelectedCultureInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn add__onChangeLanguage(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::BGLib::Polyglot::LocalizationModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add__onChangeLanguage", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_SelectedLanguageDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::LanguageDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::LanguageDirection = __cordl_object
            .invoke("get_SelectedLanguageDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectLanguage(
        &mut self,
        selected: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectLanguage", (selected))?;
        Ok(__cordl_ret)
    }
    pub fn Get(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Get", (key))?;
        Ok(__cordl_ret)
    }
    pub fn get_SelectedLanguageIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SelectedLanguageIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryGet(
        &mut self,
        key: *mut crate::System::String,
        language: crate::BGLib::Polyglot::Language,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGet", (key, language, value))?;
        Ok(__cordl_ret)
    }
    pub fn KeyExist(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("KeyExist", (key))?;
        Ok(__cordl_ret)
    }
    pub fn set_SelectedLanguage(
        &mut self,
        value: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SelectedLanguage", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetOrKey(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetOrKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn get_SelectedLanguage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::Language = __cordl_object
            .invoke("get_SelectedLanguage", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        localization: *mut crate::BGLib::Polyglot::Localization,
        language: crate::BGLib::Polyglot::Language,
        inputFiles: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BGLib::Polyglot::LocalizationAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (localization, language, inputFiles))?;
        Ok(__cordl_ret)
    }
    pub fn get_SupportedLanguages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            crate::BGLib::Polyglot::Language,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            crate::BGLib::Polyglot::Language,
        > = __cordl_object.invoke("get_SupportedLanguages", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InputFiles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BGLib::Polyglot::LocalizationAsset,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BGLib::Polyglot::LocalizationAsset,
        > = __cordl_object.invoke("get_InputFiles", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove__onChangeLanguage(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::BGLib::Polyglot::LocalizationModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove__onChangeLanguage", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetCultureInfo(
        &mut self,
        language: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::CultureInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::CultureInfo = __cordl_object
            .invoke("GetCultureInfo", (language))?;
        Ok(__cordl_ret)
    }
    pub fn get_EnglishLanguageNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_EnglishLanguageNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        localization: *mut crate::BGLib::Polyglot::Localization,
        language: crate::BGLib::Polyglot::Language,
        inputFiles: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BGLib::Polyglot::LocalizationAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (localization, language, inputFiles))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationModel")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LocalizationModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
