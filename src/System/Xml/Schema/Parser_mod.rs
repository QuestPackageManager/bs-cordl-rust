#[cfg(feature = "System+Xml+Schema+Parser")]
#[repr(C)]
#[derive(Debug)]
pub struct Parser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub schemaType: crate::System::Xml::Schema::SchemaType,
    pub nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    pub schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
    pub eventHandler: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::ValidationEventHandler,
    >,
    pub namespaceManager: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlNamespaceManager,
    >,
    pub reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    pub positionInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::PositionInfo>,
    pub isProcessNamespaces: bool,
    pub schemaXmlDepth: i32,
    pub markupDepth: i32,
    pub builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaBuilder>,
    pub schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    pub xdrSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaInfo>,
    pub xmlResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    pub dummyDocument: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    pub processMarkup: bool,
    pub parentNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    pub annotationNSManager: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlNamespaceManager,
    >,
    pub xmlns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub xmlCharType: crate::System::Xml::XmlCharType,
}
#[cfg(feature = "System+Xml+Schema+Parser")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::Parser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "Parser";
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
#[cfg(feature = "System+Xml+Schema+Parser")]
impl std::ops::Deref for crate::System::Xml::Schema::Parser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        code: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckSchemaRoot", (rootType, code))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateXmlNsAttribute(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("CreateXmlNsAttribute", (prefix, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishParsing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaType = __cordl_object
            .invoke("FinishParsing", ())?;
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
    pub fn LoadElementNode(
        &mut self,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement> = __cordl_object
            .invoke("LoadElementNode", (root))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadEntityReferenceInAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlEntityReference>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlEntityReference,
        > = __cordl_object.invoke("LoadEntityReferenceInAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        schemaType: crate::System::Xml::Schema::SchemaType,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
        eventHandler: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (schemaType, nameTable, schemaNames, eventHandler))?;
        Ok(__cordl_object.into())
    }
    pub fn Parse(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaType = __cordl_object
            .invoke("Parse", (reader, targetNamespace))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseReaderNode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseReaderNode", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn StartParsing(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartParsing", (reader, targetNamespace))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        schemaType: crate::System::Xml::Schema::SchemaType,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
        eventHandler: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (schemaType, nameTable, schemaNames, eventHandler))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XdrSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SchemaInfo,
        > = __cordl_object.invoke("get_XdrSchema", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = __cordl_object.invoke("get_XmlSchema", ())?;
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
#[cfg(feature = "System+Xml+Schema+Parser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Parser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
