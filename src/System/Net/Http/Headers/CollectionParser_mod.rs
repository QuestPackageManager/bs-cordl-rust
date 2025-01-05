#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::CollectionParser =>
    "System.Net.Http.Headers"."CollectionParser"
);
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
impl std::ops::Deref for crate::System::Net::Http::Headers::CollectionParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::CollectionParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
impl crate::System::Net::Http::Headers::CollectionParser {
    pub fn TryParseStringElement(
        lexer: quest_hook::libil2cpp::Gc<crate::System::Net::Http::Headers::Lexer>,
        parsedValue: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        t: quest_hook::libil2cpp::ByRefMut<crate::System::Net::Http::Headers::Token>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseStringElement", (lexer, parsedValue, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ByRefMut1(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        minimalCount: i32,
        result: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, minimalCount, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ElementTryParser_1_ByRefMut0<T>(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        minimalCount: i32,
        parser: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::Headers::ElementTryParser_1<T>,
        >,
        result: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, minimalCount, parser, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::CollectionParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
