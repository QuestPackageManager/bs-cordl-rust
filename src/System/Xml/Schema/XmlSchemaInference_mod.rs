#[cfg(feature = "System+Xml+Schema+XmlSchemaInference")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaInference {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub rootSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    pub schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    pub xtr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    pub nametable: quest_hook::libil2cpp::Gc<crate::System::Xml::NameTable>,
    pub TargetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NamespaceManager: quest_hook::libil2cpp::Gc<
        crate::System::Xml::XmlNamespaceManager,
    >,
    pub schemaList: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub occurrence: crate::System::Xml::Schema::XmlSchemaInference_InferenceOption,
    pub typeInference: crate::System::Xml::Schema::XmlSchemaInference_InferenceOption,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::XmlSchemaInference {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "XmlSchemaInference";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaInference {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        childURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bCreatingNewType: bool,
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        addLocation: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        compiledAttributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttribute,
        > = __cordl_object
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
        Ok(__cordl_ret.into())
    }
    pub fn AddElement(
        &mut self,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        childURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        addLocation: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        positionWithinCollection: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        > = __cordl_object
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
        Ok(__cordl_ret.into())
    }
    pub fn CheckSimpleContentExtension(
        &mut self,
        ct: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
        > = __cordl_object.invoke("CheckSimpleContentExtension", (ct))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewElementforChoice(
        &mut self,
        copyElement: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        > = __cordl_object.invoke("CreateNewElementforChoice", (copyElement))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateXmlSchema(
        &mut self,
        targetNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = __cordl_object.invoke("CreateXmlSchema", (targetNS))?;
        Ok(__cordl_ret.into())
    }
    pub fn DateTime(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bDate: bool,
        bTime: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DateTime", (s, bDate, bTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindAttribute(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttribute,
        > = __cordl_object.invoke("FindAttribute", (attributes, attrName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindAttributeRef(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
        attributeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nsURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttribute,
        > = __cordl_object
            .invoke("FindAttributeRef", (attributes, attributeName, nsURI))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindElement(
        &mut self,
        elements: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        elementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        > = __cordl_object.invoke("FindElement", (elements, elementName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindElementRef(
        &mut self,
        elements: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        elementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nsURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        > = __cordl_object.invoke("FindElementRef", (elements, elementName, nsURI))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindGlobalElement(
        &mut self,
        namespaceURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentSchema: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        > = __cordl_object
            .invoke("FindGlobalElement", (namespaceURI, localName, parentSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMatchingElement(
        &mut self,
        bCreatingNewType: bool,
        xtr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        ct: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
        lastUsedSeqItem: quest_hook::libil2cpp::ByRefMut<i32>,
        bParticleChanged: quest_hook::libil2cpp::ByRefMut<bool>,
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        setMaxoccurs: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        > = __cordl_object
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
        Ok(__cordl_ret.into())
    }
    pub fn GetEffectiveSchemaType(
        &mut self,
        elem: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
        bCreatingNewType: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaType,
        > = __cordl_object.invoke("GetEffectiveSchemaType", (elem, bCreatingNewType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSchemaType(
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSchemaType", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn InferElement(
        &mut self,
        xse: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
        bCreatingNewType: bool,
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InferElement", (xse, bCreatingNewType, parentSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn InferSchema(
        &mut self,
        instanceDocument: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSet,
        > = __cordl_object.invoke("InferSchema", (instanceDocument))?;
        Ok(__cordl_ret.into())
    }
    pub fn InferSchema1(
        &mut self,
        instanceDocument: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        schemas: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSet,
        > = __cordl_object.invoke("InferSchema1", (instanceDocument, schemas))?;
        Ok(__cordl_ret.into())
    }
    pub fn InferSimpleType(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bNeedsRangeCheck: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InferSimpleType", (s, bNeedsRangeCheck))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeExistingAttributesOptional(
        &mut self,
        ct: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
        attributesInInstance: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MakeExistingAttributesOptional", (ct, attributesInInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveAttributes_XmlSchemaComplexType_XmlSchemaSimpleContentExtension__cordl_bool1(
        &mut self,
        ct: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
        simpleContentExtension: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
        >,
        bCreatingNewType: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveAttributes", (ct, simpleContentExtension, bCreatingNewType))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveAttributes_XmlSchemaSimpleContentExtension_XmlSchemaComplexType0(
        &mut self,
        scExtension: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
        >,
        ct: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveAttributes", (scExtension, ct))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessAttributes(
        &mut self,
        xse: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
        >,
        effectiveSchemaType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaType,
        >,
        bCreatingNewType: bool,
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessAttributes",
                (xse, effectiveSchemaType, bCreatingNewType, parentSchema),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RefineSimpleType(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        iTypeFlags: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlQualifiedName,
        > = __cordl_object.invoke("RefineSimpleType", (s, iTypeFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMinMaxOccurs(
        &mut self,
        el: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
        setMaxOccurs: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMinMaxOccurs", (el, setMaxOccurs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchUseToOptional(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        attributesInInstance: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchUseToOptional", (attributes, attributesInInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlSchemaInference_InferenceOption {
    #[default]
    Relaxed = 1i32,
    Restricted = 0i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInference+InferenceOption")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::XmlSchemaInference_InferenceOption {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "InferenceOption";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Xml::Schema::XmlSchemaInference_InferenceOption {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::XmlSchemaInference_InferenceOption {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Xml::Schema::XmlSchemaInference_InferenceOption {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Xml::Schema::XmlSchemaInference_InferenceOption {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
