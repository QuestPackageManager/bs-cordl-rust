#[cfg(feature = "System+Xml+XmlLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlLoader {
    __cordl_parent: crate::System::Object,
    pub doc: *mut crate::System::Xml::XmlDocument,
    pub reader: *mut crate::System::Xml::XmlReader,
    pub preserveWhitespace: bool,
}
#[cfg(feature = "System+Xml+XmlLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlLoader => "System.Xml"
    ."XmlLoader"
);
#[cfg(feature = "System+Xml+XmlLoader")]
impl std::ops::Deref for crate::System::Xml::XmlLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlLoader")]
impl std::ops::DerefMut for crate::System::Xml::XmlLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlLoader")]
impl crate::System::Xml::XmlLoader {
    pub fn LoadAttributeNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("LoadAttributeNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadAttributeNodeDirect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("LoadAttributeNodeDirect", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseDocumentType_XmlDocumentType0(
        &mut self,
        dtNode: *mut crate::System::Xml::XmlDocumentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseDocumentType", (dtNode))?;
        Ok(__cordl_ret)
    }
    pub fn ParseDocumentType__cordl_bool_XmlResolver1(
        &mut self,
        dtNode: *mut crate::System::Xml::XmlDocumentType,
        bUseResolver: bool,
        resolver: *mut crate::System::Xml::XmlResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseDocumentType", (dtNode, bUseResolver, resolver))?;
        Ok(__cordl_ret)
    }
    pub fn GetContext(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlParserContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlParserContext = __cordl_object
            .invoke("GetContext", (node))?;
        Ok(__cordl_ret)
    }
    pub fn ParsePartialContent(
        &mut self,
        parentNode: *mut crate::System::Xml::XmlNode,
        innerxmltext: *mut crate::System::String,
        nt: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNamespaceManager> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNamespaceManager = __cordl_object
            .invoke("ParsePartialContent", (parentNode, innerxmltext, nt))?;
        Ok(__cordl_ret)
    }
    pub fn LoadDeclarationNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDeclaration> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDeclaration = __cordl_object
            .invoke("LoadDeclarationNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn Load(
        &mut self,
        doc: *mut crate::System::Xml::XmlDocument,
        reader: *mut crate::System::Xml::XmlReader,
        preserveWhitespace: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Load", (doc, reader, preserveWhitespace))?;
        Ok(__cordl_ret)
    }
    pub fn LoadInnerXmlAttribute(
        &mut self,
        node: *mut crate::System::Xml::XmlAttribute,
        innerxmltext: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadInnerXmlAttribute", (node, innerxmltext))?;
        Ok(__cordl_ret)
    }
    pub fn ExpandEntityReference(
        &mut self,
        eref: *mut crate::System::Xml::XmlEntityReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandEntityReference", (eref))?;
        Ok(__cordl_ret)
    }
    pub fn LoadDefaultAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("LoadDefaultAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadDocumentType(
        &mut self,
        dtdInfo: *mut crate::System::Xml::IDtdInfo,
        dtNode: *mut crate::System::Xml::XmlDocumentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadDocumentType", (dtdInfo, dtNode))?;
        Ok(__cordl_ret)
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
    pub fn LoadNodeDirect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("LoadNodeDirect", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExpandEntity(
        &mut self,
        ent: *mut crate::System::Xml::XmlEntity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandEntity", (ent))?;
        Ok(__cordl_ret)
    }
    pub fn CreateInnerXmlReader(
        &mut self,
        xmlFragment: *mut crate::System::String,
        nt: crate::System::Xml::XmlNodeType,
        context: *mut crate::System::Xml::XmlParserContext,
        doc: *mut crate::System::Xml::XmlDocument,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlReader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlReader = __cordl_object
            .invoke("CreateInnerXmlReader", (xmlFragment, nt, context, doc))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCurrentNode(
        &mut self,
        doc: *mut crate::System::Xml::XmlDocument,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("ReadCurrentNode", (doc, reader))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAttributeValue(
        &mut self,
        parent: *mut crate::System::Xml::XmlNode,
        direct: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadAttributeValue", (parent, direct))?;
        Ok(__cordl_ret)
    }
    pub fn LoadInnerXmlElement(
        &mut self,
        node: *mut crate::System::Xml::XmlElement,
        innerxmltext: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadInnerXmlElement", (node, innerxmltext))?;
        Ok(__cordl_ret)
    }
    pub fn EntitizeName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("EntitizeName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn LoadEntityReferenceNode(
        &mut self,
        direct: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlEntityReference> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlEntityReference = __cordl_object
            .invoke("LoadEntityReferenceNode", (direct))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveDuplicateNamespace(
        &mut self,
        elem: *mut crate::System::Xml::XmlElement,
        mgr: *mut crate::System::Xml::XmlNamespaceManager,
        fCheckElemAttrs: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDuplicateNamespace", (elem, mgr, fCheckElemAttrs))?;
        Ok(__cordl_ret)
    }
    pub fn LoadNode(
        &mut self,
        skipOverWhitespace: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("LoadNode", (skipOverWhitespace))?;
        Ok(__cordl_ret)
    }
    pub fn LoadDocumentTypeNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDocumentType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDocumentType = __cordl_object
            .invoke("LoadDocumentTypeNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadDocSequence(
        &mut self,
        parentDoc: *mut crate::System::Xml::XmlDocument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadDocSequence", (parentDoc))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
