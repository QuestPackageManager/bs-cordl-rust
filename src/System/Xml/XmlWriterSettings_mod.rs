#[cfg(feature = "System+Xml+XmlWriterSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlWriterSettings {
    __cordl_parent: crate::System::Object,
    pub useAsync: bool,
    pub encoding: *mut crate::System::Text::Encoding,
    pub omitXmlDecl: bool,
    pub newLineHandling: crate::System::Xml::NewLineHandling,
    pub newLineChars: *mut crate::System::String,
    pub indent: crate::System::Xml::TriState,
    pub indentChars: *mut crate::System::String,
    pub newLineOnAttributes: bool,
    pub closeOutput: bool,
    pub namespaceHandling: crate::System::Xml::NamespaceHandling,
    pub conformanceLevel: crate::System::Xml::ConformanceLevel,
    pub checkCharacters: bool,
    pub writeEndDocumentOnClose: bool,
    pub outputMethod: crate::System::Xml::XmlOutputMethod,
    pub cdataSections: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Xml::XmlQualifiedName,
    >,
    pub doNotEscapeUriAttributes: bool,
    pub mergeCDataSections: bool,
    pub mediaType: *mut crate::System::String,
    pub docTypeSystem: *mut crate::System::String,
    pub docTypePublic: *mut crate::System::String,
    pub standalone: crate::System::Xml::XmlStandalone,
    pub autoXmlDecl: bool,
    pub isReadOnly: bool,
}
#[cfg(feature = "System+Xml+XmlWriterSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlWriterSettings => "System.Xml"
    ."XmlWriterSettings"
);
#[cfg(feature = "System+Xml+XmlWriterSettings")]
impl std::ops::Deref for crate::System::Xml::XmlWriterSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWriterSettings")]
impl std::ops::DerefMut for crate::System::Xml::XmlWriterSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlWriterSettings")]
impl crate::System::Xml::XmlWriterSettings {
    pub fn CheckReadOnly(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckReadOnly", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlWriterSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlWriterSettings = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateWriter_Stream0(
        &mut self,
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlWriter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlWriter = __cordl_object
            .invoke("CreateWriter", (output))?;
        Ok(__cordl_ret)
    }
    pub fn CreateWriter_TextWriter1(
        &mut self,
        output: *mut crate::System::IO::TextWriter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlWriter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlWriter = __cordl_object
            .invoke("CreateWriter", (output))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn get_Async(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Async", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AutoXmlDeclaration(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AutoXmlDeclaration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CDataSectionElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Xml::XmlQualifiedName,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Xml::XmlQualifiedName,
        > = __cordl_object.invoke("get_CDataSectionElements", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CheckCharacters(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CheckCharacters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CloseOutput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CloseOutput", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConformanceLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::ConformanceLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::ConformanceLevel = __cordl_object
            .invoke("get_ConformanceLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DoNotEscapeUriAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_DoNotEscapeUriAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DocTypePublic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DocTypePublic", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DocTypeSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DocTypeSystem", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("get_Encoding", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Indent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Indent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IndentChars(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_IndentChars", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IndentInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::TriState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::TriState = __cordl_object
            .invoke("get_IndentInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsQuerySpecific(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsQuerySpecific", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MediaType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_MediaType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MergeCDataSections(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_MergeCDataSections", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::NamespaceHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::NamespaceHandling = __cordl_object
            .invoke("get_NamespaceHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NewLineChars(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_NewLineChars", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NewLineHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::NewLineHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::NewLineHandling = __cordl_object
            .invoke("get_NewLineHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NewLineOnAttributes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_NewLineOnAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OmitXmlDeclaration(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_OmitXmlDeclaration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OutputMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlOutputMethod> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlOutputMethod = __cordl_object
            .invoke("get_OutputMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Standalone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlStandalone> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlStandalone = __cordl_object
            .invoke("get_Standalone", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WriteEndDocumentOnClose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_WriteEndDocumentOnClose", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_Indent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Indent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_NamespaceHandling(
        &mut self,
        value: crate::System::Xml::NamespaceHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NamespaceHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_OmitXmlDeclaration(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_OmitXmlDeclaration", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_OutputMethod(
        &mut self,
        value: crate::System::Xml::XmlOutputMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_OutputMethod", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlWriterSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlWriterSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}