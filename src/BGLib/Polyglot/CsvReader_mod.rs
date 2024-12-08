#[cfg(feature = "BGLib+Polyglot+CsvReader")]
#[repr(C)]
#[derive(Debug)]
pub struct CsvReader {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BGLib+Polyglot+CsvReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::CsvReader => "BGLib.Polyglot"
    ."CsvReader"
);
#[cfg(feature = "BGLib+Polyglot+CsvReader")]
impl std::ops::Deref for crate::BGLib::Polyglot::CsvReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+CsvReader")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::CsvReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+CsvReader")]
impl crate::BGLib::Polyglot::CsvReader {
    #[cfg(feature = "BGLib+Polyglot+CsvReader+ParsingMode")]
    pub type ParsingMode = crate::BGLib::Polyglot::CsvReader_ParsingMode;
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
}
#[cfg(feature = "BGLib+Polyglot+CsvReader")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::CsvReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+Polyglot+CsvReader+ParsingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsvReader_ParsingMode {
    InQuote = 2i32,
    None = 0i32,
    OutQuote = 1i32,
}
#[cfg(feature = "BGLib+Polyglot+CsvReader+ParsingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::CsvReader_ParsingMode =>
    "BGLib.Polyglot"."CsvReader/ParsingMode"
);