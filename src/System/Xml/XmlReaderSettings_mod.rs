#[cfg(feature = "System+Xml+XmlReaderSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlReaderSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub useAsync: bool,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub xmlResolver: *mut crate::System::Xml::XmlResolver,
    pub lineNumberOffset: i32,
    pub linePositionOffset: i32,
    pub conformanceLevel: crate::System::Xml::ConformanceLevel,
    pub checkCharacters: bool,
    pub maxCharactersInDocument: i64,
    pub maxCharactersFromEntities: i64,
    pub ignoreWhitespace: bool,
    pub ignorePIs: bool,
    pub ignoreComments: bool,
    pub dtdProcessing: crate::System::Xml::DtdProcessing,
    pub validationType: crate::System::Xml::ValidationType,
    pub validationFlags: crate::System::Xml::Schema::XmlSchemaValidationFlags,
    pub schemas: *mut crate::System::Xml::Schema::XmlSchemaSet,
    pub valEventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    pub closeInput: bool,
    pub isReadOnly: bool,
    pub _IsXmlResolverSet_k__BackingField: bool,
}
#[cfg(feature = "System+Xml+XmlReaderSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlReaderSettings => "System.Xml"
    ."XmlReaderSettings"
);
#[cfg(feature = "System+Xml+XmlReaderSettings")]
impl std::ops::Deref for crate::System::Xml::XmlReaderSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlReaderSettings")]
impl std::ops::DerefMut for crate::System::Xml::XmlReaderSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlReaderSettings")]
impl crate::System::Xml::XmlReaderSettings {
    pub fn AddValidation(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader> = __cordl_object
            .invoke("AddValidation", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckReadOnly(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckReadOnly", (propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReaderSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlReaderSettings,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultResolver() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDefaultResolver", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDtdValidatingReader(
        &mut self,
        baseReader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlValidatingReaderImpl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlValidatingReaderImpl,
        > = __cordl_object.invoke("CreateDtdValidatingReader", (baseReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateReader_Stream_Uri_Il2CppString_XmlParserContext0(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        baseUriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inputContext: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlParserContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader> = __cordl_object
            .invoke("CreateReader", (input, baseUri, baseUriString, inputContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateReader_TextReader_Il2CppString_XmlParserContext1(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
        baseUriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        inputContext: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlParserContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader> = __cordl_object
            .invoke("CreateReader", (input, baseUriString, inputContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableLegacyXmlSettings() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableLegacyXmlSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEventHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationEventHandler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        > = __cordl_object.invoke("GetEventHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXmlResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver> = __cordl_object
            .invoke("GetXmlResolver", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXmlResolver_CheckConfig(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver> = __cordl_object
            .invoke("GetXmlResolver_CheckConfig", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_XmlResolver1(
        &mut self,
        resolver: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (resolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_Async(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Async", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CheckCharacters(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CheckCharacters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CloseInput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CloseInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConformanceLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::ConformanceLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::ConformanceLevel = __cordl_object
            .invoke("get_ConformanceLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DtdProcessing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdProcessing> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdProcessing = __cordl_object
            .invoke("get_DtdProcessing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreComments(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IgnoreComments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreProcessingInstructions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IgnoreProcessingInstructions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreWhitespace(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IgnoreWhitespace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsXmlResolverSet(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsXmlResolverSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LineNumberOffset(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumberOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LinePositionOffset(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePositionOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxCharactersFromEntities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_MaxCharactersFromEntities", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxCharactersInDocument(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_MaxCharactersInDocument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable> = __cordl_object
            .invoke("get_NameTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Schemas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSet,
        > = __cordl_object.invoke("get_Schemas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValidationFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaValidationFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaValidationFlags = __cordl_object
            .invoke("get_ValidationFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValidationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::ValidationType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::ValidationType = __cordl_object
            .invoke("get_ValidationType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Async(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Async", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CheckCharacters(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CheckCharacters", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CloseInput(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CloseInput", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ConformanceLevel(
        &mut self,
        value: crate::System::Xml::ConformanceLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ConformanceLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DtdProcessing(
        &mut self,
        value: crate::System::Xml::DtdProcessing,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DtdProcessing", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IgnoreComments(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreComments", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IgnoreProcessingInstructions(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreProcessingInstructions", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IgnoreWhitespace(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreWhitespace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsXmlResolverSet(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsXmlResolverSet", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LineNumberOffset(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LineNumberOffset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LinePositionOffset(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LinePositionOffset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxCharactersFromEntities(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaxCharactersFromEntities", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxCharactersInDocument(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaxCharactersInDocument", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_NameTable(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NameTable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ReadOnly(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReadOnly", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Schemas(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Schemas", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ValidationFlags(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaValidationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ValidationFlags", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ValidationType(
        &mut self,
        value: crate::System::Xml::ValidationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ValidationType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_XmlResolver(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_XmlResolver", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlReaderSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlReaderSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
