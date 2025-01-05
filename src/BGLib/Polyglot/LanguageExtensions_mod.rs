#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct LanguageExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LanguageExtensions =>
    "BGLib.Polyglot"."LanguageExtensions"
);
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl std::ops::Deref for crate::BGLib::Polyglot::LanguageExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LanguageExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl crate::BGLib::Polyglot::LanguageExtensions {
    pub fn ToCultureInfoName(
        lang: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToCultureInfoName", (lang))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLanguage_Gc0(
        serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_ret: crate::BGLib::Polyglot::Language = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLanguage", (serializedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLanguage_SystemLanguage__cordl_bool1(
        systemLanguage: crate::UnityEngine::SystemLanguage,
        useFallbackLanguage: bool,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_ret: crate::BGLib::Polyglot::Language = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLanguage", (systemLanguage, useFallbackLanguage))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSerializedName(
        lang: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSerializedName", (lang))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LanguageExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
