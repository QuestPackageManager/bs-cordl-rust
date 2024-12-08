#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDocumentWrapper {
    __cordl_parent: crate::Newtonsoft::Json::Converters::XmlNodeWrapper,
    pub _document: *mut crate::System::Xml::XmlDocument,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Converters::XmlDocumentWrapper
    => "Newtonsoft.Json.Converters"."XmlDocumentWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XmlDocumentWrapper {
    type Target = crate::Newtonsoft::Json::Converters::XmlNodeWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentWrapper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XmlDocumentWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentWrapper")]
impl crate::Newtonsoft::Json::Converters::XmlDocumentWrapper {
    pub fn CreateAttribute_String1(
        &mut self,
        qualifiedName: *mut crate::System::String,
        namespaceUri: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("CreateAttribute", (qualifiedName, namespaceUri, value))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAttribute_String_String0(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("CreateAttribute", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCDataSection(
        &mut self,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("CreateCDataSection", (data))?;
        Ok(__cordl_ret)
    }
    pub fn CreateComment(
        &mut self,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("CreateComment", (data))?;
        Ok(__cordl_ret)
    }
    pub fn CreateElement_String0(
        &mut self,
        elementName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlElement = __cordl_object
            .invoke("CreateElement", (elementName))?;
        Ok(__cordl_ret)
    }
    pub fn CreateElement_String1(
        &mut self,
        qualifiedName: *mut crate::System::String,
        namespaceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlElement = __cordl_object
            .invoke("CreateElement", (qualifiedName, namespaceUri))?;
        Ok(__cordl_ret)
    }
    pub fn CreateProcessingInstruction(
        &mut self,
        target: *mut crate::System::String,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("CreateProcessingInstruction", (target, data))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSignificantWhitespace(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("CreateSignificantWhitespace", (text))?;
        Ok(__cordl_ret)
    }
    pub fn CreateTextNode(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("CreateTextNode", (text))?;
        Ok(__cordl_ret)
    }
    pub fn CreateWhitespace(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("CreateWhitespace", (text))?;
        Ok(__cordl_ret)
    }
    pub fn CreateXmlDeclaration(
        &mut self,
        version: *mut crate::System::String,
        encoding: *mut crate::System::String,
        standalone: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("CreateXmlDeclaration", (version, encoding, standalone))?;
        Ok(__cordl_ret)
    }
    pub fn CreateXmlDocumentType(
        &mut self,
        name: *mut crate::System::String,
        publicId: *mut crate::System::String,
        systemId: *mut crate::System::String,
        internalSubset: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke(
                "CreateXmlDocumentType",
                (name, publicId, systemId, internalSubset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        document: *mut crate::System::Xml::XmlDocument,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (document))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        document: *mut crate::System::Xml::XmlDocument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (document))?;
        Ok(__cordl_ret)
    }
    pub fn get_DocumentElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlElement = __cordl_object
            .invoke("get_DocumentElement", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlDocumentWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XmlDocumentWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
