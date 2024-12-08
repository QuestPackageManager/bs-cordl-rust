#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct LanguageExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LanguageExtensions =>
    "BGLib.Polyglot"."LanguageExtensions"
);
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl std::ops::Deref for crate::BGLib::Polyglot::LanguageExtensions {
    type Target = crate::System::Object;
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
impl crate::BGLib::Polyglot::LanguageExtensions {}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LanguageExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
