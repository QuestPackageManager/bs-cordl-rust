#[cfg(feature = "System+Xml+Schema+XsdValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct XsdValidator {
    __cordl_parent: crate::System::Xml::Schema::BaseValidator,
    pub startIDConstraint: i32,
    pub validationStack: *mut crate::System::Xml::HWStack,
    pub attPresence: *mut crate::System::Collections::Hashtable,
    pub nsManager: *mut crate::System::Xml::XmlNamespaceManager,
    pub bManageNamespaces: bool,
    pub IDs: *mut crate::System::Collections::Hashtable,
    pub idRefListHead: *mut crate::System::Xml::Schema::IdRefNode,
    pub inlineSchemaParser: *mut crate::System::Xml::Schema::Parser,
    pub processContents: crate::System::Xml::Schema::XmlSchemaContentProcessing,
    pub NsXmlNs: *mut crate::System::String,
    pub NsXs: *mut crate::System::String,
    pub NsXsi: *mut crate::System::String,
    pub XsiType: *mut crate::System::String,
    pub XsiNil: *mut crate::System::String,
    pub XsiSchemaLocation: *mut crate::System::String,
    pub XsiNoNamespaceSchemaLocation: *mut crate::System::String,
    pub XsdSchema: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Schema+XsdValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdValidator =>
    "System.Xml.Schema"."XsdValidator"
);
#[cfg(feature = "System+Xml+Schema+XsdValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::XsdValidator {
    type Target = crate::System::Xml::Schema::BaseValidator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XsdValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdValidator")]
impl crate::System::Xml::Schema::XsdValidator {
    pub fn AddID(
        &mut self,
        name: *mut crate::System::String,
        node: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddID", (name, node))?;
        Ok(__cordl_ret)
    }
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
    pub fn AttributeIdentityConstraints(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        obj: *mut crate::System::Object,
        sobj: *mut crate::System::String,
        attdef: *mut crate::System::Xml::Schema::SchemaAttDef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttributeIdentityConstraints", (name, ns, obj, sobj, attdef))?;
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
    pub fn CheckValue(
        &mut self,
        value: *mut crate::System::String,
        attdef: *mut crate::System::Xml::Schema::SchemaAttDef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckValue", (value, attdef))?;
        Ok(__cordl_ret)
    }
    pub fn CompleteValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteValidation", ())?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndElementIdentityConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn FastGetElementDecl(
        &mut self,
        particle: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaElementDecl = __cordl_object
            .invoke("FastGetElementDecl", (particle))?;
        Ok(__cordl_ret)
    }
    pub fn FindId(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("FindId", (name))?;
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
    pub fn IsXSDRoot(
        &mut self,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXSDRoot", (localName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSchema(
        &mut self,
        uri: *mut crate::System::String,
        url: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSchema", (uri, url))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSchemaFromLocation(
        &mut self,
        uri: *mut crate::System::String,
        url: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSchemaFromLocation", (uri, url))?;
        Ok(__cordl_ret)
    }
    pub fn New_BaseValidator0(
        validator: *mut crate::System::Xml::Schema::BaseValidator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (validator))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlValidatingReaderImpl_XmlSchemaCollection_IValidationEventHandling1(
        reader: *mut crate::System::Xml::XmlValidatingReaderImpl,
        schemaCollection: *mut crate::System::Xml::Schema::XmlSchemaCollection,
        eventHandling: *mut crate::System::Xml::IValidationEventHandling,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader, schemaCollection, eventHandling))?;
        Ok(__cordl_object)
    }
    pub fn Pop(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessElement(
        &mut self,
        particle: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessElement", (particle))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessInlineSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessInlineSchema", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessTokenizedType(
        &mut self,
        ttype: crate::System::Xml::XmlTokenizedType,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessTokenizedType", (ttype, name))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessXsiAttributes(
        &mut self,
        xsiType: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::XmlQualifiedName,
        >,
        xsiNil: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessXsiAttributes", (xsiType, xsiNil))?;
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
    pub fn ThoroughGetElementDecl(
        &mut self,
        elementDecl: *mut crate::System::Xml::Schema::SchemaElementDecl,
        xsiType: *mut crate::System::Xml::XmlQualifiedName,
        xsiNil: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::SchemaElementDecl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaElementDecl = __cordl_object
            .invoke("ThoroughGetElementDecl", (elementDecl, xsiType, xsiNil))?;
        Ok(__cordl_ret)
    }
    pub fn UnWrapUnion(
        &mut self,
        typedValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("UnWrapUnion", (typedValue))?;
        Ok(__cordl_ret)
    }
    pub fn Validate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Validate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateChildElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ValidateChildElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateEndElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateEndStartElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateEndStartElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateStartElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateStartElement", ())?;
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
    pub fn _ctor_BaseValidator0(
        &mut self,
        validator: *mut crate::System::Xml::Schema::BaseValidator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (validator))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlValidatingReaderImpl_XmlSchemaCollection_IValidationEventHandling1(
        &mut self,
        reader: *mut crate::System::Xml::XmlValidatingReaderImpl,
        schemaCollection: *mut crate::System::Xml::Schema::XmlSchemaCollection,
        eventHandling: *mut crate::System::Xml::IValidationEventHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader, schemaCollection, eventHandling))?;
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
    pub fn get_IsInlineSchemaStarted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInlineSchemaStarted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PreserveWhitespace(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_PreserveWhitespace", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdValidator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XsdValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}