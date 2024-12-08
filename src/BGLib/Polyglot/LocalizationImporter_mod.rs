#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizationImporter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LocalizationImporter =>
    "BGLib.Polyglot"."LocalizationImporter"
);
#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizationImporter {
    type Target = crate::System::Object;
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
impl crate::BGLib::Polyglot::LocalizationImporter {}
#[cfg(feature = "BGLib+Polyglot+LocalizationImporter")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LocalizationImporter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
