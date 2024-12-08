#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonFileHandler {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::JsonExtension::JsonFileHandler =>
    "BGLib.JsonExtension"."JsonFileHandler"
);
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
impl std::ops::Deref for crate::BGLib::JsonExtension::JsonFileHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
impl std::ops::DerefMut for crate::BGLib::JsonExtension::JsonFileHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
impl crate::BGLib::JsonExtension::JsonFileHandler {
    #[cfg(feature = "BGLib+JsonExtension+JsonFileHandler+__c__DisplayClass0_0_1")]
    pub type __c__DisplayClass0_0_1<T: quest_hook::libil2cpp::Type> = crate::BGLib::JsonExtension::JsonFileHandler___c__DisplayClass0_0_1<
        T,
    >;
}
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::JsonExtension::JsonFileHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
