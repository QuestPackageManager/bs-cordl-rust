#[cfg(feature = "System+Xml+Schema+SchemaInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SchemaInfo {
    __cordl_parent: crate::System::Object,
    pub elementDecls: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::XmlQualifiedName,
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    >,
    pub undeclaredElementDecls: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::XmlQualifiedName,
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    >,
    pub generalEntities: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::XmlQualifiedName,
        *mut crate::System::Xml::Schema::SchemaEntity,
    >,
    pub parameterEntities: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::XmlQualifiedName,
        *mut crate::System::Xml::Schema::SchemaEntity,
    >,
    pub docTypeName: *mut crate::System::Xml::XmlQualifiedName,
    pub internalDtdSubset: *mut crate::System::String,
    pub hasNonCDataAttributes: bool,
    pub hasDefaultAttributes: bool,
    pub targetNamespaces: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        bool,
    >,
    pub attributeDecls: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::XmlQualifiedName,
        *mut crate::System::Xml::Schema::SchemaAttDef,
    >,
    pub errorCount: i32,
    pub schemaType: crate::System::Xml::Schema::SchemaType,
    pub elementDeclsByType: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::XmlQualifiedName,
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    >,
    pub notations: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::Xml::Schema::SchemaNotation,
    >,
}
#[cfg(feature = "System+Xml+Schema+SchemaInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SchemaInfo =>
    "System.Xml.Schema"."SchemaInfo"
);
#[cfg(feature = "System+Xml+Schema+SchemaInfo")]
impl std::ops::Deref for crate::System::Xml::Schema::SchemaInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaInfo")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SchemaInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaInfo")]
impl crate::System::Xml::Schema::SchemaInfo {
    pub fn Add(
        &mut self,
        sinfo: *mut crate::System::Xml::Schema::SchemaInfo,
        eventhandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (sinfo, eventhandler))?;
        Ok(__cordl_ret)
    }
    pub fn Contains(
        &mut self,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributeXdr(
        &mut self,
        ed: *mut crate::System::Xml::Schema::SchemaElementDecl,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaAttDef> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaAttDef = __cordl_object
            .invoke("GetAttributeXdr", (ed, qname))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributeXsd_ByRefMut1(
        &mut self,
        ed: *mut crate::System::Xml::Schema::SchemaElementDecl,
        qname: *mut crate::System::Xml::XmlQualifiedName,
        skip: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaAttDef> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaAttDef = __cordl_object
            .invoke("GetAttributeXsd", (ed, qname, skip))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributeXsd_XmlSchemaObject_ByRefMut0(
        &mut self,
        ed: *mut crate::System::Xml::Schema::SchemaElementDecl,
        qname: *mut crate::System::Xml::XmlQualifiedName,
        partialValidationType: *mut crate::System::Xml::Schema::XmlSchemaObject,
        attributeMatchState: quest_hook::libil2cpp::ByRefMut<
            crate::System::Xml::Schema::AttributeMatchState,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaAttDef> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaAttDef = __cordl_object
            .invoke(
                "GetAttributeXsd",
                (ed, qname, partialValidationType, attributeMatchState),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetElement(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("GetElement", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn GetElementDecl(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaElementDecl = __cordl_object
            .invoke("GetElementDecl", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeDecl(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaElementDecl = __cordl_object
            .invoke("GetTypeDecl", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn HasSchema(
        &mut self,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasSchema", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn System_Xml_IDtdInfo_LookupAttributeList(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IDtdAttributeListInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IDtdAttributeListInfo = __cordl_object
            .invoke("System.Xml.IDtdInfo.LookupAttributeList", (prefix, localName))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdInfo_LookupEntity(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IDtdEntityInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IDtdEntityInfo = __cordl_object
            .invoke("System.Xml.IDtdInfo.LookupEntity", (name))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdInfo_get_HasDefaultAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdInfo.get_HasDefaultAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdInfo_get_HasNonCDataAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdInfo.get_HasNonCDataAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdInfo_get_InternalDtdSubset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("System.Xml.IDtdInfo.get_InternalDtdSubset", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdInfo_get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("System.Xml.IDtdInfo.get_Name", ())?;
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
    pub fn get_AttributeDecls(
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
        > = __cordl_object.invoke("get_AttributeDecls", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DocTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("get_DocTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ElementDecls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaElementDecl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaElementDecl,
        > = __cordl_object.invoke("get_ElementDecls", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ElementDeclsByType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaElementDecl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaElementDecl,
        > = __cordl_object.invoke("get_ElementDeclsByType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ErrorCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ErrorCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GeneralEntities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaEntity,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaEntity,
        > = __cordl_object.invoke("get_GeneralEntities", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Notations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::Xml::Schema::SchemaNotation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::Xml::Schema::SchemaNotation,
        > = __cordl_object.invoke("get_Notations", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParameterEntities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaEntity,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaEntity,
        > = __cordl_object.invoke("get_ParameterEntities", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::SchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaType = __cordl_object
            .invoke("get_SchemaType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TargetNamespaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            bool,
        > = __cordl_object.invoke("get_TargetNamespaces", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UndeclaredElementDecls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaElementDecl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Xml::XmlQualifiedName,
            *mut crate::System::Xml::Schema::SchemaElementDecl,
        > = __cordl_object.invoke("get_UndeclaredElementDecls", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_DocTypeName(
        &mut self,
        value: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DocTypeName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ErrorCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ErrorCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_InternalDtdSubset(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InternalDtdSubset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SchemaType(
        &mut self,
        value: crate::System::Xml::Schema::SchemaType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SchemaType", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::SchemaInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}