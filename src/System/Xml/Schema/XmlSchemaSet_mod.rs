#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaSet {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    pub schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
    pub schemas: quest_hook::libil2cpp::Gc<crate::System::Collections::SortedList>,
    pub internalEventHandler: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::ValidationEventHandler,
    >,
    pub eventHandler: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::ValidationEventHandler,
    >,
    pub isCompiled: bool,
    pub schemaLocations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub chameleonSchemas: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub targetNamespaces: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub compileAll: bool,
    pub cachedCompiledInfo: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SchemaInfo,
    >,
    pub readerSettings: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReaderSettings>,
    pub schemaForSchema: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchema,
    >,
    pub compilationSettings: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    >,
    pub elements: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub attributes: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub schemaTypes: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub substitutionGroups: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub typeExtensions: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub internalSyncObject: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaSet =>
    "System.Xml.Schema"."XmlSchemaSet"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaSet {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSchemaToSet", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        >,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        item: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddToTable", (table, qname, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_Il2CppString_XmlReader_Hashtable3(
        &mut self,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        validatedNamespaces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Hashtable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (targetNamespace, reader, validatedNamespaces))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_Il2CppString_XmlSchema2(
        &mut self,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = __cordl_object.invoke("Add", (targetNamespace, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_XmlSchema1(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = __cordl_object.invoke("Add", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_XmlSchemaSet0(
        &mut self,
        schemas: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (schemas))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearTables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Compile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Compile", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        &mut self,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (targetNamespace))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromCompiledSet(
        &mut self,
        otherSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFromCompiledSet", (otherSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        &mut self,
        schemas: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Xml::Schema::XmlSchema,
            >,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (schemas, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindSchemaByNSAndUrl(
        &mut self,
        schemaUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        locationsTable: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Collections::DictionaryEntry,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = __cordl_object
            .invoke("FindSchemaByNSAndUrl", (schemaUri, ns, locationsTable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEventHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationEventHandler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        > = __cordl_object.invoke("GetEventHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver> = __cordl_object
            .invoke("GetResolver", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSchemaByUri(
        &mut self,
        schemaUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        schema: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchema,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetSchemaByUri", (schemaUri, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSchemaNames(
        &mut self,
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SchemaNames,
        > = __cordl_object.invoke("GetSchemaNames", (nt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTargetNamespace(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetTargetNamespace", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalValidationCallback(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalValidationCallback", (sender, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSchemaLoaded(
        &mut self,
        schemaUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        schema: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchema,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSchemaLoaded", (schemaUri, targetNamespace, schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_XmlNameTable1(
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameTable))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseSchema(
        &mut self,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = __cordl_object.invoke("ParseSchema", (targetNamespace, reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessSchema(
        &mut self,
        schema: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchema,
        >,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PreprocessSchema", (schema, targetNamespace))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessNewSubstitutionGroups(
        &mut self,
        substitutionGroupsTable: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        >,
        resolve: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewSubstitutionGroups", (substitutionGroupsTable, resolve))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        forceCompile: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = __cordl_object.invoke("Remove", (schema, forceCompile))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveRecursive(
        &mut self,
        schemaToRemove: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RemoveRecursive", (schemaToRemove))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSchemaFromCaches(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSchemaFromCaches", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSchemaFromGlobalTables(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSchemaFromGlobalTables", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reprocess(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = __cordl_object.invoke("Reprocess", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveSubstitutionGroup(
        &mut self,
        substitutionGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSubstitutionGroup,
        >,
        substTable: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveSubstitutionGroup", (substitutionGroup, substTable))?;
        Ok(__cordl_ret.into())
    }
    pub fn Schemas_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("Schemas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Schemas_Il2CppString1(
        &mut self,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("Schemas", (targetNamespace))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent(
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
    pub fn VerifyTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VerifyTables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlNameTable1(
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
    pub fn add_ValidationEventHandler(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_ValidationEventHandler", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompilationSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaCompilationSettings,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaCompilationSettings,
        > = __cordl_object.invoke("get_CompilationSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompiledInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SchemaInfo,
        > = __cordl_object.invoke("get_CompiledInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GlobalAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        > = __cordl_object.invoke("get_GlobalAttributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GlobalElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        > = __cordl_object.invoke("get_GlobalElements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GlobalTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        > = __cordl_object.invoke("get_GlobalTypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalSyncObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_InternalSyncObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompiled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompiled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReaderSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReaderSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlReaderSettings,
        > = __cordl_object.invoke("get_ReaderSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SchemaLocations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Hashtable,
        > = __cordl_object.invoke("get_SchemaLocations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SortedSchemas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::SortedList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::SortedList,
        > = __cordl_object.invoke("get_SortedSchemas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SubstitutionGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        > = __cordl_object.invoke("get_SubstitutionGroups", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        > = __cordl_object.invoke("get_TypeExtensions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_ValidationEventHandler(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_ValidationEventHandler", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CompilationSettings(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaCompilationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CompilationSettings", (value))?;
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
#[cfg(feature = "System+Xml+Schema+XmlSchemaSet")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlSchemaSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
