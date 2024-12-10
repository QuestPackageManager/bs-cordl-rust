#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNodeConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter,
    pub _DeserializeRootElementName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _WriteArrayAttribute_k__BackingField: bool,
    pub _OmitRootObject_k__BackingField: bool,
    pub _EncodeSpecialCharacters_k__BackingField: bool,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Converters::XmlNodeConverter
    => "Newtonsoft.Json.Converters"."XmlNodeConverter"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeConverter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XmlNodeConverter {
    type Target = crate::Newtonsoft::Json::JsonConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeConverter")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XmlNodeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeConverter")]
impl crate::Newtonsoft::Json::Converters::XmlNodeConverter {
    pub const CDataName: &'static str = "#cdata-section";
    pub const CommentName: &'static str = "#comment";
    pub const DeclarationName: &'static str = "?xml";
    pub const JsonNamespaceUri: &'static str = "http://james.newtonking.com/projects/json";
    pub const SignificantWhitespaceName: &'static str = "#significant-whitespace";
    pub const TextName: &'static str = "#text";
    pub const WhitespaceName: &'static str = "#whitespace";
    pub fn AddJsonArrayAttribute(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlElement,
        >,
        document: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlDocument,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddJsonArrayAttribute", (element, document))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanConvert(
        &mut self,
        valueType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanConvert", (valueType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDocumentType(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        document: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlDocument,
        >,
        currentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateDocumentType", (reader, document, currentNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateElement_Il2CppString_Il2CppString_XmlNamespaceManager1(
        &mut self,
        elementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        document: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlDocument,
        >,
        elementPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlElement,
        > = __cordl_object
            .invoke("CreateElement", (elementName, document, elementPrefix, manager))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateElement_JsonReader_IXmlNode_Il2CppString_XmlNamespaceManager_Il2CppString_Dictionary_2_0(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        document: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlDocument,
        >,
        currentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
        elementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        elementPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeNameValues: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CreateElement",
                (
                    reader,
                    document,
                    currentNode,
                    elementName,
                    manager,
                    elementPrefix,
                    attributeNameValues,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstruction(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        document: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlDocument,
        >,
        currentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateInstruction", (reader, document, currentNode, propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeNode(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        document: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlDocument,
        >,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        currentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeNode", (reader, document, manager, currentNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeValue(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        document: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlDocument,
        >,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        currentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DeserializeValue",
                (reader, document, manager, propertyName, currentNode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyName(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPropertyName", (node, manager))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsArray(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsArray", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNamespaceAttribute(
        &mut self,
        attributeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prefix: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsNamespaceAttribute", (attributeName, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsXObject(
        &mut self,
        valueType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXObject", (valueType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsXmlNode(
        &mut self,
        valueType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXmlNode", (valueType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PushParentNamespaces(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushParentNamespaces", (node, manager))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadArrayElements(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        document: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlDocument,
        >,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        currentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReadArrayElements",
                (reader, document, propertyName, currentNode, manager),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAttributeElements(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("ReadAttributeElements", (reader, manager))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadElement(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        document: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlDocument,
        >,
        currentNode: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReadElement",
                (reader, document, currentNode, propertyName, manager),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadJson(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        existingValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("ReadJson", (reader, objectType, existingValue, serializer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveFullName(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ResolveFullName", (node, manager))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeGroupedNodes(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        node: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        writePropertyName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeGroupedNodes",
                (writer, node, manager, writePropertyName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeNode(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        node: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        writePropertyName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeNode", (writer, node, manager, writePropertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldReadInto(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldReadInto", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValueAttributes(
        &mut self,
        c: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Converters::IXmlNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValueAttributes", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn WrapXml(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("WrapXml", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteGroupedNodes_IXmlNode1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        writePropertyName: bool,
        node: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
        elementNames: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteGroupedNodes",
                (writer, manager, writePropertyName, node, elementNames),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteGroupedNodes_List_1_0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        manager: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        writePropertyName: bool,
        groupedNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Converters::IXmlNode,
            >,
        >,
        elementNames: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteGroupedNodes",
                (writer, manager, writePropertyName, groupedNodes, elementNames),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteJson(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteJson", (writer, value, serializer))?;
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
    pub fn get_DeserializeRootElementName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DeserializeRootElementName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EncodeSpecialCharacters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_EncodeSpecialCharacters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OmitRootObject(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_OmitRootObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WriteArrayAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_WriteArrayAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DeserializeRootElementName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DeserializeRootElementName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EncodeSpecialCharacters(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EncodeSpecialCharacters", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_OmitRootObject(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_OmitRootObject", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_WriteArrayAttribute(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WriteArrayAttribute", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XmlNodeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
