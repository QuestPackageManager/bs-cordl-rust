#[cfg(feature = "System+Xml+Schema+XsdBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdBuilder {
    __cordl_parent: crate::System::Xml::Schema::SchemaBuilder,
    pub reader: *mut crate::System::Xml::XmlReader,
    pub positionInfo: *mut crate::System::Xml::PositionInfo,
    pub currentEntry: *mut crate::System::Xml::Schema::XsdBuilder_XsdEntry,
    pub nextEntry: *mut crate::System::Xml::Schema::XsdBuilder_XsdEntry,
    pub hasChild: bool,
    pub stateHistory: *mut crate::System::Xml::HWStack,
    pub containerStack: *mut crate::System::Collections::Stack,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
    pub namespaceManager: *mut crate::System::Xml::XmlNamespaceManager,
    pub canIncludeImport: bool,
    pub schema: *mut crate::System::Xml::Schema::XmlSchema,
    pub xso: *mut crate::System::Xml::Schema::XmlSchemaObject,
    pub element: *mut crate::System::Xml::Schema::XmlSchemaElement,
    pub anyElement: *mut crate::System::Xml::Schema::XmlSchemaAny,
    pub attribute: *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    pub anyAttribute: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    pub complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    pub simpleType: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    pub complexContent: *mut crate::System::Xml::Schema::XmlSchemaComplexContent,
    pub complexContentExtension: *mut crate::System::Xml::Schema::XmlSchemaComplexContentExtension,
    pub complexContentRestriction: *mut crate::System::Xml::Schema::XmlSchemaComplexContentRestriction,
    pub simpleContent: *mut crate::System::Xml::Schema::XmlSchemaSimpleContent,
    pub simpleContentExtension: *mut crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
    pub simpleContentRestriction: *mut crate::System::Xml::Schema::XmlSchemaSimpleContentRestriction,
    pub simpleTypeUnion: *mut crate::System::Xml::Schema::XmlSchemaSimpleTypeUnion,
    pub simpleTypeList: *mut crate::System::Xml::Schema::XmlSchemaSimpleTypeList,
    pub simpleTypeRestriction: *mut crate::System::Xml::Schema::XmlSchemaSimpleTypeRestriction,
    pub group: *mut crate::System::Xml::Schema::XmlSchemaGroup,
    pub groupRef: *mut crate::System::Xml::Schema::XmlSchemaGroupRef,
    pub all: *mut crate::System::Xml::Schema::XmlSchemaAll,
    pub choice: *mut crate::System::Xml::Schema::XmlSchemaChoice,
    pub sequence: *mut crate::System::Xml::Schema::XmlSchemaSequence,
    pub particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    pub attributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
    pub attributeGroupRef: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroupRef,
    pub notation: *mut crate::System::Xml::Schema::XmlSchemaNotation,
    pub identityConstraint: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    pub xpath: *mut crate::System::Xml::Schema::XmlSchemaXPath,
    pub include: *mut crate::System::Xml::Schema::XmlSchemaInclude,
    pub import: *mut crate::System::Xml::Schema::XmlSchemaImport,
    pub annotation: *mut crate::System::Xml::Schema::XmlSchemaAnnotation,
    pub appInfo: *mut crate::System::Xml::Schema::XmlSchemaAppInfo,
    pub documentation: *mut crate::System::Xml::Schema::XmlSchemaDocumentation,
    pub facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    pub markup: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::XmlNode,
    >,
    pub redefine: *mut crate::System::Xml::Schema::XmlSchemaRedefine,
    pub validationEventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    pub unhandledAttributes: *mut crate::System::Collections::ArrayList,
    pub namespaces: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdBuilder =>
    "System.Xml.Schema"."XsdBuilder"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder {
    type Target = crate::System::Xml::Schema::SchemaBuilder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XsdBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder")]
impl crate::System::Xml::Schema::XsdBuilder {
    #[cfg(feature = "System+Xml+Schema+XsdBuilder+BuilderNamespaceManager")]
    pub type BuilderNamespaceManager = crate::System::Xml::Schema::XsdBuilder_BuilderNamespaceManager;
    #[cfg(feature = "System+Xml+Schema+XsdBuilder+State")]
    pub type State = crate::System::Xml::Schema::XsdBuilder_State;
    #[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdAttributeEntry")]
    pub type XsdAttributeEntry = crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry;
    #[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdBuildFunction")]
    pub type XsdBuildFunction = crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction;
    #[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEndChildFunction")]
    pub type XsdEndChildFunction = crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction;
    #[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEntry")]
    pub type XsdEntry = crate::System::Xml::Schema::XsdBuilder_XsdEntry;
    #[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdInitFunction")]
    pub type XsdInitFunction = crate::System::Xml::Schema::XsdBuilder_XsdInitFunction;
    pub fn AddAttribute(
        &mut self,
        value: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttribute", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AddParticle(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddParticle", (particle))?;
        Ok(__cordl_ret)
    }
    pub fn EndChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndChildren", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetContainer(
        &mut self,
        state: crate::System::Xml::Schema::XsdBuilder_State,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObject = __cordl_object
            .invoke("GetContainer", (state))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextState(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetNextState", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn IsContentParsed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsContentParsed", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsSkipableElement(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSkipableElement", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        reader: *mut crate::System::Xml::XmlReader,
        curmgr: *mut crate::System::Xml::XmlNamespaceManager,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventhandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (reader, curmgr, schema, nameTable, schemaNames, eventhandler),
            )?;
        Ok(__cordl_object)
    }
    pub fn ParseBlockFinalEnum(
        &mut self,
        value: *mut crate::System::String,
        attributeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseBlockFinalEnum", (value, attributeName))?;
        Ok(__cordl_ret)
    }
    pub fn ParseBoolean(
        &mut self,
        value: *mut crate::System::String,
        attributeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParseBoolean", (value, attributeName))?;
        Ok(__cordl_ret)
    }
    pub fn ParseEnum(
        &mut self,
        value: *mut crate::System::String,
        attributeName: *mut crate::System::String,
        values: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseEnum", (value, attributeName, values))?;
        Ok(__cordl_ret)
    }
    pub fn ParseQName(
        &mut self,
        value: *mut crate::System::String,
        attributeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("ParseQName", (value, attributeName))?;
        Ok(__cordl_ret)
    }
    pub fn Pop(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAttribute(
        &mut self,
        prefix: *mut crate::System::String,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAttribute", (prefix, name, ns, value))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCData(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessElement(
        &mut self,
        prefix: *mut crate::System::String,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ProcessElement", (prefix, name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMarkup(
        &mut self,
        markup: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMarkup", (markup))?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecordPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_Il2CppArray_XmlSeverityType2(
        &mut self,
        code: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_String1(
        &mut self,
        code: *mut crate::System::String,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, msg))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_String_String_String0(
        &mut self,
        code: *mut crate::System::String,
        arg0: *mut crate::System::String,
        arg1: *mut crate::System::String,
        arg2: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, arg0, arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaException4(
        &mut self,
        e: *mut crate::System::Xml::Schema::XmlSchemaException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaException_XmlSeverityType3(
        &mut self,
        e: *mut crate::System::Xml::Schema::XmlSchemaException,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SetContainer(
        &mut self,
        state: crate::System::Xml::Schema::XsdBuilder_State,
        container: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContainer", (state, container))?;
        Ok(__cordl_ret)
    }
    pub fn SetMaxOccurs(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMaxOccurs", (particle, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetMinOccurs(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMinOccurs", (particle, value))?;
        Ok(__cordl_ret)
    }
    pub fn StartChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartChildren", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
        curmgr: *mut crate::System::Xml::XmlNamespaceManager,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventhandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (reader, curmgr, schema, nameTable, schemaNames, eventhandler),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaNames_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaNames_Token = __cordl_object
            .invoke("get_CurrentElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObject = __cordl_object
            .invoke("get_ParentContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaNames_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaNames_Token = __cordl_object
            .invoke("get_ParentElement", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XsdBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+BuilderNamespaceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdBuilder_BuilderNamespaceManager {
    __cordl_parent: crate::System::Xml::XmlNamespaceManager,
    pub nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
    pub reader: *mut crate::System::Xml::XmlReader,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+BuilderNamespaceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XsdBuilder_BuilderNamespaceManager => "System.Xml.Schema"
    ."XsdBuilder/BuilderNamespaceManager"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+BuilderNamespaceManager")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_BuilderNamespaceManager {
    type Target = crate::System::Xml::XmlNamespaceManager;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+BuilderNamespaceManager")]
impl std::ops::DerefMut
for crate::System::Xml::Schema::XsdBuilder_BuilderNamespaceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+BuilderNamespaceManager")]
impl crate::System::Xml::Schema::XsdBuilder_BuilderNamespaceManager {
    pub fn LookupNamespace(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nsMgr, reader))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        nsMgr: *mut crate::System::Xml::XmlNamespaceManager,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nsMgr, reader))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+BuilderNamespaceManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XsdBuilder_BuilderNamespaceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XsdBuilder_State {
    All = 12i32,
    Annotation = 2i32,
    Any = 15i32,
    AnyAttribute = 9i32,
    AppInfo = 45i32,
    Attribute = 6i32,
    AttributeGroup = 7i32,
    AttributeGroupRef = 8i32,
    Choice = 13i32,
    ComplexContent = 19i32,
    ComplexContentExtension = 21i32,
    ComplexContentRestriction = 20i32,
    ComplexType = 18i32,
    Documentation = 46i32,
    Element = 5i32,
    Enumeration = 42i32,
    Field = 32i32,
    FractionDigits = 38i32,
    Group = 10i32,
    GroupRef = 11i32,
    Import = 4i32,
    Include = 3i32,
    Key = 29i32,
    KeyRef = 30i32,
    Length = 39i32,
    MaxExclusive = 35i32,
    MaxInclusive = 36i32,
    MaxLength = 41i32,
    MinExclusive = 33i32,
    MinInclusive = 34i32,
    MinLength = 40i32,
    Notation = 16i32,
    Pattern = 43i32,
    Redefine = 47i32,
    Root = 0i32,
    Schema = 1i32,
    Selector = 31i32,
    Sequence = 14i32,
    SimpleContent = 22i32,
    SimpleContentExtension = 23i32,
    SimpleContentRestriction = 24i32,
    SimpleType = 17i32,
    SimpleTypeList = 26i32,
    SimpleTypeRestriction = 27i32,
    SimpleTypeUnion = 25i32,
    TotalDigits = 37i32,
    Unique = 28i32,
    WhiteSpace = 44i32,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdBuilder_State =>
    "System.Xml.Schema"."XsdBuilder/State"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdAttributeEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdBuilder_XsdAttributeEntry {
    __cordl_parent: crate::System::Object,
    pub Attribute: crate::System::Xml::Schema::SchemaNames_Token,
    pub BuildFunc: *mut crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdAttributeEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XsdBuilder_XsdAttributeEntry => "System.Xml.Schema"
    ."XsdBuilder/XsdAttributeEntry"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdAttributeEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdAttributeEntry")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdAttributeEntry")]
impl crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry {
    pub fn New(
        a: crate::System::Xml::Schema::SchemaNames_Token,
        build: *mut crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a, build))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        a: crate::System::Xml::Schema::SchemaNames_Token,
        build: *mut crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a, build))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdAttributeEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdBuildFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdBuilder_XsdBuildFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdBuildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdBuilder_XsdBuildFunction
    => "System.Xml.Schema"."XsdBuilder/XsdBuildFunction"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdBuildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdBuildFunction")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdBuildFunction")]
impl crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction {
    pub fn Invoke(
        &mut self,
        builder: *mut crate::System::Xml::Schema::XsdBuilder,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder, value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdBuildFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEndChildFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdBuilder_XsdEndChildFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEndChildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XsdBuilder_XsdEndChildFunction => "System.Xml.Schema"
    ."XsdBuilder/XsdEndChildFunction"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEndChildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEndChildFunction")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEndChildFunction")]
impl crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction {
    pub fn Invoke(
        &mut self,
        builder: *mut crate::System::Xml::Schema::XsdBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEndChildFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdBuilder_XsdEntry {
    __cordl_parent: crate::System::Object,
    pub Name: crate::System::Xml::Schema::SchemaNames_Token,
    pub CurrentState: crate::System::Xml::Schema::XsdBuilder_State,
    pub NextStates: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::Schema::XsdBuilder_State,
    >,
    pub Attributes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry,
    >,
    pub InitFunc: *mut crate::System::Xml::Schema::XsdBuilder_XsdInitFunction,
    pub EndChildFunc: *mut crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction,
    pub ParseContent: bool,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdBuilder_XsdEntry =>
    "System.Xml.Schema"."XsdBuilder/XsdEntry"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdEntry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEntry")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XsdBuilder_XsdEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEntry")]
impl crate::System::Xml::Schema::XsdBuilder_XsdEntry {
    pub fn New(
        n: crate::System::Xml::Schema::SchemaNames_Token,
        state: crate::System::Xml::Schema::XsdBuilder_State,
        nextStates: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Xml::Schema::XsdBuilder_State,
        >,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry,
        >,
        init: *mut crate::System::Xml::Schema::XsdBuilder_XsdInitFunction,
        end: *mut crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction,
        parseContent: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (n, state, nextStates, attributes, init, end, parseContent),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        n: crate::System::Xml::Schema::SchemaNames_Token,
        state: crate::System::Xml::Schema::XsdBuilder_State,
        nextStates: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Xml::Schema::XsdBuilder_State,
        >,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry,
        >,
        init: *mut crate::System::Xml::Schema::XsdBuilder_XsdInitFunction,
        end: *mut crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction,
        parseContent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (n, state, nextStates, attributes, init, end, parseContent),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XsdBuilder_XsdEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdInitFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdBuilder_XsdInitFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdInitFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdBuilder_XsdInitFunction
    => "System.Xml.Schema"."XsdBuilder/XsdInitFunction"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdInitFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdInitFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdInitFunction")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XsdBuilder_XsdInitFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdInitFunction")]
impl crate::System::Xml::Schema::XsdBuilder_XsdInitFunction {
    pub fn Invoke(
        &mut self,
        builder: *mut crate::System::Xml::Schema::XsdBuilder,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder, value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdInitFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XsdBuilder_XsdInitFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
