#[cfg(feature = "System+Xml+XmlDocument")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDocument {
    __cordl_parent: crate::System::Xml::XmlNode,
    pub implementation: *mut crate::System::Xml::XmlImplementation,
    pub domNameTable: *mut crate::System::Xml::DomNameTable,
    pub lastChild: *mut crate::System::Xml::XmlLinkedNode,
    pub entities: *mut crate::System::Xml::XmlNamedNodeMap,
    pub htElementIdMap: *mut crate::System::Collections::Hashtable,
    pub htElementIDAttrDecl: *mut crate::System::Collections::Hashtable,
    pub schemaInfo: *mut crate::System::Xml::Schema::SchemaInfo,
    pub schemas: *mut crate::System::Xml::Schema::XmlSchemaSet,
    pub reportValidity: bool,
    pub actualLoadingStatus: bool,
    pub onNodeInsertingDelegate: *mut crate::System::Xml::XmlNodeChangedEventHandler,
    pub onNodeInsertedDelegate: *mut crate::System::Xml::XmlNodeChangedEventHandler,
    pub onNodeRemovingDelegate: *mut crate::System::Xml::XmlNodeChangedEventHandler,
    pub onNodeRemovedDelegate: *mut crate::System::Xml::XmlNodeChangedEventHandler,
    pub onNodeChangingDelegate: *mut crate::System::Xml::XmlNodeChangedEventHandler,
    pub onNodeChangedDelegate: *mut crate::System::Xml::XmlNodeChangedEventHandler,
    pub fEntRefNodesPresent: bool,
    pub fCDataNodesPresent: bool,
    pub preserveWhitespace: bool,
    pub isLoading: bool,
    pub strDocumentName: *mut crate::System::String,
    pub strDocumentFragmentName: *mut crate::System::String,
    pub strCommentName: *mut crate::System::String,
    pub strTextName: *mut crate::System::String,
    pub strCDataSectionName: *mut crate::System::String,
    pub strEntityName: *mut crate::System::String,
    pub strID: *mut crate::System::String,
    pub strXmlns: *mut crate::System::String,
    pub strXml: *mut crate::System::String,
    pub strSpace: *mut crate::System::String,
    pub strLang: *mut crate::System::String,
    pub strEmpty: *mut crate::System::String,
    pub strNonSignificantWhitespaceName: *mut crate::System::String,
    pub strSignificantWhitespaceName: *mut crate::System::String,
    pub strReservedXmlns: *mut crate::System::String,
    pub strReservedXml: *mut crate::System::String,
    pub baseURI: *mut crate::System::String,
    pub resolver: *mut crate::System::Xml::XmlResolver,
    pub bSetResolver: bool,
    pub objLock: *mut crate::System::Object,
}
#[cfg(feature = "System+Xml+XmlDocument")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlDocument => "System.Xml"
    ."XmlDocument"
);
#[cfg(feature = "System+Xml+XmlDocument")]
impl std::ops::Deref for crate::System::Xml::XmlDocument {
    type Target = crate::System::Xml::XmlNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlDocument")]
impl std::ops::DerefMut for crate::System::Xml::XmlDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlDocument")]
impl crate::System::Xml::XmlDocument {
    pub fn AddAttrXmlName(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
        schemaInfo: *mut crate::System::Xml::Schema::IXmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlName = __cordl_object
            .invoke("AddAttrXmlName", (prefix, localName, namespaceURI, schemaInfo))?;
        Ok(__cordl_ret)
    }
    pub fn AddDefaultAttributes(
        &mut self,
        elem: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDefaultAttributes", (elem))?;
        Ok(__cordl_ret)
    }
    pub fn AddElementWithId(
        &mut self,
        id: *mut crate::System::String,
        elem: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddElementWithId", (id, elem))?;
        Ok(__cordl_ret)
    }
    pub fn AddIdInfo(
        &mut self,
        eleName: *mut crate::System::Xml::XmlName,
        attrName: *mut crate::System::Xml::XmlName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AddIdInfo", (eleName, attrName))?;
        Ok(__cordl_ret)
    }
    pub fn AddXmlName(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
        schemaInfo: *mut crate::System::Xml::Schema::IXmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlName = __cordl_object
            .invoke("AddXmlName", (prefix, localName, namespaceURI, schemaInfo))?;
        Ok(__cordl_ret)
    }
    pub fn AfterEvent(
        &mut self,
        args: *mut crate::System::Xml::XmlNodeChangedEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AfterEvent", (args))?;
        Ok(__cordl_ret)
    }
    pub fn AppendChildForLoad(
        &mut self,
        newChild: *mut crate::System::Xml::XmlNode,
        doc: *mut crate::System::Xml::XmlDocument,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("AppendChildForLoad", (newChild, doc))?;
        Ok(__cordl_ret)
    }
    pub fn BeforeEvent(
        &mut self,
        args: *mut crate::System::Xml::XmlNodeChangedEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeforeEvent", (args))?;
        Ok(__cordl_ret)
    }
    pub fn CanInsertAfter(
        &mut self,
        newChild: *mut crate::System::Xml::XmlNode,
        refChild: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanInsertAfter", (newChild, refChild))?;
        Ok(__cordl_ret)
    }
    pub fn CanInsertBefore(
        &mut self,
        newChild: *mut crate::System::Xml::XmlNode,
        refChild: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanInsertBefore", (newChild, refChild))?;
        Ok(__cordl_ret)
    }
    pub fn CloneNode(
        &mut self,
        deep: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("CloneNode", (deep))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAttribute_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("CreateAttribute", (name))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAttribute_String1(
        &mut self,
        qualifiedName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("CreateAttribute", (qualifiedName, namespaceURI))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAttribute_String_String2(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("CreateAttribute", (prefix, localName, namespaceURI))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCDataSection(
        &mut self,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlCDataSection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlCDataSection = __cordl_object
            .invoke("CreateCDataSection", (data))?;
        Ok(__cordl_ret)
    }
    pub fn CreateComment(
        &mut self,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlComment> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlComment = __cordl_object
            .invoke("CreateComment", (data))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDefaultAttribute(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("CreateDefaultAttribute", (prefix, localName, namespaceURI))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDocumentFragment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDocumentFragment> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDocumentFragment = __cordl_object
            .invoke("CreateDocumentFragment", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateDocumentType(
        &mut self,
        name: *mut crate::System::String,
        publicId: *mut crate::System::String,
        systemId: *mut crate::System::String,
        internalSubset: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDocumentType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDocumentType = __cordl_object
            .invoke("CreateDocumentType", (name, publicId, systemId, internalSubset))?;
        Ok(__cordl_ret)
    }
    pub fn CreateElement_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlElement = __cordl_object
            .invoke("CreateElement", (name))?;
        Ok(__cordl_ret)
    }
    pub fn CreateElement_String1(
        &mut self,
        qualifiedName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlElement = __cordl_object
            .invoke("CreateElement", (qualifiedName, namespaceURI))?;
        Ok(__cordl_ret)
    }
    pub fn CreateElement_String_String2(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlElement = __cordl_object
            .invoke("CreateElement", (prefix, localName, namespaceURI))?;
        Ok(__cordl_ret)
    }
    pub fn CreateEntityReference(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlEntityReference> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlEntityReference = __cordl_object
            .invoke("CreateEntityReference", (name))?;
        Ok(__cordl_ret)
    }
    pub fn CreateProcessingInstruction(
        &mut self,
        target: *mut crate::System::String,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlProcessingInstruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlProcessingInstruction = __cordl_object
            .invoke("CreateProcessingInstruction", (target, data))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSignificantWhitespace(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlSignificantWhitespace,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlSignificantWhitespace = __cordl_object
            .invoke("CreateSignificantWhitespace", (text))?;
        Ok(__cordl_ret)
    }
    pub fn CreateTextNode(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlText> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlText = __cordl_object
            .invoke("CreateTextNode", (text))?;
        Ok(__cordl_ret)
    }
    pub fn CreateWhitespace(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlWhitespace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlWhitespace = __cordl_object
            .invoke("CreateWhitespace", (text))?;
        Ok(__cordl_ret)
    }
    pub fn CreateXmlDeclaration(
        &mut self,
        version: *mut crate::System::String,
        encoding: *mut crate::System::String,
        standalone: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDeclaration> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDeclaration = __cordl_object
            .invoke("CreateXmlDeclaration", (version, encoding, standalone))?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultAttribute(
        &mut self,
        elem: *mut crate::System::Xml::XmlElement,
        attrPrefix: *mut crate::System::String,
        attrLocalname: *mut crate::System::String,
        attrNamespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke(
                "GetDefaultAttribute",
                (elem, attrPrefix, attrLocalname, attrNamespaceURI),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetElement(
        &mut self,
        elementList: *mut crate::System::Collections::ArrayList,
        elem: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::WeakReference> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::WeakReference = __cordl_object
            .invoke("GetElement", (elementList, elem))?;
        Ok(__cordl_ret)
    }
    pub fn GetEntityNode(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlEntity> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlEntity = __cordl_object
            .invoke("GetEntityNode", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetEventArgs(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        oldParent: *mut crate::System::Xml::XmlNode,
        newParent: *mut crate::System::Xml::XmlNode,
        oldValue: *mut crate::System::String,
        newValue: *mut crate::System::String,
        action: crate::System::Xml::XmlNodeChangedAction,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlNodeChangedEventArgs,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNodeChangedEventArgs = __cordl_object
            .invoke(
                "GetEventArgs",
                (node, oldParent, newParent, oldValue, newValue, action),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetIDInfoByElement(
        &mut self,
        eleName: *mut crate::System::Xml::XmlName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlName = __cordl_object
            .invoke("GetIDInfoByElement", (eleName))?;
        Ok(__cordl_ret)
    }
    pub fn GetIDInfoByElement_(
        &mut self,
        eleName: *mut crate::System::Xml::XmlName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlName = __cordl_object
            .invoke("GetIDInfoByElement_", (eleName))?;
        Ok(__cordl_ret)
    }
    pub fn GetInsertEventArgsForLoad(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        newParent: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlNodeChangedEventArgs,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNodeChangedEventArgs = __cordl_object
            .invoke("GetInsertEventArgsForLoad", (node, newParent))?;
        Ok(__cordl_ret)
    }
    pub fn GetResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlResolver> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlResolver = __cordl_object
            .invoke("GetResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSchemaElementDecl(
        &mut self,
        elem: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaElementDecl = __cordl_object
            .invoke("GetSchemaElementDecl", (elem))?;
        Ok(__cordl_ret)
    }
    pub fn GetXmlName(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
        schemaInfo: *mut crate::System::Xml::Schema::IXmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlName = __cordl_object
            .invoke("GetXmlName", (prefix, localName, namespaceURI, schemaInfo))?;
        Ok(__cordl_ret)
    }
    pub fn HasNodeTypeInNextSiblings(
        &mut self,
        nt: crate::System::Xml::XmlNodeType,
        refNode: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasNodeTypeInNextSiblings", (nt, refNode))?;
        Ok(__cordl_ret)
    }
    pub fn HasNodeTypeInPrevSiblings(
        &mut self,
        nt: crate::System::Xml::XmlNodeType,
        refNode: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasNodeTypeInPrevSiblings", (nt, refNode))?;
        Ok(__cordl_ret)
    }
    pub fn ImportAttributes(
        &mut self,
        fromElem: *mut crate::System::Xml::XmlNode,
        toElem: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImportAttributes", (fromElem, toElem))?;
        Ok(__cordl_ret)
    }
    pub fn ImportChildren(
        &mut self,
        fromNode: *mut crate::System::Xml::XmlNode,
        toNode: *mut crate::System::Xml::XmlNode,
        deep: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImportChildren", (fromNode, toNode, deep))?;
        Ok(__cordl_ret)
    }
    pub fn ImportNodeInternal(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        deep: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("ImportNodeInternal", (node, deep))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidChildType(
        &mut self,
        _cordl_type: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidChildType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn Load(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Load", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn LoadXml(
        &mut self,
        xml: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadXml", (xml))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_XmlImplementation2(
        imp: *mut crate::System::Xml::XmlImplementation,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (imp))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlNameTable1(
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nt))?;
        Ok(__cordl_object)
    }
    pub fn PrepareDefaultAttribute(
        &mut self,
        attdef: *mut crate::System::Xml::Schema::SchemaAttDef,
        attrPrefix: *mut crate::System::String,
        attrLocalname: *mut crate::System::String,
        attrNamespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke(
                "PrepareDefaultAttribute",
                (attdef, attrPrefix, attrLocalname, attrNamespaceURI),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReadNode(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("ReadNode", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveElementWithId(
        &mut self,
        id: *mut crate::System::String,
        elem: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveElementWithId", (id, elem))?;
        Ok(__cordl_ret)
    }
    pub fn Save(
        &mut self,
        w: *mut crate::System::Xml::XmlWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (w))?;
        Ok(__cordl_ret)
    }
    pub fn SetBaseURI(
        &mut self,
        inBaseURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBaseURI", (inBaseURI))?;
        Ok(__cordl_ret)
    }
    pub fn SetDefaultNamespace(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        namespaceURI: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultNamespace", (prefix, localName, namespaceURI))?;
        Ok(__cordl_ret)
    }
    pub fn SetupReader(
        &mut self,
        tr: *mut crate::System::Xml::XmlTextReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlTextReader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlTextReader = __cordl_object
            .invoke("SetupReader", (tr))?;
        Ok(__cordl_ret)
    }
    pub fn WriteContentTo(
        &mut self,
        xw: *mut crate::System::Xml::XmlWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteContentTo", (xw))?;
        Ok(__cordl_ret)
    }
    pub fn WriteTo(
        &mut self,
        w: *mut crate::System::Xml::XmlWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (w))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlImplementation2(
        &mut self,
        imp: *mut crate::System::Xml::XmlImplementation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (imp))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlNameTable1(
        &mut self,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nt))?;
        Ok(__cordl_ret)
    }
    pub fn get_ActualLoadingStatus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ActualLoadingStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_BaseURI", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CanReportValidity(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanReportValidity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Declaration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDeclaration> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDeclaration = __cordl_object
            .invoke("get_Declaration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DocumentElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlElement = __cordl_object
            .invoke("get_DocumentElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DocumentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDocumentType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDocumentType = __cordl_object
            .invoke("get_DocumentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdSchemaInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaInfo = __cordl_object
            .invoke("get_DtdSchemaInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Encoding", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Entities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNamedNodeMap> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNamedNodeMap = __cordl_object
            .invoke("get_Entities", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasSetResolver(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasSetResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Implementation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlImplementation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlImplementation = __cordl_object
            .invoke("get_Implementation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsContainer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsLoading(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsLoading", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LastNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlLinkedNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlLinkedNode = __cordl_object
            .invoke("get_LastNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LocalName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNameTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNameTable = __cordl_object
            .invoke("get_NameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OwnerDocument(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDocument> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDocument = __cordl_object
            .invoke("get_OwnerDocument", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("get_ParentNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::IXmlSchemaInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::IXmlSchemaInfo = __cordl_object
            .invoke("get_SchemaInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Standalone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Standalone", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_DtdSchemaInfo(
        &mut self,
        value: *mut crate::System::Xml::Schema::SchemaInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DtdSchemaInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Entities(
        &mut self,
        value: *mut crate::System::Xml::XmlNamedNodeMap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Entities", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_InnerText(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InnerText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_InnerXml(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InnerXml", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsLoading(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsLoading", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_LastNode(
        &mut self,
        value: *mut crate::System::Xml::XmlLinkedNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LastNode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Schemas(
        &mut self,
        value: *mut crate::System::Xml::Schema::XmlSchemaSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Schemas", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_XmlResolver(
        &mut self,
        value: *mut crate::System::Xml::XmlResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_XmlResolver", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlDocument")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlDocument {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
