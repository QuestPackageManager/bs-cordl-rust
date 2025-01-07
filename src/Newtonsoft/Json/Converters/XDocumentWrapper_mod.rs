#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XDocumentWrapper {
    __cordl_parent: crate::Newtonsoft::Json::Converters::XContainerWrapper,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Converters::XDocumentWrapper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Converters";
    const CLASS_NAME: &'static str = "XDocumentWrapper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XDocumentWrapper {
    type Target = crate::Newtonsoft::Json::Converters::XContainerWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XDocumentWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
impl crate::Newtonsoft::Json::Converters::XDocumentWrapper {
    pub fn AppendChild(
        &mut self,
        newChild: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("AppendChild", (newChild))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAttribute_Il2CppString1(
        &mut self,
        qualifiedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object
            .invoke("CreateAttribute", (qualifiedName, namespaceUri, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAttribute_Il2CppString_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("CreateAttribute", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCDataSection(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("CreateCDataSection", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateComment(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("CreateComment", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateElement_Il2CppString0(
        &mut self,
        elementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlElement,
        > = __cordl_object.invoke("CreateElement", (elementName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateElement_Il2CppString1(
        &mut self,
        qualifiedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlElement,
        > = __cordl_object.invoke("CreateElement", (qualifiedName, namespaceUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateProcessingInstruction(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("CreateProcessingInstruction", (target, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSignificantWhitespace(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("CreateSignificantWhitespace", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTextNode(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("CreateTextNode", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateWhitespace(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("CreateWhitespace", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateXmlDeclaration(
        &mut self,
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        standalone: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object
            .invoke("CreateXmlDeclaration", (version, encoding, standalone))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateXmlDocumentType(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        publicId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        systemId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        internalSubset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object
            .invoke(
                "CreateXmlDocumentType",
                (name, publicId, systemId, internalSubset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        document: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (document))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        document: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (document))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChildNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
            >,
        > = __cordl_object.invoke("get_ChildNodes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Document(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDocument>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDocument,
        > = __cordl_object.invoke("get_Document", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DocumentElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlElement,
        > = __cordl_object.invoke("get_DocumentElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasChildNodes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasChildNodes", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XDocumentWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
impl AsRef<crate::Newtonsoft::Json::Converters::IXmlDocument>
for crate::Newtonsoft::Json::Converters::XDocumentWrapper {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Converters::IXmlDocument {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
impl AsMut<crate::Newtonsoft::Json::Converters::IXmlDocument>
for crate::Newtonsoft::Json::Converters::XDocumentWrapper {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::Converters::IXmlDocument {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
impl AsRef<crate::Newtonsoft::Json::Converters::IXmlNode>
for crate::Newtonsoft::Json::Converters::XDocumentWrapper {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Converters::IXmlNode {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDocumentWrapper")]
impl AsMut<crate::Newtonsoft::Json::Converters::IXmlNode>
for crate::Newtonsoft::Json::Converters::XDocumentWrapper {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::Converters::IXmlNode {
        unsafe { std::mem::transmute(self) }
    }
}
