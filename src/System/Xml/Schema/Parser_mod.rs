#[cfg(feature = "System+Xml+Schema+Parser")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser {
    __cordl_parent: crate::System::Object,
    pub schemaType: crate::System::Xml::Schema::SchemaType,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
    pub eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    pub namespaceManager: *mut crate::System::Xml::XmlNamespaceManager,
    pub reader: *mut crate::System::Xml::XmlReader,
    pub positionInfo: *mut crate::System::Xml::PositionInfo,
    pub isProcessNamespaces: bool,
    pub schemaXmlDepth: i32,
    pub markupDepth: i32,
    pub builder: *mut crate::System::Xml::Schema::SchemaBuilder,
    pub schema: *mut crate::System::Xml::Schema::XmlSchema,
    pub xdrSchema: *mut crate::System::Xml::Schema::SchemaInfo,
    pub xmlResolver: *mut crate::System::Xml::XmlResolver,
    pub dummyDocument: *mut crate::System::Xml::XmlDocument,
    pub processMarkup: bool,
    pub parentNode: *mut crate::System::Xml::XmlNode,
    pub annotationNSManager: *mut crate::System::Xml::XmlNamespaceManager,
    pub xmlns: *mut crate::System::String,
    pub xmlCharType: crate::System::Xml::XmlCharType,
}
#[cfg(feature = "System+Xml+Schema+Parser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Parser =>
    "System.Xml.Schema"."Parser"
);
#[cfg(feature = "System+Xml+Schema+Parser")]
impl std::ops::Deref for crate::System::Xml::Schema::Parser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Parser")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Parser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Parser")]
impl crate::System::Xml::Schema::Parser {
    pub fn CheckSchemaRoot(
        &mut self,
        rootType: crate::System::Xml::Schema::SchemaType,
        code: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckSchemaRoot", (rootType, code))?;
        Ok(__cordl_ret)
    }
    pub fn CreateXmlNsAttribute(
        &mut self,
        prefix: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("CreateXmlNsAttribute", (prefix, value))?;
        Ok(__cordl_ret)
    }
    pub fn FinishParsing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaType = __cordl_object
            .invoke("FinishParsing", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn LoadElementNode(
        &mut self,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlElement = __cordl_object
            .invoke("LoadElementNode", (root))?;
        Ok(__cordl_ret)
    }
    pub fn LoadEntityReferenceInAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlEntityReference> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlEntityReference = __cordl_object
            .invoke("LoadEntityReferenceInAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        schemaType: crate::System::Xml::Schema::SchemaType,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (schemaType, nameTable, schemaNames, eventHandler))?;
        Ok(__cordl_object)
    }
    pub fn Parse(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
        targetNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaType = __cordl_object
            .invoke("Parse", (reader, targetNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ParseReaderNode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseReaderNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAppInfoDocMarkup(
        &mut self,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAppInfoDocMarkup", (root))?;
        Ok(__cordl_ret)
    }
    pub fn StartParsing(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
        targetNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartParsing", (reader, targetNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        schemaType: crate::System::Xml::Schema::SchemaType,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (schemaType, nameTable, schemaNames, eventHandler))?;
        Ok(__cordl_ret)
    }
    pub fn get_XdrSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaInfo = __cordl_object
            .invoke("get_XdrSchema", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("get_XmlSchema", ())?;
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
#[cfg(feature = "System+Xml+Schema+Parser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Parser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
