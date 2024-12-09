#[cfg(feature = "System+Xml+Schema+XmlSchemaInference")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaInference {
    __cordl_parent: crate::System::Object,
    pub rootSchema: *mut crate::System::Xml::Schema::XmlSchema,
    pub schemaSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
    pub xtr: *mut crate::System::Xml::XmlReader,
    pub nametable: *mut crate::System::Xml::NameTable,
    pub TargetNamespace: *mut crate::System::String,
    pub NamespaceManager: *mut crate::System::Xml::XmlNamespaceManager,
    pub schemaList: *mut crate::System::Collections::ArrayList,
    pub occurrence: crate::System::Xml::Schema::XmlSchemaInference_InferenceOption,
    pub typeInference: crate::System::Xml::Schema::XmlSchemaInference_InferenceOption,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaInference =>
    "System.Xml.Schema"."XmlSchemaInference"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaInference {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaInference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference")]
impl crate::System::Xml::Schema::XmlSchemaInference {
    #[cfg(feature = "System+Xml+Schema+XmlSchemaInference+InferenceOption")]
    pub type InferenceOption = crate::System::Xml::Schema::XmlSchemaInference_InferenceOption;
    pub fn AddAttribute(
        &mut self,
        localName: *mut crate::System::String,
        prefix: *mut crate::System::String,
        childURI: *mut crate::System::String,
        attrValue: *mut crate::System::String,
        bCreatingNewType: bool,
        parentSchema: *mut crate::System::Xml::Schema::XmlSchema,
        addLocation: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        compiledAttributes: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAttribute = __cordl_object
            .invoke(
                "AddAttribute",
                (
                    localName,
                    prefix,
                    childURI,
                    attrValue,
                    bCreatingNewType,
                    parentSchema,
                    addLocation,
                    compiledAttributes,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddElement(
        &mut self,
        localName: *mut crate::System::String,
        prefix: *mut crate::System::String,
        childURI: *mut crate::System::String,
        parentSchema: *mut crate::System::Xml::Schema::XmlSchema,
        addLocation: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        positionWithinCollection: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke(
                "AddElement",
                (
                    localName,
                    prefix,
                    childURI,
                    parentSchema,
                    addLocation,
                    positionWithinCollection,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CheckSimpleContentExtension(
        &mut self,
        ct: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaSimpleContentExtension = __cordl_object
            .invoke("CheckSimpleContentExtension", (ct))?;
        Ok(__cordl_ret)
    }
    pub fn CreateNewElementforChoice(
        &mut self,
        copyElement: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("CreateNewElementforChoice", (copyElement))?;
        Ok(__cordl_ret)
    }
    pub fn CreateXmlSchema(
        &mut self,
        targetNS: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("CreateXmlSchema", (targetNS))?;
        Ok(__cordl_ret)
    }
    pub fn FindAttribute(
        &mut self,
        attributes: *mut crate::System::Collections::ICollection,
        attrName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAttribute = __cordl_object
            .invoke("FindAttribute", (attributes, attrName))?;
        Ok(__cordl_ret)
    }
    pub fn FindAttributeRef(
        &mut self,
        attributes: *mut crate::System::Collections::ICollection,
        attributeName: *mut crate::System::String,
        nsURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAttribute = __cordl_object
            .invoke("FindAttributeRef", (attributes, attributeName, nsURI))?;
        Ok(__cordl_ret)
    }
    pub fn FindElement(
        &mut self,
        elements: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        elementName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("FindElement", (elements, elementName))?;
        Ok(__cordl_ret)
    }
    pub fn FindElementRef(
        &mut self,
        elements: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        elementName: *mut crate::System::String,
        nsURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("FindElementRef", (elements, elementName, nsURI))?;
        Ok(__cordl_ret)
    }
    pub fn FindGlobalElement(
        &mut self,
        namespaceURI: *mut crate::System::String,
        localName: *mut crate::System::String,
        parentSchema: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchema,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("FindGlobalElement", (namespaceURI, localName, parentSchema))?;
        Ok(__cordl_ret)
    }
    pub fn FindMatchingElement(
        &mut self,
        bCreatingNewType: bool,
        xtr: *mut crate::System::Xml::XmlReader,
        ct: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        lastUsedSeqItem: quest_hook::libil2cpp::ByRefMut<i32>,
        bParticleChanged: quest_hook::libil2cpp::ByRefMut<bool>,
        parentSchema: *mut crate::System::Xml::Schema::XmlSchema,
        setMaxoccurs: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke(
                "FindMatchingElement",
                (
                    bCreatingNewType,
                    xtr,
                    ct,
                    lastUsedSeqItem,
                    bParticleChanged,
                    parentSchema,
                    setMaxoccurs,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetEffectiveSchemaType(
        &mut self,
        elem: *mut crate::System::Xml::Schema::XmlSchemaElement,
        bCreatingNewType: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaType = __cordl_object
            .invoke("GetEffectiveSchemaType", (elem, bCreatingNewType))?;
        Ok(__cordl_ret)
    }
    pub fn InferElement(
        &mut self,
        xse: *mut crate::System::Xml::Schema::XmlSchemaElement,
        bCreatingNewType: bool,
        parentSchema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InferElement", (xse, bCreatingNewType, parentSchema))?;
        Ok(__cordl_ret)
    }
    pub fn InferSchema(
        &mut self,
        instanceDocument: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchemaSet> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaSet = __cordl_object
            .invoke("InferSchema", (instanceDocument))?;
        Ok(__cordl_ret)
    }
    pub fn InferSchema1(
        &mut self,
        instanceDocument: *mut crate::System::Xml::XmlReader,
        schemas: *mut crate::System::Xml::Schema::XmlSchemaSet,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchemaSet> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaSet = __cordl_object
            .invoke("InferSchema1", (instanceDocument, schemas))?;
        Ok(__cordl_ret)
    }
    pub fn MakeExistingAttributesOptional(
        &mut self,
        ct: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        attributesInInstance: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MakeExistingAttributesOptional", (ct, attributesInInstance))?;
        Ok(__cordl_ret)
    }
    pub fn MoveAttributes_XmlSchemaComplexType_XmlSchemaSimpleContentExtension__cordl_bool1(
        &mut self,
        ct: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        simpleContentExtension: *mut crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
        bCreatingNewType: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveAttributes", (ct, simpleContentExtension, bCreatingNewType))?;
        Ok(__cordl_ret)
    }
    pub fn MoveAttributes_XmlSchemaSimpleContentExtension_XmlSchemaComplexType0(
        &mut self,
        scExtension: *mut crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
        ct: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveAttributes", (scExtension, ct))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessAttributes(
        &mut self,
        xse: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::XmlSchemaElement,
        >,
        effectiveSchemaType: *mut crate::System::Xml::Schema::XmlSchemaType,
        bCreatingNewType: bool,
        parentSchema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessAttributes",
                (xse, effectiveSchemaType, bCreatingNewType, parentSchema),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RefineSimpleType(
        &mut self,
        s: *mut crate::System::String,
        iTypeFlags: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("RefineSimpleType", (s, iTypeFlags))?;
        Ok(__cordl_ret)
    }
    pub fn SetMinMaxOccurs(
        &mut self,
        el: *mut crate::System::Xml::Schema::XmlSchemaElement,
        setMaxOccurs: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMinMaxOccurs", (el, setMaxOccurs))?;
        Ok(__cordl_ret)
    }
    pub fn SwitchUseToOptional(
        &mut self,
        attributes: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        attributesInInstance: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchUseToOptional", (attributes, attributesInInstance))?;
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
    pub fn get_Occurrence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaInference_InferenceOption,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaInference_InferenceOption = __cordl_object
            .invoke("get_Occurrence", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Occurrence(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaInference_InferenceOption,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Occurrence", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TypeInference(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaInference_InferenceOption,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TypeInference", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaInference {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference+InferenceOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSchemaInference_InferenceOption {
    Relaxed = 1i32,
    Restricted = 0i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference+InferenceOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XmlSchemaInference_InferenceOption => "System.Xml.Schema"
    ."XmlSchemaInference/InferenceOption"
);
