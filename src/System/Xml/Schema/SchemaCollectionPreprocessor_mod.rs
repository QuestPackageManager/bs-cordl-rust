#[cfg(feature = "System+Xml+Schema+SchemaCollectionPreprocessor+Compositor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaCollectionPreprocessor_Compositor {
    Import = 2i32,
    Include = 1i32,
    Root = 0i32,
}
#[cfg(feature = "System+Xml+Schema+SchemaCollectionPreprocessor+Compositor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::SchemaCollectionPreprocessor_Compositor => "System.Xml.Schema"
    ."SchemaCollectionPreprocessor/Compositor"
);
#[cfg(feature = "System+Xml+Schema+SchemaCollectionPreprocessor")]
#[repr(C)]
#[derive(Debug)]
pub struct SchemaCollectionPreprocessor {
    __cordl_parent: crate::System::Xml::Schema::BaseProcessor,
    pub schema: *mut crate::System::Xml::Schema::XmlSchema,
    pub targetNamespace: *mut crate::System::String,
    pub buildinIncluded: bool,
    pub elementFormDefault: crate::System::Xml::Schema::XmlSchemaForm,
    pub attributeFormDefault: crate::System::Xml::Schema::XmlSchemaForm,
    pub blockDefault: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub finalDefault: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub schemaLocations: *mut crate::System::Collections::Hashtable,
    pub referenceNamespaces: *mut crate::System::Collections::Hashtable,
    pub Xmlns: *mut crate::System::String,
    pub xmlResolver: *mut crate::System::Xml::XmlResolver,
}
#[cfg(feature = "System+Xml+Schema+SchemaCollectionPreprocessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::SchemaCollectionPreprocessor => "System.Xml.Schema"
    ."SchemaCollectionPreprocessor"
);
#[cfg(feature = "System+Xml+Schema+SchemaCollectionPreprocessor")]
impl std::ops::Deref for crate::System::Xml::Schema::SchemaCollectionPreprocessor {
    type Target = crate::System::Xml::Schema::BaseProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaCollectionPreprocessor")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SchemaCollectionPreprocessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaCollectionPreprocessor")]
impl crate::System::Xml::Schema::SchemaCollectionPreprocessor {
    #[cfg(feature = "System+Xml+Schema+SchemaCollectionPreprocessor+Compositor")]
    pub type Compositor = crate::System::Xml::Schema::SchemaCollectionPreprocessor_Compositor;
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
    pub fn ValidateQNameAttribute(
        &mut self,
        xso: *mut crate::System::Xml::Schema::XmlSchemaObject,
        attributeName: *mut crate::System::String,
        value: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateQNameAttribute", (xso, attributeName, value))?;
        Ok(__cordl_ret)
    }
    pub fn PreprocessAnnotation(
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
    pub fn LoadExternals(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        xsc: *mut crate::System::Xml::Schema::XmlSchemaCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadExternals", (schema, xsc))?;
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
    pub fn ResolveSchemaLocationUri(
        &mut self,
        enclosingSchema: *mut crate::System::Xml::Schema::XmlSchema,
        location: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("ResolveSchemaLocationUri", (enclosingSchema, location))?;
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
    pub fn GetSchemaEntity(
        &mut self,
        ruri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetSchemaEntity", (ruri))?;
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
    pub fn PreprocessRedefine(
        &mut self,
        redefine: *mut crate::System::Xml::Schema::XmlSchemaRedefine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreprocessRedefine", (redefine))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameTable, schemaNames, eventHandler))?;
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
    pub fn Execute(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        targetNamespace: *mut crate::System::String,
        loadExternals: bool,
        xsc: *mut crate::System::Xml::Schema::XmlSchemaCollection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Execute", (schema, targetNamespace, loadExternals, xsc))?;
        Ok(__cordl_ret)
    }
    pub fn CountGroupSelfReference(
        &mut self,
        items: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CountGroupSelfReference", (items, name))?;
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
    pub fn Preprocess(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        targetNamespace: *mut crate::System::String,
        compositor: crate::System::Xml::Schema::SchemaCollectionPreprocessor_Compositor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Preprocess", (schema, targetNamespace, compositor))?;
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
    pub fn New(
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameTable, schemaNames, eventHandler))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaCollectionPreprocessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::SchemaCollectionPreprocessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
