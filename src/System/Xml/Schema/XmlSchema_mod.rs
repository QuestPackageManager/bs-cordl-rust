#[cfg(feature = "System+Xml+Schema+XmlSchema")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchema {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaObject,
    pub attributeFormDefault: crate::System::Xml::Schema::XmlSchemaForm,
    pub elementFormDefault: crate::System::Xml::Schema::XmlSchemaForm,
    pub blockDefault: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub finalDefault: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub targetNs: *mut crate::System::String,
    pub version: *mut crate::System::String,
    pub includes: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    pub items: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    pub id: *mut crate::System::String,
    pub moreAttributes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::XmlAttribute,
    >,
    pub isCompiled: bool,
    pub isCompiledBySet: bool,
    pub isPreprocessed: bool,
    pub isRedefined: bool,
    pub errorCount: i32,
    pub attributes: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub attributeGroups: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub elements: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub types: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub groups: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub notations: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub identityConstraints: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub importedSchemas: *mut crate::System::Collections::ArrayList,
    pub importedNamespaces: *mut crate::System::Collections::ArrayList,
    pub schemaId: i32,
    pub baseUri: *mut crate::System::Uri,
    pub isChameleon: bool,
    pub ids: *mut crate::System::Collections::Hashtable,
    pub document: *mut crate::System::Xml::XmlDocument,
}
#[cfg(feature = "System+Xml+Schema+XmlSchema")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchema =>
    "System.Xml.Schema"."XmlSchema"
);
#[cfg(feature = "System+Xml+Schema+XmlSchema")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchema {
    type Target = crate::System::Xml::Schema::XmlSchemaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchema")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchema")]
impl crate::System::Xml::Schema::XmlSchema {
    pub fn get_TargetNamespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_TargetNamespace", ())?;
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
    pub fn set_FinalDefault(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FinalDefault", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ElementFormDefault(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaForm,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ElementFormDefault", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_BlockDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaDerivationMethod = __cordl_object
            .invoke("get_BlockDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsCompiledBySet(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsCompiledBySet", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Items(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection = __cordl_object
            .invoke("get_Items", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsChameleon(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsChameleon", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Elements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_Elements", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_BaseUri(
        &mut self,
        value: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BaseUri", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCompiledBySet(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompiledBySet", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsRedefined(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsRedefined", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AttributeFormDefault(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaForm,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AttributeFormDefault", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IdAttribute(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IdAttribute", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CompileSchemaInSet(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
        compilationSettings: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileSchemaInSet",
                (nameTable, eventHandler, compilationSettings),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DeepClone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("DeepClone", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_Attributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ErrorCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ErrorCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_TargetNamespace(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TargetNamespace", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_SchemaTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompileSchema(
        &mut self,
        xsc: *mut crate::System::Xml::Schema::XmlSchemaCollection,
        resolver: *mut crate::System::Xml::XmlResolver,
        schemaInfo: *mut crate::System::Xml::Schema::SchemaInfo,
        ns: *mut crate::System::String,
        validationEventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        CompileContentModel: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CompileSchema",
                (
                    xsc,
                    resolver,
                    schemaInfo,
                    ns,
                    validationEventHandler,
                    nameTable,
                    CompileContentModel,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_FinalDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaDerivationMethod = __cordl_object
            .invoke("get_FinalDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SchemaId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Document(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlDocument> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlDocument = __cordl_object
            .invoke("get_Document", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("get_BaseUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Version(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Version", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ImportedNamespaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_ImportedNamespaces", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetExternalSchemasList(
        &mut self,
        extList: *mut crate::System::Collections::IList,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetExternalSchemasList", (extList, schema))?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_AttributeGroups", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Id(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Id", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsRedefined(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsRedefined", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetIsCompiled(
        &mut self,
        isCompiled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsCompiled", (isCompiled))?;
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
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddAnnotation(
        &mut self,
        annotation: *mut crate::System::Xml::Schema::XmlSchemaAnnotation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAnnotation", (annotation))?;
        Ok(__cordl_ret)
    }
    pub fn get_Includes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection = __cordl_object
            .invoke("get_Includes", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsPreprocessed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsPreprocessed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPreprocessed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPreprocessed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ElementFormDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSchemaForm> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaForm = __cordl_object
            .invoke("get_ElementFormDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeFormDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSchemaForm> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaForm = __cordl_object
            .invoke("get_AttributeFormDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsChameleon(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsChameleon", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetUnhandledAttributes(
        &mut self,
        moreAttributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::XmlAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUnhandledAttributes", (moreAttributes))?;
        Ok(__cordl_ret)
    }
    pub fn get_ImportedSchemas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_ImportedSchemas", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Notations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_Notations", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Ids(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Hashtable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Hashtable = __cordl_object
            .invoke("get_Ids", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_BlockDefault(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BlockDefault", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Groups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_Groups", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IdentityConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_IdentityConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IdAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_IdAttribute", ())?;
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
#[cfg(feature = "System+Xml+Schema+XmlSchema")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlSchema {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
