#[cfg(feature = "System+Xml+Schema+XsdBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdBuilder {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaBuilder>,
    pub reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    pub positionInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::PositionInfo>,
    pub currentEntry: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XsdBuilder_XsdEntry,
    >,
    pub nextEntry: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XsdBuilder_XsdEntry,
    >,
    pub hasChild: bool,
    pub stateHistory: quest_hook::libil2cpp::Gc<crate::System::Xml::HWStack>,
    pub containerStack: quest_hook::libil2cpp::Gc<crate::System::Collections::Stack>,
    pub nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    pub schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
    pub namespaceManager: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlNamespaceManager,
    >,
    pub canIncludeImport: bool,
    pub schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    pub xso: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    pub element: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    pub anyElement: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAny>,
    pub attribute: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaAttribute,
    >,
    pub anyAttribute: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    >,
    pub complexType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaComplexType,
    >,
    pub simpleType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaSimpleType,
    >,
    pub complexContent: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaComplexContent,
    >,
    pub complexContentExtension: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaComplexContentExtension,
    >,
    pub complexContentRestriction: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaComplexContentRestriction,
    >,
    pub simpleContent: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaSimpleContent,
    >,
    pub simpleContentExtension: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
    >,
    pub simpleContentRestriction: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaSimpleContentRestriction,
    >,
    pub simpleTypeUnion: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaSimpleTypeUnion,
    >,
    pub simpleTypeList: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaSimpleTypeList,
    >,
    pub simpleTypeRestriction: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaSimpleTypeRestriction,
    >,
    pub group: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaGroup>,
    pub groupRef: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaGroupRef,
    >,
    pub all: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAll>,
    pub choice: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaChoice>,
    pub sequence: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaSequence,
    >,
    pub particle: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaParticle,
    >,
    pub attributeGroup: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaAttributeGroup,
    >,
    pub attributeGroupRef: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaAttributeGroupRef,
    >,
    pub notation: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaNotation,
    >,
    pub identityConstraint: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    >,
    pub xpath: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaXPath>,
    pub include: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaInclude>,
    pub import: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaImport>,
    pub annotation: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaAnnotation,
    >,
    pub appInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAppInfo>,
    pub documentation: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaDocumentation,
    >,
    pub facet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaFacet>,
    pub markup: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        >,
    >,
    pub redefine: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaRedefine,
    >,
    pub validationEventHandler: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::ValidationEventHandler,
    >,
    pub unhandledAttributes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ArrayList,
    >,
    pub namespaces: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdBuilder =>
    "System.Xml.Schema"."XsdBuilder"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaBuilder>;
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
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttribute", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddParticle(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddParticle", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAnnotated_Id(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAnnotated_Id", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAnyAttribute_Namespace(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAnyAttribute_Namespace", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAnyAttribute_ProcessContents(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAnyAttribute_ProcessContents", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAny_Namespace(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAny_Namespace", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAny_ProcessContents(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAny_ProcessContents", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAppinfo_Source(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAppinfo_Source", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAttributeGroupRef_Ref(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAttributeGroupRef_Ref", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAttributeGroup_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAttributeGroup_Name", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAttribute_Default(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAttribute_Default", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAttribute_Fixed(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAttribute_Fixed", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAttribute_Form(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAttribute_Form", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAttribute_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAttribute_Name", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAttribute_Ref(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAttribute_Ref", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAttribute_Type(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAttribute_Type", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAttribute_Use(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildAttribute_Use", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildComplexContentExtension_Base(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildComplexContentExtension_Base", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildComplexContentRestriction_Base(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildComplexContentRestriction_Base", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildComplexContent_Mixed(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildComplexContent_Mixed", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildComplexType_Abstract(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildComplexType_Abstract", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildComplexType_Block(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildComplexType_Block", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildComplexType_Final(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildComplexType_Final", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildComplexType_Mixed(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildComplexType_Mixed", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildComplexType_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildComplexType_Name", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildDocumentation_Source(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildDocumentation_Source", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildDocumentation_XmlLang(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildDocumentation_XmlLang", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Abstract(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Abstract", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Block(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Block", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Default(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Default", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Final(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Final", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Fixed(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Fixed", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Form(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Form", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_MaxOccurs(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_MaxOccurs", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_MinOccurs(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_MinOccurs", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Name", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Nillable(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Nillable", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Ref(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Ref", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_SubstitutionGroup(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_SubstitutionGroup", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildElement_Type(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildElement_Type", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildFacet_Fixed(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildFacet_Fixed", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildFacet_Value(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildFacet_Value", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildField_XPath(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildField_XPath", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildGroupRef_Ref(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildGroupRef_Ref", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildGroup_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildGroup_Name", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildIdentityConstraint_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildIdentityConstraint_Name", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildIdentityConstraint_Refer(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildIdentityConstraint_Refer", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildImport_Namespace(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildImport_Namespace", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildImport_SchemaLocation(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildImport_SchemaLocation", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildInclude_SchemaLocation(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildInclude_SchemaLocation", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildNotation_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildNotation_Name", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildNotation_Public(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildNotation_Public", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildNotation_System(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildNotation_System", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildParticle_MaxOccurs(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildParticle_MaxOccurs", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildParticle_MinOccurs(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildParticle_MinOccurs", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildRedefine_SchemaLocation(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildRedefine_SchemaLocation", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSchema_AttributeFormDefault(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSchema_AttributeFormDefault", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSchema_BlockDefault(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSchema_BlockDefault", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSchema_ElementFormDefault(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSchema_ElementFormDefault", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSchema_FinalDefault(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSchema_FinalDefault", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSchema_TargetNamespace(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSchema_TargetNamespace", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSchema_Version(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSchema_Version", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSelector_XPath(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSelector_XPath", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSimpleContentExtension_Base(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSimpleContentExtension_Base", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSimpleContentRestriction_Base(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSimpleContentRestriction_Base", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSimpleTypeList_ItemType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSimpleTypeList_ItemType", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSimpleTypeRestriction_Base(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSimpleTypeRestriction_Base", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSimpleTypeUnion_MemberTypes(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSimpleTypeUnion_MemberTypes", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSimpleType_Final(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSimpleType_Final", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildSimpleType_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildSimpleType_Name", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndAppinfo(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndAppinfo", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndChildren", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EndDocumentation(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndDocumentation", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndRedefine(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndRedefine", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContainer(
        &mut self,
        state: crate::System::Xml::Schema::XsdBuilder_State,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObject,
        > = __cordl_object.invoke("GetContainer", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextState(
        &mut self,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetNextState", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAll(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitAll", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAnnotation(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitAnnotation", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAny(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitAny", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAnyAttribute(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitAnyAttribute", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAppinfo(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitAppinfo", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAttribute(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitAttribute", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAttributeGroup(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitAttributeGroup", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAttributeGroupRef(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitAttributeGroupRef", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitChoice(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitChoice", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitComplexContent(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitComplexContent", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitComplexContentExtension(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitComplexContentExtension", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitComplexContentRestriction(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitComplexContentRestriction", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitComplexType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitComplexType", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitDocumentation(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitDocumentation", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitElement(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitElement", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitFacet(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitFacet", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitField(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitField", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitGroup(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitGroup", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitGroupRef(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitGroupRef", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitIdentityConstraint(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitIdentityConstraint", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitImport(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitImport", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitInclude(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitInclude", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitNotation(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitNotation", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitRedefine(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitRedefine", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSchema(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSchema", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSelector(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSelector", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSequence(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSequence", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSimpleContent(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSimpleContent", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSimpleContentExtension(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSimpleContentExtension", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSimpleContentRestriction(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSimpleContentRestriction", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSimpleType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSimpleType", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSimpleTypeList(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSimpleTypeList", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSimpleTypeRestriction(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSimpleTypeRestriction", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSimpleTypeUnion(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitSimpleTypeUnion", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsContentParsed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsContentParsed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSkipableElement(
        &mut self,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSkipableElement", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        curmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
        eventhandler: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (reader, curmgr, schema, nameTable, schemaNames, eventhandler),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ParseBlockFinalEnum(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseBlockFinalEnum", (value, attributeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseBoolean(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParseBoolean", (value, attributeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseEnum(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseEnum", (value, attributeName, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseQName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlQualifiedName,
        > = __cordl_object.invoke("ParseQName", (value, attributeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUriReference(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseUriReference", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pop(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Pop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessAttribute(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAttribute", (prefix, name, ns, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessElement(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ProcessElement", (prefix, name, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMarkup(
        &mut self,
        markup: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMarkup", (markup))?;
        Ok(__cordl_ret.into())
    }
    pub fn Push(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_Gc1(
        &mut self,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_Gc4(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_Gc_Gc_Gc0(
        &mut self,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, arg0, arg1, arg2))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_Gc_XmlSeverityType2(
        &mut self,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args, severity))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_XmlSeverityType3(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e, severity))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetContainer(
        &mut self,
        state: crate::System::Xml::Schema::XsdBuilder_State,
        container: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContainer", (state, container))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMaxOccurs(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMaxOccurs", (particle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMinOccurs(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMinOccurs", (particle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartChildren", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        curmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
        eventhandler: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (reader, curmgr, schema, nameTable, schemaNames, eventhandler),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaNames_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaNames_Token = __cordl_object
            .invoke("get_CurrentElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObject,
        > = __cordl_object.invoke("get_ParentContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaNames_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaNames_Token = __cordl_object
            .invoke("get_ParentElement", ())?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    pub nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    pub reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+BuilderNamespaceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XsdBuilder_BuilderNamespaceManager => "System.Xml.Schema"
    ."XsdBuilder/BuilderNamespaceManager"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+BuilderNamespaceManager")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_BuilderNamespaceManager {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>;
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
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nsMgr, reader))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nsMgr, reader))?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XsdBuilder_State {
    #[default]
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Attribute: crate::System::Xml::Schema::SchemaNames_Token,
    pub BuildFunc: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction,
    >,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdAttributeEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XsdBuilder_XsdAttributeEntry => "System.Xml.Schema"
    ."XsdBuilder/XsdAttributeEntry"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdAttributeEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        build: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a, build))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        a: crate::System::Xml::Schema::SchemaNames_Token,
        build: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a, build))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdBuildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdBuilder_XsdBuildFunction
    => "System.Xml.Schema"."XsdBuilder/XsdBuildFunction"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdBuildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdBuildFunction {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
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
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEndChildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XsdBuilder_XsdEndChildFunction => "System.Xml.Schema"
    ."XsdBuilder/XsdEndChildFunction"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEndChildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
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
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Name: crate::System::Xml::Schema::SchemaNames_Token,
    pub CurrentState: crate::System::Xml::Schema::XsdBuilder_State,
    pub NextStates: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::Xml::Schema::XsdBuilder_State>,
    >,
    pub Attributes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry,
            >,
        >,
    >,
    pub InitFunc: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XsdBuilder_XsdInitFunction,
    >,
    pub EndChildFunc: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction,
    >,
    pub ParseContent: bool,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdBuilder_XsdEntry =>
    "System.Xml.Schema"."XsdBuilder/XsdEntry"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdEntry {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        nextStates: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Xml::Schema::XsdBuilder_State,
            >,
        >,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry,
                >,
            >,
        >,
        init: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XsdBuilder_XsdInitFunction,
        >,
        end: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction,
        >,
        parseContent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (n, state, nextStates, attributes, init, end, parseContent),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        n: crate::System::Xml::Schema::SchemaNames_Token,
        state: crate::System::Xml::Schema::XsdBuilder_State,
        nextStates: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Xml::Schema::XsdBuilder_State,
            >,
        >,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Schema::XsdBuilder_XsdAttributeEntry,
                >,
            >,
        >,
        init: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XsdBuilder_XsdInitFunction,
        >,
        end: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XsdBuilder_XsdEndChildFunction,
        >,
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
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdInitFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdBuilder_XsdInitFunction
    => "System.Xml.Schema"."XsdBuilder/XsdInitFunction"
);
#[cfg(feature = "System+Xml+Schema+XsdBuilder+XsdInitFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdBuilder_XsdInitFunction {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
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
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XsdBuilder>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
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
