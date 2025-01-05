#[cfg(feature = "BGLib+Polyglot+CsvWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct CsvWriter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+Polyglot+CsvWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::CsvWriter => "BGLib.Polyglot"
    ."CsvWriter"
);
#[cfg(feature = "BGLib+Polyglot+CsvWriter")]
impl std::ops::Deref for crate::BGLib::Polyglot::CsvWriter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+CsvWriter")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::CsvWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+CsvWriter")]
impl crate::BGLib::Polyglot::CsvWriter {
    pub fn AppendCSVLine_IEnumerable_1_0(
        buffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendCSVLine", (buffer, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendCSVLine_Il2CppArray1(
        buffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendCSVLine", (buffer, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendElement(
        buffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendElement", (buffer, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendRow(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        row: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendRow", (filePath, row))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendRowInternal(
        buffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        row: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendRowInternal", (buffer, row))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasEscapeChars(
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasEscapeChars", (element))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+Polyglot+CsvWriter")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::CsvWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
