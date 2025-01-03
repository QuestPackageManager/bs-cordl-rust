#[cfg(feature = "System+Xml+Schema+SchemaNames")]
#[repr(C)]
#[derive(Debug)]
pub struct SchemaNames {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    pub NsDataType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NsDataTypeAlias: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NsDataTypeOld: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NsXml: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NsXmlNs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NsXdr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NsXdrAlias: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NsXs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NsXsi: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub XsiType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub XsiNil: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub XsiSchemaLocation: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub XsiNoNamespaceSchemaLocation: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub XsdSchema: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub XdrSchema: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub QnPCData: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXml: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXmlNs: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnDtDt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXmlLang: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnType: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnMaxOccurs: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnMinOccurs: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnInfinite: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnModel: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnOpen: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnClosed: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnContent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnMixed: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnEmpty: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnEltOnly: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnTextOnly: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnOrder: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnSeq: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnOne: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnMany: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnRequired: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnYes: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnNo: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnString: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnID: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnIDRef: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnIDRefs: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnEntity: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnEntities: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnNmToken: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnNmTokens: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnEnumeration: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnDefault: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXdrSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXdrElementType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXdrElement: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXdrGroup: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXdrAttributeType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXdrAttribute: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXdrDataType: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXdrDescription: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXdrExtends: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXdrAliasSchema: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnDtType: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnDtValues: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnDtMaxLength: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnDtMinLength: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnDtMax: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnDtMin: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnDtMinExclusive: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnDtMaxExclusive: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnTargetNamespace: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnVersion: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnFinalDefault: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnBlockDefault: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnFixed: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnAbstract: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnBlock: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnSubstitutionGroup: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnFinal: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnNillable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnRef: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnBase: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnDerivedBy: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnNamespace: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnProcessContents: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnRefer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnPublic: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnSystem: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnSchemaLocation: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnValue: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnUse: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnForm: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnElementFormDefault: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnAttributeFormDefault: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnItemType: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnMemberTypes: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXPath: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdAnnotation: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdInclude: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdImport: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdElement: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdAttribute: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdAttributeGroup: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdAnyAttribute: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdGroup: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdAll: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdChoice: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdSequence: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdAny: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdNotation: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdSimpleType: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdComplexType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdUnique: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdKey: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdKeyRef: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdSelector: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdField: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdMinExclusive: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdMinInclusive: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdMaxInclusive: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdMaxExclusive: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdTotalDigits: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdFractionDigits: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdLength: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdMinLength: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdMaxLength: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdEnumeration: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdPattern: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdDocumentation: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdAppinfo: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnSource: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdComplexContent: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdSimpleContent: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdRestriction: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlQualifiedName,
    >,
    pub QnXsdExtension: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdUnion: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdList: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdWhiteSpace: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdRedefine: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub QnXsdAnyType: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub TokenToQName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::XmlQualifiedName>,
    >,
}
#[cfg(feature = "System+Xml+Schema+SchemaNames")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SchemaNames =>
    "System.Xml.Schema"."SchemaNames"
);
#[cfg(feature = "System+Xml+Schema+SchemaNames")]
impl std::ops::Deref for crate::System::Xml::Schema::SchemaNames {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaNames")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SchemaNames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaNames")]
impl crate::System::Xml::Schema::SchemaNames {
    #[cfg(feature = "System+Xml+Schema+SchemaNames+Token")]
    pub type Token = crate::System::Xml::Schema::SchemaNames_Token;
    pub fn CreateTokenToQNameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateTokenToQNameTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsXDRRoot(
        &mut self,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXDRRoot", (localName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsXSDRoot(
        &mut self,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXSDRoot", (localName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameTable))?;
        Ok(__cordl_object.into())
    }
    pub fn SchemaTypeFromRoot(
        &mut self,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaType = __cordl_object
            .invoke("SchemaTypeFromRoot", (localName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameTable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaNames")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::SchemaNames {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaNames+Token")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaNames_Token {
    Empty = 0i32,
    SchemaAbstract = 54i32,
    SchemaAttributeFormDefault = 70i32,
    SchemaBase = 60i32,
    SchemaBlock = 55i32,
    SchemaBlockDefault = 52i32,
    SchemaClosed = 8i32,
    SchemaContent = 9i32,
    SchemaDefault = 30i32,
    SchemaDerivedBy = 61i32,
    SchemaDtMax = 45i32,
    SchemaDtMaxExclusive = 48i32,
    SchemaDtMaxLength = 43i32,
    SchemaDtMin = 46i32,
    SchemaDtMinExclusive = 47i32,
    SchemaDtMinLength = 44i32,
    SchemaDtType = 41i32,
    SchemaDtValues = 42i32,
    SchemaElementFormDefault = 71i32,
    SchemaElementOnly = 12i32,
    SchemaEmpty = 11i32,
    SchemaEntities = 26i32,
    SchemaEntity = 25i32,
    SchemaEnumeration = 29i32,
    SchemaFinal = 57i32,
    SchemaFinalDefault = 51i32,
    SchemaFixed = 53i32,
    SchemaForm = 73i32,
    SchemaId = 22i32,
    SchemaIdref = 23i32,
    SchemaIdrefs = 24i32,
    SchemaInfinite = 5i32,
    SchemaItemType = 119i32,
    SchemaMany = 17i32,
    SchemaMaxOccurs = 3i32,
    SchemaMemberTypes = 120i32,
    SchemaMinOccurs = 4i32,
    SchemaMixed = 10i32,
    SchemaModel = 6i32,
    SchemaName = 1i32,
    SchemaNamespace = 62i32,
    SchemaNillable = 58i32,
    SchemaNmtoken = 27i32,
    SchemaNmtokens = 28i32,
    SchemaNo = 20i32,
    SchemaOne = 16i32,
    SchemaOpen = 7i32,
    SchemaOrder = 14i32,
    SchemaProcessContents = 63i32,
    SchemaPublic = 65i32,
    SchemaRef = 59i32,
    SchemaRefer = 64i32,
    SchemaRequired = 18i32,
    SchemaSchemaLocation = 67i32,
    SchemaSeq = 15i32,
    SchemaSource = 69i32,
    SchemaString = 21i32,
    SchemaSubstitutionGroup = 56i32,
    SchemaSystem = 66i32,
    SchemaTargetNamespace = 49i32,
    SchemaTextOnly = 13i32,
    SchemaType = 2i32,
    SchemaUse = 72i32,
    SchemaValue = 68i32,
    SchemaVersion = 50i32,
    SchemaXPath = 121i32,
    SchemaXdrRootAlias = 40i32,
    SchemaYes = 19i32,
    XdrAttribute = 36i32,
    XdrAttributeType = 35i32,
    XdrDatatype = 37i32,
    XdrDescription = 38i32,
    XdrElement = 33i32,
    XdrElementType = 32i32,
    XdrExtends = 39i32,
    XdrGroup = 34i32,
    XdrRoot = 31i32,
    XmlLang = 122i32,
    XsdAll = 83i32,
    XsdAnnotation = 75i32,
    XsdAny = 86i32,
    XsdAnyAttribute = 81i32,
    XsdAppInfo = 107i32,
    XsdAttribute = 79i32,
    XsdChoice = 84i32,
    XsdComplexContent = 108i32,
    XsdComplexContentExtension = 109i32,
    XsdComplexContentRestriction = 110i32,
    XsdComplexType = 89i32,
    XsdDocumentation = 106i32,
    XsdElement = 78i32,
    XsdEnumeration = 104i32,
    XsdField = 94i32,
    XsdFractionDigits = 100i32,
    XsdGroup = 82i32,
    XsdImport = 77i32,
    XsdInclude = 76i32,
    XsdKey = 91i32,
    XsdKeyref = 92i32,
    XsdLength = 101i32,
    XsdMaxExclusive = 97i32,
    XsdMaxInclusive = 98i32,
    XsdMaxLength = 103i32,
    XsdMinExclusive = 95i32,
    XsdMinInclusive = 96i32,
    XsdMinLength = 102i32,
    XsdNotation = 87i32,
    XsdPattern = 105i32,
    XsdRedefine = 118i32,
    XsdSchema = 74i32,
    XsdSelector = 93i32,
    XsdSequence = 85i32,
    XsdSimpleContent = 111i32,
    XsdSimpleContentExtension = 112i32,
    XsdSimpleContentRestriction = 113i32,
    XsdSimpleType = 88i32,
    XsdSimpleTypeList = 114i32,
    XsdSimpleTypeRestriction = 115i32,
    XsdSimpleTypeUnion = 116i32,
    XsdTotalDigits = 99i32,
    XsdUnique = 90i32,
    XsdWhitespace = 117i32,
    xsdAttributeGroup = 80i32,
}
#[cfg(feature = "System+Xml+Schema+SchemaNames+Token")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SchemaNames_Token =>
    "System.Xml.Schema"."SchemaNames/Token"
);
