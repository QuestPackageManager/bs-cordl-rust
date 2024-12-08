#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaSet {
    __cordl_parent: crate::System::Object,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
    pub schemas: *mut crate::System::Collections::SortedList,
    pub internalEventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    pub eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    pub isCompiled: bool,
    pub schemaLocations: *mut crate::System::Collections::Hashtable,
    pub chameleonSchemas: *mut crate::System::Collections::Hashtable,
    pub targetNamespaces: *mut crate::System::Collections::Hashtable,
    pub compileAll: bool,
    pub cachedCompiledInfo: *mut crate::System::Xml::Schema::SchemaInfo,
    pub readerSettings: *mut crate::System::Xml::XmlReaderSettings,
    pub schemaForSchema: *mut crate::System::Xml::Schema::XmlSchema,
    pub compilationSettings: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    pub elements: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub attributes: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub schemaTypes: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub substitutionGroups: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub typeExtensions: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub internalSyncObject: *mut crate::System::Object,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaSet =>
    "System.Xml.Schema"."XmlSchemaSet"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaSet {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
impl crate::System::Xml::Schema::XmlSchemaSet {
    pub fn AddSchemaToSet(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSchemaToSet", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn AddToTable(
        &mut self,
        table: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
        qname: *mut crate::System::Xml::XmlQualifiedName,
        item: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddToTable", (table, qname, item))?;
        Ok(__cordl_ret)
    }
    pub fn Add_String_XmlReader_Hashtable3(
        &mut self,
        targetNamespace: *mut crate::System::String,
        reader: *mut crate::System::Xml::XmlReader,
        validatedNamespaces: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (targetNamespace, reader, validatedNamespaces))?;
        Ok(__cordl_ret)
    }
    pub fn Add_String_XmlSchema2(
        &mut self,
        targetNamespace: *mut crate::System::String,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("Add", (targetNamespace, schema))?;
        Ok(__cordl_ret)
    }
    pub fn Add_XmlSchema1(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("Add", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn Add_XmlSchemaSet0(
        &mut self,
        schemas: *mut crate::System::Xml::Schema::XmlSchemaSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (schemas))?;
        Ok(__cordl_ret)
    }
    pub fn ClearTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearTables", ())?;
        Ok(__cordl_ret)
    }
    pub fn Compile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Compile", ())?;
        Ok(__cordl_ret)
    }
    pub fn Contains(
        &mut self,
        targetNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (targetNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn CopyFromCompiledSet(
        &mut self,
        otherSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFromCompiledSet", (otherSet))?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo(
        &mut self,
        schemas: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::XmlSchema,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (schemas, index))?;
        Ok(__cordl_ret)
    }
    pub fn FindSchemaByNSAndUrl(
        &mut self,
        schemaUri: *mut crate::System::Uri,
        ns: *mut crate::System::String,
        locationsTable: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Collections::DictionaryEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("FindSchemaByNSAndUrl", (schemaUri, ns, locationsTable))?;
        Ok(__cordl_ret)
    }
    pub fn GetEventHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::ValidationEventHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::ValidationEventHandler = __cordl_object
            .invoke("GetEventHandler", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlResolver> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlResolver = __cordl_object
            .invoke("GetResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSchemaByUri(
        &mut self,
        schemaUri: *mut crate::System::Uri,
        schema: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchema,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetSchemaByUri", (schemaUri, schema))?;
        Ok(__cordl_ret)
    }
    pub fn GetSchemaNames(
        &mut self,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaNames> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaNames = __cordl_object
            .invoke("GetSchemaNames", (nt))?;
        Ok(__cordl_ret)
    }
    pub fn GetTargetNamespace(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTargetNamespace", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn InternalValidationCallback(
        &mut self,
        sender: *mut crate::System::Object,
        e: *mut crate::System::Xml::Schema::ValidationEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalValidationCallback", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn IsSchemaLoaded(
        &mut self,
        schemaUri: *mut crate::System::Uri,
        targetNamespace: *mut crate::System::String,
        schema: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchema,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSchemaLoaded", (schemaUri, targetNamespace, schema))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_XmlNameTable1(
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameTable))?;
        Ok(__cordl_object)
    }
    pub fn ParseSchema(
        &mut self,
        targetNamespace: *mut crate::System::String,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("ParseSchema", (targetNamespace, reader))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessSchema(
        &mut self,
        schema: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchema,
        >,
        targetNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PreprocessSchema", (schema, targetNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessNewSubstitutionGroups(
        &mut self,
        substitutionGroupsTable: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
        resolve: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewSubstitutionGroups", (substitutionGroupsTable, resolve))?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        forceCompile: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("Remove", (schema, forceCompile))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveRecursive(
        &mut self,
        schemaToRemove: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RemoveRecursive", (schemaToRemove))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveSchemaFromCaches(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSchemaFromCaches", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveSchemaFromGlobalTables(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSchemaFromGlobalTables", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn Reprocess(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("Reprocess", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveSubstitutionGroup(
        &mut self,
        substitutionGroup: *mut crate::System::Xml::Schema::XmlSchemaSubstitutionGroup,
        substTable: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveSubstitutionGroup", (substitutionGroup, substTable))?;
        Ok(__cordl_ret)
    }
    pub fn Schemas_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("Schemas", ())?;
        Ok(__cordl_ret)
    }
    pub fn Schemas_String1(
        &mut self,
        targetNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("Schemas", (targetNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent(
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
    pub fn VerifyTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VerifyTables", ())?;
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
    pub fn _ctor_XmlNameTable1(
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
    pub fn add_ValidationEventHandler(
        &mut self,
        value: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_ValidationEventHandler", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_CompilationSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings = __cordl_object
            .invoke("get_CompilationSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CompiledInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaInfo = __cordl_object
            .invoke("get_CompiledInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GlobalAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_GlobalAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GlobalElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_GlobalElements", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GlobalTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_GlobalTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalSyncObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_InternalSyncObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCompiled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompiled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReaderSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlReaderSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlReaderSettings = __cordl_object
            .invoke("get_ReaderSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaLocations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Hashtable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Hashtable = __cordl_object
            .invoke("get_SchemaLocations", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SortedSchemas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::SortedList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::SortedList = __cordl_object
            .invoke("get_SortedSchemas", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SubstitutionGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_SubstitutionGroups", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectTable = __cordl_object
            .invoke("get_TypeExtensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_ValidationEventHandler(
        &mut self,
        value: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_ValidationEventHandler", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CompilationSettings(
        &mut self,
        value: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CompilationSettings", (value))?;
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
#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlSchemaSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
