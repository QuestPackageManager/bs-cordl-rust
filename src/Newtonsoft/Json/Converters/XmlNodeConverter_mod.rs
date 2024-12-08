#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNodeConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter,
    pub _DeserializeRootElementName_k__BackingField: *mut crate::System::String,
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
    pub fn IsXObject(
        &mut self,
        valueType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXObject", (valueType))?;
        Ok(__cordl_ret)
    }
    pub fn get_EncodeSpecialCharacters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_EncodeSpecialCharacters", ())?;
        Ok(__cordl_ret)
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
    pub fn CreateElement_JsonReader_IXmlNode_String_XmlNamespaceManager_String_Dictionary_2_0(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        document: *mut crate::Newtonsoft::Json::Converters::IXmlDocument,
        currentNode: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        elementName: *mut crate::System::String,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
        elementPrefix: *mut crate::System::String,
        attributeNameValues: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn CreateElement_String_String_XmlNamespaceManager1(
        &mut self,
        elementName: *mut crate::System::String,
        document: *mut crate::Newtonsoft::Json::Converters::IXmlDocument,
        elementPrefix: *mut crate::System::String,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlElement = __cordl_object
            .invoke("CreateElement", (elementName, document, elementPrefix, manager))?;
        Ok(__cordl_ret)
    }
    pub fn AddJsonArrayAttribute(
        &mut self,
        element: *mut crate::Newtonsoft::Json::Converters::IXmlElement,
        document: *mut crate::Newtonsoft::Json::Converters::IXmlDocument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddJsonArrayAttribute", (element, document))?;
        Ok(__cordl_ret)
    }
    pub fn WriteJson(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteJson", (writer, value, serializer))?;
        Ok(__cordl_ret)
    }
    pub fn IsNamespaceAttribute(
        &mut self,
        attributeName: *mut crate::System::String,
        prefix: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsNamespaceAttribute", (attributeName, prefix))?;
        Ok(__cordl_ret)
    }
    pub fn set_DeserializeRootElementName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DeserializeRootElementName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeNode(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        document: *mut crate::Newtonsoft::Json::Converters::IXmlDocument,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
        currentNode: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeNode", (reader, document, manager, currentNode))?;
        Ok(__cordl_ret)
    }
    pub fn IsXmlNode(
        &mut self,
        valueType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXmlNode", (valueType))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeGroupedNodes(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        node: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
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
        Ok(__cordl_ret)
    }
    pub fn ShouldReadInto(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldReadInto", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveFullName(
        &mut self,
        node: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ResolveFullName", (node, manager))?;
        Ok(__cordl_ret)
    }
    pub fn CanConvert(
        &mut self,
        valueType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanConvert", (valueType))?;
        Ok(__cordl_ret)
    }
    pub fn ValueAttributes(
        &mut self,
        c: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValueAttributes", (c))?;
        Ok(__cordl_ret)
    }
    pub fn get_WriteArrayAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_WriteArrayAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsArray(
        &mut self,
        node: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsArray", (node))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn WrapXml(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("WrapXml", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArrayElements(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        document: *mut crate::Newtonsoft::Json::Converters::IXmlDocument,
        propertyName: *mut crate::System::String,
        currentNode: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReadArrayElements",
                (reader, document, propertyName, currentNode, manager),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PushParentNamespaces(
        &mut self,
        node: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushParentNamespaces", (node, manager))?;
        Ok(__cordl_ret)
    }
    pub fn CreateInstruction(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        document: *mut crate::Newtonsoft::Json::Converters::IXmlDocument,
        currentNode: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateInstruction", (reader, document, currentNode, propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeNode(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        node: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
        writePropertyName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeNode", (writer, node, manager, writePropertyName))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAttributeElements(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("ReadAttributeElements", (reader, manager))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyName(
        &mut self,
        node: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetPropertyName", (node, manager))?;
        Ok(__cordl_ret)
    }
    pub fn ReadJson(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        existingValue: *mut crate::System::Object,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadJson", (reader, objectType, existingValue, serializer))?;
        Ok(__cordl_ret)
    }
    pub fn ReadElement(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        document: *mut crate::Newtonsoft::Json::Converters::IXmlDocument,
        currentNode: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        propertyName: *mut crate::System::String,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReadElement",
                (reader, document, currentNode, propertyName, manager),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_DeserializeRootElementName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DeserializeRootElementName", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn WriteGroupedNodes_List_1_0(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
        writePropertyName: bool,
        groupedNodes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
        elementNames: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteGroupedNodes",
                (writer, manager, writePropertyName, groupedNodes, elementNames),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteGroupedNodes_IXmlNode1(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
        writePropertyName: bool,
        node: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        elementNames: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteGroupedNodes",
                (writer, manager, writePropertyName, node, elementNames),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateDocumentType(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        document: *mut crate::Newtonsoft::Json::Converters::IXmlDocument,
        currentNode: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateDocumentType", (reader, document, currentNode))?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeValue(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        document: *mut crate::Newtonsoft::Json::Converters::IXmlDocument,
        manager: *mut crate::System::Xml::XmlNamespaceManager,
        propertyName: *mut crate::System::String,
        currentNode: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DeserializeValue",
                (reader, document, manager, propertyName, currentNode),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_OmitRootObject(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_OmitRootObject", ())?;
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
