#[cfg(feature = "System+Xml+Schema+SchemaNames")]
#[repr(C)]
#[derive(Debug)]
pub struct SchemaNames {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub NsDataType: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsDataTypeAlias: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsDataTypeOld: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXml: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXmlNs: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXdr: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXdrAlias: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXs: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXsi: *mut quest_hook::libil2cpp::Il2CppString,
    pub XsiType: *mut quest_hook::libil2cpp::Il2CppString,
    pub XsiNil: *mut quest_hook::libil2cpp::Il2CppString,
    pub XsiSchemaLocation: *mut quest_hook::libil2cpp::Il2CppString,
    pub XsiNoNamespaceSchemaLocation: *mut quest_hook::libil2cpp::Il2CppString,
    pub XsdSchema: *mut quest_hook::libil2cpp::Il2CppString,
    pub XdrSchema: *mut quest_hook::libil2cpp::Il2CppString,
    pub QnPCData: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXml: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXmlNs: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDtDt: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXmlLang: *mut crate::System::Xml::XmlQualifiedName,
    pub QnName: *mut crate::System::Xml::XmlQualifiedName,
    pub QnType: *mut crate::System::Xml::XmlQualifiedName,
    pub QnMaxOccurs: *mut crate::System::Xml::XmlQualifiedName,
    pub QnMinOccurs: *mut crate::System::Xml::XmlQualifiedName,
    pub QnInfinite: *mut crate::System::Xml::XmlQualifiedName,
    pub QnModel: *mut crate::System::Xml::XmlQualifiedName,
    pub QnOpen: *mut crate::System::Xml::XmlQualifiedName,
    pub QnClosed: *mut crate::System::Xml::XmlQualifiedName,
    pub QnContent: *mut crate::System::Xml::XmlQualifiedName,
    pub QnMixed: *mut crate::System::Xml::XmlQualifiedName,
    pub QnEmpty: *mut crate::System::Xml::XmlQualifiedName,
    pub QnEltOnly: *mut crate::System::Xml::XmlQualifiedName,
    pub QnTextOnly: *mut crate::System::Xml::XmlQualifiedName,
    pub QnOrder: *mut crate::System::Xml::XmlQualifiedName,
    pub QnSeq: *mut crate::System::Xml::XmlQualifiedName,
    pub QnOne: *mut crate::System::Xml::XmlQualifiedName,
    pub QnMany: *mut crate::System::Xml::XmlQualifiedName,
    pub QnRequired: *mut crate::System::Xml::XmlQualifiedName,
    pub QnYes: *mut crate::System::Xml::XmlQualifiedName,
    pub QnNo: *mut crate::System::Xml::XmlQualifiedName,
    pub QnString: *mut crate::System::Xml::XmlQualifiedName,
    pub QnID: *mut crate::System::Xml::XmlQualifiedName,
    pub QnIDRef: *mut crate::System::Xml::XmlQualifiedName,
    pub QnIDRefs: *mut crate::System::Xml::XmlQualifiedName,
    pub QnEntity: *mut crate::System::Xml::XmlQualifiedName,
    pub QnEntities: *mut crate::System::Xml::XmlQualifiedName,
    pub QnNmToken: *mut crate::System::Xml::XmlQualifiedName,
    pub QnNmTokens: *mut crate::System::Xml::XmlQualifiedName,
    pub QnEnumeration: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDefault: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrSchema: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrElementType: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrElement: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrGroup: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrAttributeType: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrAttribute: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrDataType: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrDescription: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrExtends: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXdrAliasSchema: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDtType: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDtValues: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDtMaxLength: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDtMinLength: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDtMax: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDtMin: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDtMinExclusive: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDtMaxExclusive: *mut crate::System::Xml::XmlQualifiedName,
    pub QnTargetNamespace: *mut crate::System::Xml::XmlQualifiedName,
    pub QnVersion: *mut crate::System::Xml::XmlQualifiedName,
    pub QnFinalDefault: *mut crate::System::Xml::XmlQualifiedName,
    pub QnBlockDefault: *mut crate::System::Xml::XmlQualifiedName,
    pub QnFixed: *mut crate::System::Xml::XmlQualifiedName,
    pub QnAbstract: *mut crate::System::Xml::XmlQualifiedName,
    pub QnBlock: *mut crate::System::Xml::XmlQualifiedName,
    pub QnSubstitutionGroup: *mut crate::System::Xml::XmlQualifiedName,
    pub QnFinal: *mut crate::System::Xml::XmlQualifiedName,
    pub QnNillable: *mut crate::System::Xml::XmlQualifiedName,
    pub QnRef: *mut crate::System::Xml::XmlQualifiedName,
    pub QnBase: *mut crate::System::Xml::XmlQualifiedName,
    pub QnDerivedBy: *mut crate::System::Xml::XmlQualifiedName,
    pub QnNamespace: *mut crate::System::Xml::XmlQualifiedName,
    pub QnProcessContents: *mut crate::System::Xml::XmlQualifiedName,
    pub QnRefer: *mut crate::System::Xml::XmlQualifiedName,
    pub QnPublic: *mut crate::System::Xml::XmlQualifiedName,
    pub QnSystem: *mut crate::System::Xml::XmlQualifiedName,
    pub QnSchemaLocation: *mut crate::System::Xml::XmlQualifiedName,
    pub QnValue: *mut crate::System::Xml::XmlQualifiedName,
    pub QnUse: *mut crate::System::Xml::XmlQualifiedName,
    pub QnForm: *mut crate::System::Xml::XmlQualifiedName,
    pub QnElementFormDefault: *mut crate::System::Xml::XmlQualifiedName,
    pub QnAttributeFormDefault: *mut crate::System::Xml::XmlQualifiedName,
    pub QnItemType: *mut crate::System::Xml::XmlQualifiedName,
    pub QnMemberTypes: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXPath: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdSchema: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdAnnotation: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdInclude: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdImport: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdElement: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdAttribute: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdAttributeGroup: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdAnyAttribute: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdGroup: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdAll: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdChoice: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdSequence: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdAny: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdNotation: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdSimpleType: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdComplexType: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdUnique: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdKey: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdKeyRef: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdSelector: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdField: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdMinExclusive: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdMinInclusive: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdMaxInclusive: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdMaxExclusive: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdTotalDigits: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdFractionDigits: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdLength: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdMinLength: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdMaxLength: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdEnumeration: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdPattern: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdDocumentation: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdAppinfo: *mut crate::System::Xml::XmlQualifiedName,
    pub QnSource: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdComplexContent: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdSimpleContent: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdRestriction: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdExtension: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdUnion: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdList: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdWhiteSpace: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdRedefine: *mut crate::System::Xml::XmlQualifiedName,
    pub QnXsdAnyType: *mut crate::System::Xml::XmlQualifiedName,
    pub TokenToQName: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::XmlQualifiedName,
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
        Ok(__cordl_ret)
    }
    pub fn IsXDRRoot(
        &mut self,
        localName: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXDRRoot", (localName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn IsXSDRoot(
        &mut self,
        localName: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXSDRoot", (localName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameTable))?;
        Ok(__cordl_object)
    }
    pub fn SchemaTypeFromRoot(
        &mut self,
        localName: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaType = __cordl_object
            .invoke("SchemaTypeFromRoot", (localName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameTable))?;
        Ok(__cordl_ret)
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
