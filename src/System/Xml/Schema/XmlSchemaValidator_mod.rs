#[cfg(feature = "System+Xml+Schema+XmlSchemaValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaValidator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub schemaSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
    pub validationFlags: crate::System::Xml::Schema::XmlSchemaValidationFlags,
    pub startIDConstraint: i32,
    pub isRoot: bool,
    pub rootHasSchema: bool,
    pub attrValid: bool,
    pub checkEntity: bool,
    pub compiledSchemaInfo: *mut crate::System::Xml::Schema::SchemaInfo,
    pub dtdSchemaInfo: *mut crate::System::Xml::IDtdInfo,
    pub validatedNamespaces: *mut crate::System::Collections::Hashtable,
    pub validationStack: *mut crate::System::Xml::HWStack,
    pub context: *mut crate::System::Xml::Schema::ValidationState,
    pub currentState: crate::System::Xml::Schema::ValidatorState,
    pub attPresence: *mut crate::System::Collections::Hashtable,
    pub wildID: *mut crate::System::Xml::Schema::SchemaAttDef,
    pub IDs: *mut crate::System::Collections::Hashtable,
    pub idRefListHead: *mut crate::System::Xml::Schema::IdRefNode,
    pub contextQName: *mut crate::System::Xml::XmlQualifiedName,
    pub NsXs: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXsi: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXmlNs: *mut quest_hook::libil2cpp::Il2CppString,
    pub NsXml: *mut quest_hook::libil2cpp::Il2CppString,
    pub partialValidationType: *mut crate::System::Xml::Schema::XmlSchemaObject,
    pub textValue: *mut crate::System::Text::StringBuilder,
    pub eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    pub validationEventSender: *mut quest_hook::libil2cpp::Il2CppObject,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub positionInfo: *mut crate::System::Xml::IXmlLineInfo,
    pub dummyPositionInfo: *mut crate::System::Xml::IXmlLineInfo,
    pub xmlResolver: *mut crate::System::Xml::XmlResolver,
    pub sourceUri: *mut crate::System::Uri,
    pub sourceUriString: *mut quest_hook::libil2cpp::Il2CppString,
    pub nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    pub processContents: crate::System::Xml::Schema::XmlSchemaContentProcessing,
    pub xsiTypeString: *mut quest_hook::libil2cpp::Il2CppString,
    pub xsiNilString: *mut quest_hook::libil2cpp::Il2CppString,
    pub xsiSchemaLocationString: *mut quest_hook::libil2cpp::Il2CppString,
    pub xsiNoNamespaceSchemaLocationString: *mut quest_hook::libil2cpp::Il2CppString,
    pub xmlCharType: crate::System::Xml::XmlCharType,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaValidator =>
    "System.Xml.Schema"."XmlSchemaValidator"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaValidator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaValidator")]
impl crate::System::Xml::Schema::XmlSchemaValidator {
    pub fn AddIdentityConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddIdentityConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddSchema(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSchema", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn AddXmlNamespaceSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddXmlNamespaceSchema", ())?;
        Ok(__cordl_ret)
    }
    pub fn AttributeIdentityConstraints(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        sobj: *mut quest_hook::libil2cpp::Il2CppString,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttributeIdentityConstraints", (name, ns, obj, sobj, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckAttributeValue(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        attdef: *mut crate::System::Xml::Schema::SchemaAttDef,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("CheckAttributeValue", (value, attdef))?;
        Ok(__cordl_ret)
    }
    pub fn CheckElementProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckElementProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckElementValue(
        &mut self,
        stringValue: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("CheckElementValue", (stringValue))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForwardRefs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckForwardRefs", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckIsXmlAttribute(
        &mut self,
        attQName: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaAttDef> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaAttDef = __cordl_object
            .invoke("CheckIsXmlAttribute", (attQName))?;
        Ok(__cordl_ret)
    }
    pub fn CheckMixedValueConstraint(
        &mut self,
        elementValue: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("CheckMixedValueConstraint", (elementValue))?;
        Ok(__cordl_ret)
    }
    pub fn CheckRequiredAttributes(
        &mut self,
        currentElementDecl: *mut crate::System::Xml::Schema::SchemaElementDecl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckRequiredAttributes", (currentElementDecl))?;
        Ok(__cordl_ret)
    }
    pub fn CheckStateTransition(
        &mut self,
        toState: crate::System::Xml::Schema::ValidatorState,
        methodName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckStateTransition", (toState, methodName))?;
        Ok(__cordl_ret)
    }
    pub fn CheckTokenizedTypes(
        &mut self,
        dtype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
        typedValue: *mut quest_hook::libil2cpp::Il2CppObject,
        attrValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckTokenizedTypes", (dtype, typedValue, attrValue))?;
        Ok(__cordl_ret)
    }
    pub fn CheckXsiTypeAndNil(
        &mut self,
        elementDecl: *mut crate::System::Xml::Schema::SchemaElementDecl,
        xsiType: *mut quest_hook::libil2cpp::Il2CppString,
        xsiNil: *mut quest_hook::libil2cpp::Il2CppString,
        declFound: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaElementDecl = __cordl_object
            .invoke("CheckXsiTypeAndNil", (elementDecl, xsiType, xsiNil, declFound))?;
        Ok(__cordl_ret)
    }
    pub fn ClearPSVI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearPSVI", ())?;
        Ok(__cordl_ret)
    }
    pub fn ElementIdentityConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ElementIdentityConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn EndElementIdentityConstraints(
        &mut self,
        typedValue: *mut quest_hook::libil2cpp::Il2CppObject,
        stringValue: *mut quest_hook::libil2cpp::Il2CppString,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "EndElementIdentityConstraints",
                (typedValue, stringValue, datatype),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndValidation", ())?;
        Ok(__cordl_ret)
    }
    pub fn FastGetElementDecl(
        &mut self,
        elementName: *mut crate::System::Xml::XmlQualifiedName,
        particle: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaElementDecl = __cordl_object
            .invoke("FastGetElementDecl", (elementName, particle))?;
        Ok(__cordl_ret)
    }
    pub fn FindId(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("FindId", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetConcatenatedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetConcatenatedValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultAttributePrefix(
        &mut self,
        attributeNS: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetDefaultAttributePrefix", (attributeNS))?;
        Ok(__cordl_ret)
    }
    pub fn GetSchemaElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("GetSchemaElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSubstitutionGroupHead(
        &mut self,
        member: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("GetSubstitutionGroupHead", (member))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeName(
        &mut self,
        decl: *mut crate::System::Xml::Schema::SchemaDeclBase,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetTypeName", (decl))?;
        Ok(__cordl_ret)
    }
    pub fn GetUnspecifiedDefaultAttributes(
        &mut self,
        defaultAttributes: *mut crate::System::Collections::ArrayList,
        createNodeData: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetUnspecifiedDefaultAttributes",
                (defaultAttributes, createNodeData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize_XmlSchemaObject1(
        &mut self,
        partialValidationType: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (partialValidationType))?;
        Ok(__cordl_ret)
    }
    pub fn InternalValidateEndElement(
        &mut self,
        schemaInfo: *mut crate::System::Xml::Schema::XmlSchemaInfo,
        typedValue: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("InternalValidateEndElement", (schemaInfo, typedValue))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSchema(
        &mut self,
        uri: *mut quest_hook::libil2cpp::Il2CppString,
        url: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSchema", (uri, url))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemas: *mut crate::System::Xml::Schema::XmlSchemaSet,
        namespaceResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
        validationFlags: crate::System::Xml::Schema::XmlSchemaValidationFlags,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (nameTable, schemas, namespaceResolver, validationFlags),
            )?;
        Ok(__cordl_object)
    }
    pub fn Pop(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessEntity(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEntity", (name))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessSchemaLocations(
        &mut self,
        xsiSchemaLocation: *mut quest_hook::libil2cpp::Il2CppString,
        xsiNoNamespaceSchemaLocation: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessSchemaLocations",
                (xsiSchemaLocation, xsiNoNamespaceSchemaLocation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessTokenizedType(
        &mut self,
        ttype: crate::System::Xml::XmlTokenizedType,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        attrValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessTokenizedType", (ttype, name, attrValue))?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
        elementName: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (elementName))?;
        Ok(__cordl_ret)
    }
    pub fn RecompileSchemaSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecompileSchemaSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn SaveTextValue(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveTextValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString0(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppArray1(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppArray_Exception5(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppArray_Exception_XmlSeverityType4(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        innerException: *mut crate::System::Exception,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args, innerException, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppString2(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        arg: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, arg))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppString_Il2CppString3(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        arg1: *mut quest_hook::libil2cpp::Il2CppString,
        arg2: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppString_XmlSeverityType8(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        msg: *mut quest_hook::libil2cpp::Il2CppString,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, msg, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaException7(
        &mut self,
        e: *mut crate::System::Xml::Schema::XmlSchemaException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaValidationException6(
        &mut self,
        e: *mut crate::System::Xml::Schema::XmlSchemaValidationException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaValidationException_XmlSeverityType9(
        &mut self,
        e: *mut crate::System::Xml::Schema::XmlSchemaValidationException,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SetDtdSchemaInfo(
        &mut self,
        dtdSchemaInfo: *mut crate::System::Xml::IDtdInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDtdSchemaInfo", (dtdSchemaInfo))?;
        Ok(__cordl_ret)
    }
    pub fn SkipToEndElement(
        &mut self,
        schemaInfo: *mut crate::System::Xml::Schema::XmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipToEndElement", (schemaInfo))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowDeclNotFoundWarningOrError(
        &mut self,
        declFound: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowDeclNotFoundWarningOrError", (declFound))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateAtomicValue_Il2CppObject1(
        &mut self,
        parsedValue: *mut quest_hook::libil2cpp::Il2CppObject,
        memberType: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ValidateAtomicValue", (parsedValue, memberType))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateAtomicValue_Il2CppString0(
        &mut self,
        stringValue: *mut quest_hook::libil2cpp::Il2CppString,
        memberType: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ValidateAtomicValue", (stringValue, memberType))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateAttribute_Il2CppString_XmlSchemaInfo1(
        &mut self,
        lName: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
        attributeValueGetter: *mut crate::System::Xml::Schema::XmlValueGetter,
        attributeStringValue: *mut quest_hook::libil2cpp::Il2CppString,
        schemaInfo: *mut crate::System::Xml::Schema::XmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke(
                "ValidateAttribute",
                (lName, ns, attributeValueGetter, attributeStringValue, schemaInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ValidateAttribute_XmlSchemaInfo0(
        &mut self,
        localName: *mut quest_hook::libil2cpp::Il2CppString,
        namespaceUri: *mut quest_hook::libil2cpp::Il2CppString,
        attributeValue: *mut crate::System::Xml::Schema::XmlValueGetter,
        schemaInfo: *mut crate::System::Xml::Schema::XmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke(
                "ValidateAttribute",
                (localName, namespaceUri, attributeValue, schemaInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ValidateElement(
        &mut self,
        localName: *mut quest_hook::libil2cpp::Il2CppString,
        namespaceUri: *mut quest_hook::libil2cpp::Il2CppString,
        schemaInfo: *mut crate::System::Xml::Schema::XmlSchemaInfo,
        xsiType: *mut quest_hook::libil2cpp::Il2CppString,
        xsiNil: *mut quest_hook::libil2cpp::Il2CppString,
        xsiSchemaLocation: *mut quest_hook::libil2cpp::Il2CppString,
        xsiNoNamespaceSchemaLocation: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ValidateElement",
                (
                    localName,
                    namespaceUri,
                    schemaInfo,
                    xsiType,
                    xsiNil,
                    xsiSchemaLocation,
                    xsiNoNamespaceSchemaLocation,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ValidateElementContext(
        &mut self,
        elementName: *mut crate::System::Xml::XmlQualifiedName,
        invalidElementInContext: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ValidateElementContext", (elementName, invalidElementInContext))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateEndElement(
        &mut self,
        schemaInfo: *mut crate::System::Xml::Schema::XmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ValidateEndElement", (schemaInfo))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateEndOfAttributes(
        &mut self,
        schemaInfo: *mut crate::System::Xml::Schema::XmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateEndOfAttributes", (schemaInfo))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateStartElementIdentityConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateStartElementIdentityConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateText_Il2CppString_XmlValueGetter1(
        &mut self,
        elementStringValue: *mut quest_hook::libil2cpp::Il2CppString,
        elementValueGetter: *mut crate::System::Xml::Schema::XmlValueGetter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateText", (elementStringValue, elementValueGetter))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateText_XmlValueGetter0(
        &mut self,
        elementValue: *mut crate::System::Xml::Schema::XmlValueGetter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateText", (elementValue))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateWhitespace_Il2CppString_XmlValueGetter1(
        &mut self,
        elementStringValue: *mut quest_hook::libil2cpp::Il2CppString,
        elementValueGetter: *mut crate::System::Xml::Schema::XmlValueGetter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateWhitespace", (elementStringValue, elementValueGetter))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateWhitespace_XmlValueGetter0(
        &mut self,
        elementValue: *mut crate::System::Xml::Schema::XmlValueGetter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateWhitespace", (elementValue))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemas: *mut crate::System::Xml::Schema::XmlSchemaSet,
        namespaceResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
        validationFlags: crate::System::Xml::Schema::XmlSchemaValidationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameTable, schemas, namespaceResolver, validationFlags))?;
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
    pub fn get_CurrentContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaContentType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaContentType = __cordl_object
            .invoke("get_CurrentContentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasIdentityConstraints(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasIdentityConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasSchema(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasSchema", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProcessIdentityConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ProcessIdentityConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProcessSchemaHints(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ProcessSchemaHints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReportValidationWarnings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ReportValidationWarnings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchemaSet> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaSet = __cordl_object
            .invoke("get_SchemaSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StrictlyAssessed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_StrictlyAssessed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValidationFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaValidationFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaValidationFlags = __cordl_object
            .invoke("get_ValidationFlags", ())?;
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
    pub fn set_LineInfoProvider(
        &mut self,
        value: *mut crate::System::Xml::IXmlLineInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LineInfoProvider", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SourceUri(
        &mut self,
        value: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SourceUri", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ValidationEventSender(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ValidationEventSender", (value))?;
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
#[cfg(feature = "System+Xml+Schema+XmlSchemaValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
