#[cfg(feature = "System+Xml+XmlLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub doc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    pub reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    pub preserveWhitespace: bool,
}
#[cfg(feature = "System+Xml+XmlLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlLoader => "System.Xml"
    ."XmlLoader"
);
#[cfg(feature = "System+Xml+XmlLoader")]
impl std::ops::Deref for crate::System::Xml::XmlLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CreateInnerXmlReader(
        &mut self,
        xmlFragment: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nt: crate::System::Xml::XmlNodeType,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlParserContext>,
        doc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader> = __cordl_object
            .invoke("CreateInnerXmlReader", (xmlFragment, nt, context, doc))?;
        Ok(__cordl_ret.into())
    }
    pub fn EntitizeName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("EntitizeName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandEntity(
        &mut self,
        ent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlEntity>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandEntity", (ent))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandEntityReference(
        &mut self,
        eref: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlEntityReference>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandEntityReference", (eref))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContext(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlParserContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlParserContext,
        > = __cordl_object.invoke("GetContext", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load(
        &mut self,
        doc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        preserveWhitespace: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Load", (doc, reader, preserveWhitespace))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAttributeNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("LoadAttributeNode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAttributeNodeDirect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("LoadAttributeNodeDirect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAttributeValue(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        direct: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadAttributeValue", (parent, direct))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadDeclarationNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDeclaration>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDeclaration> = __cordl_object
            .invoke("LoadDeclarationNode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadDefaultAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("LoadDefaultAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadDocSequence(
        &mut self,
        parentDoc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadDocSequence", (parentDoc))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadDocumentType(
        &mut self,
        dtdInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdInfo>,
        dtNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocumentType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadDocumentType", (dtdInfo, dtNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadDocumentTypeNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocumentType>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlDocumentType,
        > = __cordl_object.invoke("LoadDocumentTypeNode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadEntityReferenceNode(
        &mut self,
        direct: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlEntityReference>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlEntityReference,
        > = __cordl_object.invoke("LoadEntityReferenceNode", (direct))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadInnerXmlAttribute(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
        innerxmltext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadInnerXmlAttribute", (node, innerxmltext))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadInnerXmlElement(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        innerxmltext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadInnerXmlElement", (node, innerxmltext))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadNode(
        &mut self,
        skipOverWhitespace: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("LoadNode", (skipOverWhitespace))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadNodeDirect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("LoadNodeDirect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParseDocumentType_XmlDocumentType0(
        &mut self,
        dtNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocumentType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseDocumentType", (dtNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDocumentType__cordl_bool_XmlResolver1(
        &mut self,
        dtNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocumentType>,
        bUseResolver: bool,
        resolver: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseDocumentType", (dtNode, bUseResolver, resolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParsePartialContent(
        &mut self,
        parentNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        innerxmltext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nt: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlNamespaceManager,
        > = __cordl_object
            .invoke("ParsePartialContent", (parentNode, innerxmltext, nt))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseXmlDeclarationValue(
        strValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        version: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        encoding: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        standalone: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ParseXmlDeclarationValue",
                (strValue, version, encoding, standalone),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCurrentNode(
        &mut self,
        doc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("ReadCurrentNode", (doc, reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDuplicateNamespace(
        &mut self,
        elem: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        mgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        fCheckElemAttrs: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDuplicateNamespace", (elem, mgr, fCheckElemAttrs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnexpectedNodeType(
        nodetype: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnexpectedNodeType", (nodetype))?;
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
#[cfg(feature = "System+Xml+XmlLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
