#[cfg(feature = "System+Xml+Schema+Preprocessor")]
#[repr(C)]
#[derive(Debug)]
pub struct Preprocessor {
    __cordl_parent: crate::System::Xml::Schema::BaseProcessor,
    pub Xmlns: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXsi: *mut quest_hook::libil2cpp::Il2CppString,
    pub targetNamespace: *mut quest_hook::libil2cpp::Il2CppString,
    pub rootSchema: *mut crate::System::Xml::Schema::XmlSchema,
    pub currentSchema: *mut crate::System::Xml::Schema::XmlSchema,
    pub elementFormDefault: crate::System::Xml::Schema::XmlSchemaForm,
    pub attributeFormDefault: crate::System::Xml::Schema::XmlSchemaForm,
    pub blockDefault: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub finalDefault: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub schemaLocations: *mut crate::System::Collections::Hashtable,
    pub chameleonSchemas: *mut crate::System::Collections::Hashtable,
    pub referenceNamespaces: *mut crate::System::Collections::Hashtable,
    pub processedExternals: *mut crate::System::Collections::Hashtable,
    pub lockList: *mut crate::System::Collections::SortedList,
    pub readerSettings: *mut crate::System::Xml::XmlReaderSettings,
    pub rootSchemaForRedefine: *mut crate::System::Xml::Schema::XmlSchema,
    pub redefinedList: *mut crate::System::Collections::ArrayList,
    pub xmlResolver: *mut crate::System::Xml::XmlResolver,
}
#[cfg(feature = "System+Xml+Schema+Preprocessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Preprocessor =>
    "System.Xml.Schema"."Preprocessor"
);
#[cfg(feature = "System+Xml+Schema+Preprocessor")]
impl std::ops::Deref for crate::System::Xml::Schema::Preprocessor {
    type Target = crate::System::Xml::Schema::BaseProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Preprocessor")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Preprocessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Preprocessor")]
impl crate::System::Xml::Schema::Preprocessor {
    pub fn BuildRefNamespaces(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildRefNamespaces", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn BuildSchemaList(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildSchemaList", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn CheckRefinedAttributeGroup(
        &mut self,
        attributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckRefinedAttributeGroup", (attributeGroup))?;
        Ok(__cordl_ret)
    }
    pub fn CheckRefinedComplexType(
        &mut self,
        ctype: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckRefinedComplexType", (ctype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckRefinedGroup(
        &mut self,
        group: *mut crate::System::Xml::Schema::XmlSchemaGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckRefinedGroup", (group))?;
        Ok(__cordl_ret)
    }
    pub fn CheckRefinedSimpleType(
        &mut self,
        stype: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckRefinedSimpleType", (stype))?;
        Ok(__cordl_ret)
    }
    pub fn Cleanup(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn CleanupRedefine(
        &mut self,
        include: *mut crate::System::Xml::Schema::XmlSchemaExternal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupRedefine", (include))?;
        Ok(__cordl_ret)
    }
    pub fn CopyIncludedComponents(
        &mut self,
        includedSchema: *mut crate::System::Xml::Schema::XmlSchema,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyIncludedComponents", (includedSchema, schema))?;
        Ok(__cordl_ret)
    }
    pub fn CountGroupSelfReference(
        &mut self,
        items: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        name: *mut crate::System::Xml::XmlQualifiedName,
        redefined: *mut crate::System::Xml::Schema::XmlSchemaGroup,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CountGroupSelfReference", (items, name, redefined))?;
        Ok(__cordl_ret)
    }
    pub fn Execute(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        targetNamespace: *mut quest_hook::libil2cpp::Il2CppString,
        loadExternals: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Execute", (schema, targetNamespace, loadExternals))?;
        Ok(__cordl_ret)
    }
    pub fn GetChameleonSchema(
        &mut self,
        targetNamespace: *mut quest_hook::libil2cpp::Il2CppString,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("GetChameleonSchema", (targetNamespace, schema))?;
        Ok(__cordl_ret)
    }
    pub fn GetIncludedSet(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        includesList: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIncludedSet", (schema, includesList))?;
        Ok(__cordl_ret)
    }
    pub fn GetSchemaEntity(
        &mut self,
        ruri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetSchemaEntity", (ruri))?;
        Ok(__cordl_ret)
    }
    pub fn LoadExternals(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadExternals", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
        compilationSettings: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (nameTable, schemaNames, eventHandler, compilationSettings),
            )?;
        Ok(__cordl_object)
    }
    pub fn ParseUri(
        &mut self,
        uri: *mut quest_hook::libil2cpp::Il2CppString,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        sourceSchemaObject: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseUri", (uri, code, sourceSchemaObject))?;
        Ok(__cordl_ret)
    }
    pub fn Preprocess(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        targetNamespace: *mut quest_hook::libil2cpp::Il2CppString,
        imports: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Preprocess", (schema, targetNamespace, imports))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessAnnotation_XmlSchemaAnnotation1(
        &mut self,
        annotation: *mut crate::System::Xml::Schema::XmlSchemaAnnotation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessAnnotation", (annotation))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessAnnotation_XmlSchemaObject0(
        &mut self,
        schemaObject: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessAnnotation", (schemaObject))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessAttribute(
        &mut self,
        attribute: *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessAttribute", (attribute))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessAttributeContent(
        &mut self,
        attribute: *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessAttributeContent", (attribute))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessAttributeGroup(
        &mut self,
        attributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessAttributeGroup", (attributeGroup))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessAttributes(
        &mut self,
        attributes: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        anyAttribute: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        parent: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessAttributes", (attributes, anyAttribute, parent))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessComplexType(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        local: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessComplexType", (complexType, local))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessElement(
        &mut self,
        element: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessElement", (element))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessElementContent(
        &mut self,
        element: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessElementContent", (element))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessGroup(
        &mut self,
        group: *mut crate::System::Xml::Schema::XmlSchemaGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessGroup", (group))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessIdentityConstraint(
        &mut self,
        constraint: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessIdentityConstraint", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessLocalAttribute(
        &mut self,
        attribute: *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessLocalAttribute", (attribute))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessLocalElement(
        &mut self,
        element: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessLocalElement", (element))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessNotation(
        &mut self,
        notation: *mut crate::System::Xml::Schema::XmlSchemaNotation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessNotation", (notation))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessParticle(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessParticle", (particle))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessRedefine(
        &mut self,
        redefineEntry: *mut crate::System::Xml::Schema::RedefineEntry,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessRedefine", (redefineEntry))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessSimpleType(
        &mut self,
        simpleType: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
        local: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessSimpleType", (simpleType, local))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveSchemaLocationUri(
        &mut self,
        enclosingSchema: *mut crate::System::Xml::Schema::XmlSchema,
        location: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("ResolveSchemaLocationUri", (enclosingSchema, location))?;
        Ok(__cordl_ret)
    }
    pub fn SetParent(
        &mut self,
        child: *mut crate::System::Xml::Schema::XmlSchemaObject,
        parent: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParent", (child, parent))?;
        Ok(__cordl_ret)
    }
    pub fn SetSchemaDefaults(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSchemaDefaults", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateIdAttribute(
        &mut self,
        xso: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateIdAttribute", (xso))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateNameAttribute(
        &mut self,
        xso: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateNameAttribute", (xso))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateQNameAttribute(
        &mut self,
        xso: *mut crate::System::Xml::Schema::XmlSchemaObject,
        attributeName: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateQNameAttribute", (xso, attributeName, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
        compilationSettings: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (nameTable, schemaNames, eventHandler, compilationSettings),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_RootSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("get_RootSchema", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ChameleonSchemas(
        &mut self,
        value: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ChameleonSchemas", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ReaderSettings(
        &mut self,
        value: *mut crate::System::Xml::XmlReaderSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReaderSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SchemaLocations(
        &mut self,
        value: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SchemaLocations", (value))?;
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
#[cfg(feature = "System+Xml+Schema+Preprocessor")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Preprocessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
