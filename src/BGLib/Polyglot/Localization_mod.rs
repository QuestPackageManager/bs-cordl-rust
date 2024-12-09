#[cfg(feature = "BGLib+Polyglot+Localization")]
#[repr(C)]
#[derive(Debug)]
pub struct Localization {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub supportedLanguages: *mut crate::System::Collections::Generic::List_1<
        crate::BGLib::Polyglot::Language,
    >,
    pub selectedLanguage: crate::BGLib::Polyglot::Language,
    pub fallbackLanguage: crate::BGLib::Polyglot::Language,
}
#[cfg(feature = "BGLib+Polyglot+Localization")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::Localization =>
    "BGLib.Polyglot"."Localization"
);
#[cfg(feature = "BGLib+Polyglot+Localization")]
impl std::ops::Deref for crate::BGLib::Polyglot::Localization {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+Localization")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::Localization {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+Localization")]
impl crate::BGLib::Polyglot::Localization {
    pub const KeyNotFound: &'static str = "[{0}]";
    pub fn HasNoSupportedLanguage(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasNoSupportedLanguage", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsLanguageSupported(
        &mut self,
        language: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsLanguageSupported", (language))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_EnglishLanguageName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_EnglishLanguageName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FallbackLanguage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::Language = __cordl_object
            .invoke("get_FallbackLanguage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalizedLanguageName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_LocalizedLanguageName", ())?;
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
    pub fn get_selectedLanguageIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectedLanguageIndex", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BGLib+Polyglot+Localization")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::Localization {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
