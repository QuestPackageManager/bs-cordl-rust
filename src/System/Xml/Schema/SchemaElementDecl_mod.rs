#[cfg(feature = "System+Xml+Schema+SchemaElementDecl")]
#[repr(C)]
#[derive(Debug)]
pub struct SchemaElementDecl {
    __cordl_parent: crate::System::Xml::Schema::SchemaDeclBase,
    pub attdefs: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::XmlQualifiedName,
        *mut crate::System::Xml::Schema::SchemaAttDef,
    >,
    pub defaultAttdefs: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Xml::IDtdDefaultAttributeInfo,
    >,
    pub isIdDeclared: bool,
    pub hasNonCDataAttribute: bool,
    pub isAbstract: bool,
    pub isNillable: bool,
    pub hasRequiredAttribute: bool,
    pub isNotationDeclared: bool,
    pub prohibitedAttributes: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::XmlQualifiedName,
        *mut crate::System::Xml::XmlQualifiedName,
    >,
    pub contentValidator: *mut crate::System::Xml::Schema::ContentValidator,
    pub anyAttribute: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    pub block: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub constraints: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::Schema::CompiledIdentityConstraint,
    >,
    pub schemaElement: *mut crate::System::Xml::Schema::XmlSchemaElement,
}
#[cfg(feature = "System+Xml+Schema+SchemaElementDecl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SchemaElementDecl =>
    "System.Xml.Schema"."SchemaElementDecl"
);
#[cfg(feature = "System+Xml+Schema+SchemaElementDecl")]
impl std::ops::Deref for crate::System::Xml::Schema::SchemaElementDecl {
    type Target = crate::System::Xml::Schema::SchemaDeclBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaElementDecl")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SchemaElementDecl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaElementDecl")]
impl crate::System::Xml::Schema::SchemaElementDecl {
    pub fn CheckAttributes(
        &mut self,
        presence: *mut crate::System::Collections::Hashtable,
        standalone: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckAttributes", (presence, standalone))?;
        Ok(__cordl_ret)
    }
    pub fn set_Constraints(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::CompiledIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Constraints", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_HasNonCDataAttribute(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HasNonCDataAttribute", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasRequiredAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasRequiredAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdAttributeListInfo_LookupAttribute(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IDtdAttributeInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IDtdAttributeInfo = __cordl_object
            .invoke(
                "System.Xml.IDtdAttributeListInfo.LookupAttribute",
                (prefix, localName),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlSchemaDatatype1(
        &mut self,
        dtype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dtype))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlQualifiedName_String2(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, prefix))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNotationDeclared(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNotationDeclared", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AnyAttribute(
        &mut self,
        value: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AnyAttribute", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("get_SchemaElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultAttDefs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::System::Xml::IDtdDefaultAttributeInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::System::Xml::IDtdDefaultAttributeInfo,
        > = __cordl_object.invoke("get_DefaultAttDefs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProhibitedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::XmlQualifiedName,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::XmlQualifiedName,
        > = __cordl_object.invoke("get_ProhibitedAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentValidator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::ContentValidator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::ContentValidator = __cordl_object
            .invoke("get_ContentValidator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsIdDeclared(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsIdDeclared", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsNotationDeclared(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsNotationDeclared", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ContentValidator(
        &mut self,
        value: *mut crate::System::Xml::Schema::ContentValidator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContentValidator", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AddAttDef(
        &mut self,
        attdef: *mut crate::System::Xml::Schema::SchemaAttDef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttDef", (attdef))?;
        Ok(__cordl_ret)
    }
    pub fn get_Constraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::CompiledIdentityConstraint,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::CompiledIdentityConstraint,
        > = __cordl_object.invoke("get_Constraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsNillable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsNillable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdAttributeListInfo_get_HasNonCDataAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdAttributeListInfo.get_HasNonCDataAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsAbstract(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsAbstract", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAbstract(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAbstract", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAttDef(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaAttDef> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaAttDef = __cordl_object
            .invoke("GetAttDef", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNillable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNillable", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Block(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Block", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SchemaElement(
        &mut self,
        value: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SchemaElement", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasDefaultAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDefaultAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Block(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaDerivationMethod = __cordl_object
            .invoke("get_Block", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasNonCDataAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasNonCDataAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AttDefs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaAttDef,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaAttDef,
        > = __cordl_object.invoke("get_AttDefs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AnyAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute = __cordl_object
            .invoke("get_AnyAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsIdDeclared(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsIdDeclared", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdAttributeListInfo_LookupDefaultAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Xml::IDtdDefaultAttributeInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Xml::IDtdDefaultAttributeInfo,
        > = __cordl_object
            .invoke("System.Xml.IDtdAttributeListInfo.LookupDefaultAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaElementDecl = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_XmlSchemaDatatype1(
        dtype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dtype))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlQualifiedName_String2(
        name: *mut crate::System::Xml::XmlQualifiedName,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, prefix))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaElementDecl")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::SchemaElementDecl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
