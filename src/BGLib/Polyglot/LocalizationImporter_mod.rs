#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizationImporter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LocalizationImporter =>
    "BGLib.Polyglot"."LocalizationImporter"
);
#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizationImporter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LocalizationImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
impl crate::BGLib::Polyglot::LocalizationImporter {
    pub fn GetKeys() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetKeys", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLanguagesContains(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLanguagesContains", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLanguagesStartsWith(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLanguagesStartsWith", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLanguages_IReadOnlyList_1_1(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        supportedLanguages: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                crate::BGLib::Polyglot::Language,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLanguages", (key, supportedLanguages))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLanguages_Il2CppString0(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLanguages", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn Import(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Import", (text, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportFromFiles(
        settings: quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::LocalizationModel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImportFromFiles", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportInputFiles() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImportInputFiles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportTextFile(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImportTextFile", (text, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        settings: quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::LocalizationModel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLineBreak(
        currentString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLineBreak", (currentString))?;
        Ok(__cordl_ret.into())
    }
    pub fn NoDomainReloadInit() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NoDomainReloadInit", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LocalizationImporter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
