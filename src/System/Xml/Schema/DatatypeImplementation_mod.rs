#[cfg(feature = "System+Xml+Schema+DatatypeImplementation")]
#[repr(C)]
#[derive(Debug)]
pub struct DatatypeImplementation {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaDatatype,
    pub variety: crate::System::Xml::Schema::XmlSchemaDatatypeVariety,
    pub restriction: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::RestrictionFacets,
    >,
    pub baseType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::DatatypeImplementation,
    >,
    pub valueConverter: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlValueConverter,
    >,
    pub parentSchemaType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaType,
    >,
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::DatatypeImplementation =>
    "System.Xml.Schema"."DatatypeImplementation"
);
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation")]
impl std::ops::Deref for crate::System::Xml::Schema::DatatypeImplementation {
    type Target = crate::System::Xml::Schema::XmlSchemaDatatype;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation")]
impl std::ops::DerefMut for crate::System::Xml::Schema::DatatypeImplementation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation")]
impl crate::System::Xml::Schema::DatatypeImplementation {
    #[cfg(feature = "System+Xml+Schema+DatatypeImplementation+SchemaDatatypeMap")]
    pub type SchemaDatatypeMap = crate::System::Xml::Schema::DatatypeImplementation_SchemaDatatypeMap;
    pub fn Compare(
        &mut self,
        value1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        value2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (value1, value2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateBuiltinTypes() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateBuiltinTypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateValueConverter(
        &mut self,
        schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlValueConverter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlValueConverter,
        > = __cordl_object.invoke("CreateValueConverter", (schemaType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeriveByList_XmlSchemaType0(
        &mut self,
        schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaDatatype>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        > = __cordl_object.invoke("DeriveByList", (schemaType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeriveByList_i32_XmlSchemaType1(
        &mut self,
        minSize: i32,
        schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaDatatype>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        > = __cordl_object.invoke("DeriveByList", (minSize, schemaType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeriveByRestriction(
        &mut self,
        facets: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaDatatype>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        > = __cordl_object
            .invoke("DeriveByRestriction", (facets, nameTable, schemaType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeriveByUnion(
        types: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
            >,
        >,
        schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DatatypeImplementation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeriveByUnion", (types, schemaType))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishBuiltinType(
        derivedType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
        baseType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FinishBuiltinType", (derivedType, baseType))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromTypeName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DatatypeImplementation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromTypeName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromXdrName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DatatypeImplementation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromXdrName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromXmlTokenizedType(
        token: crate::System::Xml::XmlTokenizedType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DatatypeImplementation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromXmlTokenizedType", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromXmlTokenizedTypeXsd(
        token: crate::System::Xml::XmlTokenizedType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DatatypeImplementation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromXmlTokenizedTypeXsd", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBuiltInTypes() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBuiltInTypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNormalizedStringTypeV1Compat() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNormalizedStringTypeV1Compat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrimitiveTypeCode(
        typeCode: crate::System::Xml::Schema::XmlTypeCode,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlTypeCode> {
        let __cordl_ret: crate::System::Xml::Schema::XmlTypeCode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrimitiveTypeCode", (typeCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSimpleTypeFromTypeCode(
        typeCode: crate::System::Xml::Schema::XmlTypeCode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSimpleTypeFromTypeCode", (typeCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSimpleTypeFromXsdType(
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSimpleTypeFromXsdType", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTokenTypeV1Compat() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTokenTypeV1Compat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetTypeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsComparable(
        &mut self,
        dtype: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaDatatype>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsComparable", (dtype))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDerivedFrom(
        &mut self,
        datatype: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDerivedFrom", (datatype))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEqual(
        &mut self,
        o1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        o2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEqual", (o1, o2))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParseValue_Il2CppString_XmlNameTable_IXmlNamespaceResolver0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ParseValue", (s, nameTable, nsmgr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseValue__cordl_bool1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
        createAtomicValue: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("ParseValue", (s, nameTable, nsmgr, createAtomicValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartBuiltinType(
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        dataType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartBuiltinType", (qname, dataType))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseValue(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        namespaceResolver: quest_hook::libil2cpp::Gc<
            crate::System::Xml::IXmlNamespaceResolver,
        >,
        typedValue: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = __cordl_object
            .invoke("TryParseValue", (value, nameTable, namespaceResolver, typedValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifySchemaValid(
        &mut self,
        notations: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        >,
        caller: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VerifySchemaValid", (notations, caller))?;
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
    pub fn get_AnySimpleType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AnySimpleType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Base(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DatatypeImplementation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        > = __cordl_object.invoke("get_Base", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BuiltInWhitespaceFacet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSchemaWhiteSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaWhiteSpace = __cordl_object
            .invoke("get_BuiltInWhitespaceFacet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FacetsChecker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::FacetsChecker>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::FacetsChecker,
        > = __cordl_object.invoke("get_FacetsChecker", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasLexicalFacets(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasLexicalFacets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasValueFacets(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasValueFacets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ListValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ListValueType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Restriction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::RestrictionFacets>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::RestrictionFacets,
        > = __cordl_object.invoke("get_Restriction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TokenizedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlTokenizedType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlTokenizedType = __cordl_object
            .invoke("get_TokenizedType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlTypeCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlTypeCode = __cordl_object
            .invoke("get_TypeCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UntypedAtomicType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UntypedAtomicType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValidRestrictionFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::RestrictionFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::RestrictionFlags = __cordl_object
            .invoke("get_ValidRestrictionFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlValueConverter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlValueConverter,
        > = __cordl_object.invoke("get_ValueConverter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ValueType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Variety(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaDatatypeVariety,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaDatatypeVariety = __cordl_object
            .invoke("get_Variety", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::DatatypeImplementation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation+SchemaDatatypeMap")]
#[repr(C)]
#[derive(Debug)]
pub struct DatatypeImplementation_SchemaDatatypeMap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cordl_type: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::DatatypeImplementation,
    >,
    pub parentIndex: i32,
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation+SchemaDatatypeMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::DatatypeImplementation_SchemaDatatypeMap =>
    "System.Xml.Schema"."DatatypeImplementation/SchemaDatatypeMap"
);
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation+SchemaDatatypeMap")]
impl std::ops::Deref
for crate::System::Xml::Schema::DatatypeImplementation_SchemaDatatypeMap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation+SchemaDatatypeMap")]
impl std::ops::DerefMut
for crate::System::Xml::Schema::DatatypeImplementation_SchemaDatatypeMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation+SchemaDatatypeMap")]
impl crate::System::Xml::Schema::DatatypeImplementation_SchemaDatatypeMap {
    pub fn CompareTo(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString_DatatypeImplementation0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, _cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        >,
        parentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, _cordl_type, parentIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString_DatatypeImplementation0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        >,
        parentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, _cordl_type, parentIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ParentIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit(
        sdm: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation_SchemaDatatypeMap,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DatatypeImplementation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DatatypeImplementation,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("op_Explicit", (sdm))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation+SchemaDatatypeMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::DatatypeImplementation_SchemaDatatypeMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation+SchemaDatatypeMap")]
impl AsRef<crate::System::IComparable>
for crate::System::Xml::Schema::DatatypeImplementation_SchemaDatatypeMap {
    fn as_ref(&self) -> &crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+DatatypeImplementation+SchemaDatatypeMap")]
impl AsMut<crate::System::IComparable>
for crate::System::Xml::Schema::DatatypeImplementation_SchemaDatatypeMap {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
