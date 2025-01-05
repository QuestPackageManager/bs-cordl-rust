#[cfg(feature = "System+Net+Http+Headers+TransferCodingWithQualityHeaderValue")]
#[repr(C)]
#[derive(Debug)]
pub struct TransferCodingWithQualityHeaderValue {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Net::Http::Headers::TransferCodingHeaderValue,
    >,
}
#[cfg(feature = "System+Net+Http+Headers+TransferCodingWithQualityHeaderValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Http::Headers::TransferCodingWithQualityHeaderValue =>
    "System.Net.Http.Headers"."TransferCodingWithQualityHeaderValue"
);
#[cfg(feature = "System+Net+Http+Headers+TransferCodingWithQualityHeaderValue")]
impl std::ops::Deref
for crate::System::Net::Http::Headers::TransferCodingWithQualityHeaderValue {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Net::Http::Headers::TransferCodingHeaderValue,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+TransferCodingWithQualityHeaderValue")]
impl std::ops::DerefMut
for crate::System::Net::Http::Headers::TransferCodingWithQualityHeaderValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+TransferCodingWithQualityHeaderValue")]
impl crate::System::Net::Http::Headers::TransferCodingWithQualityHeaderValue {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        minimalCount: i32,
        result: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::Http::Headers::TransferCodingWithQualityHeaderValue,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, minimalCount, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseElement(
        lexer: quest_hook::libil2cpp::Gc<crate::System::Net::Http::Headers::Lexer>,
        parsedValue: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Net::Http::Headers::TransferCodingWithQualityHeaderValue,
            >,
        >,
        t: quest_hook::libil2cpp::ByRefMut<crate::System::Net::Http::Headers::Token>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseElement", (lexer, parsedValue, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+TransferCodingWithQualityHeaderValue")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::TransferCodingWithQualityHeaderValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
