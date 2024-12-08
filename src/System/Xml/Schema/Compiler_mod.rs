#[cfg(feature = "System+Xml+Schema+Compiler")]
#[repr(C)]
#[derive(Debug)]
pub struct Compiler {
    __cordl_parent: crate::System::Xml::Schema::BaseProcessor,
    pub restrictionErrorMsg: *mut crate::System::String,
    pub attributes: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub attributeGroups: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub elements: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub schemaTypes: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub groups: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub notations: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub examplars: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub identityConstraints: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    pub complexTypeStack: *mut crate::System::Collections::Stack,
    pub schemasToCompile: *mut crate::System::Collections::Hashtable,
    pub importedSchemas: *mut crate::System::Collections::Hashtable,
    pub schemaForSchema: *mut crate::System::Xml::Schema::XmlSchema,
}
#[cfg(feature = "System+Xml+Schema+Compiler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Compiler =>
    "System.Xml.Schema"."Compiler"
);
#[cfg(feature = "System+Xml+Schema+Compiler")]
impl std::ops::Deref for crate::System::Xml::Schema::Compiler {
    type Target = crate::System::Xml::Schema::BaseProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Compiler")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Compiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Compiler")]
impl crate::System::Xml::Schema::Compiler {
    pub fn CompileComplexTypeElements(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileComplexTypeElements", (complexType))?;
        Ok(__cordl_ret)
    }
    pub fn CompileSimpleType(
        &mut self,
        simpleType: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileSimpleType", (simpleType))?;
        Ok(__cordl_ret)
    }
    pub fn CannonicalizeGroupRef(
        &mut self,
        groupRef: *mut crate::System::Xml::Schema::XmlSchemaGroupRef,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaParticle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaParticle = __cordl_object
            .invoke("CannonicalizeGroupRef", (groupRef, root))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidOccurrenceRangeRestriction_XmlSchemaParticle_XmlSchemaParticle0(
        &mut self,
        derivedParticle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
        baseParticle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "IsValidOccurrenceRangeRestriction",
                (derivedParticle, baseParticle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsValidOccurrenceRangeRestriction_Decimal_Decimal_Decimal_Decimal1(
        &mut self,
        minOccurs: crate::System::Decimal,
        maxOccurs: crate::System::Decimal,
        baseMinOccurs: crate::System::Decimal,
        baseMaxOccurs: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "IsValidOccurrenceRangeRestriction",
                (minOccurs, maxOccurs, baseMinOccurs, baseMaxOccurs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BuildParticleContentModel(
        &mut self,
        contentValidator: *mut crate::System::Xml::Schema::ParticleContentValidator,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("BuildParticleContentModel", (contentValidator, particle))?;
        Ok(__cordl_ret)
    }
    pub fn CompileAnyAttributeUnion(
        &mut self,
        a: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        b: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute = __cordl_object
            .invoke("CompileAnyAttributeUnion", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn CannonicalizePointlessRoot(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaParticle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaParticle = __cordl_object
            .invoke("CannonicalizePointlessRoot", (particle))?;
        Ok(__cordl_ret)
    }
    pub fn CannonicalizeElement(
        &mut self,
        element: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaParticle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaParticle = __cordl_object
            .invoke("CannonicalizeElement", (element))?;
        Ok(__cordl_ret)
    }
    pub fn GetSchemaContentType(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        complexContent: *mut crate::System::Xml::Schema::XmlSchemaComplexContent,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaContentType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaContentType = __cordl_object
            .invoke("GetSchemaContentType", (complexType, complexContent, particle))?;
        Ok(__cordl_ret)
    }
    pub fn SetDefaultFixed(
        &mut self,
        xa: *mut crate::System::Xml::Schema::XmlSchemaAttribute,
        decl: *mut crate::System::Xml::Schema::SchemaAttDef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultFixed", (xa, decl))?;
        Ok(__cordl_ret)
    }
    pub fn IsChoiceFromChoiceSubstGroup(
        &mut self,
        derivedChoice: *mut crate::System::Xml::Schema::XmlSchemaChoice,
        baseChoice: *mut crate::System::Xml::Schema::XmlSchemaChoice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsChoiceFromChoiceSubstGroup", (derivedChoice, baseChoice))?;
        Ok(__cordl_ret)
    }
    pub fn CompileComplexContent(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::ContentValidator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::ContentValidator = __cordl_object
            .invoke("CompileComplexContent", (complexType))?;
        Ok(__cordl_ret)
    }
    pub fn CompileIdentityConstraint(
        &mut self,
        xi: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileIdentityConstraint", (xi))?;
        Ok(__cordl_ret)
    }
    pub fn Prepare(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        cleanup: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Prepare", (schema, cleanup))?;
        Ok(__cordl_ret)
    }
    pub fn CheckUnionType(
        &mut self,
        unionMember: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
        memberTypeDefinitions: *mut crate::System::Collections::ArrayList,
        parentType: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckUnionType", (unionMember, memberTypeDefinitions, parentType))?;
        Ok(__cordl_ret)
    }
    pub fn CleanupAttributeGroup(
        &mut self,
        attributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupAttributeGroup", (attributeGroup))?;
        Ok(__cordl_ret)
    }
    pub fn PushComplexType(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushComplexType", (complexType))?;
        Ok(__cordl_ret)
    }
    pub fn IsSequenceFromChoice(
        &mut self,
        derivedSequence: *mut crate::System::Xml::Schema::XmlSchemaSequence,
        baseChoice: *mut crate::System::Xml::Schema::XmlSchemaChoice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSequenceFromChoice", (derivedSequence, baseChoice))?;
        Ok(__cordl_ret)
    }
    pub fn Output(
        &mut self,
        schemaInfo: *mut crate::System::Xml::Schema::SchemaInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Output", (schemaInfo))?;
        Ok(__cordl_ret)
    }
    pub fn CompileAttribute(
        &mut self,
        xa: *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileAttribute", (xa))?;
        Ok(__cordl_ret)
    }
    pub fn CannonicalizeChoice(
        &mut self,
        choice: *mut crate::System::Xml::Schema::XmlSchemaChoice,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaParticle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaParticle = __cordl_object
            .invoke("CannonicalizeChoice", (choice, root))?;
        Ok(__cordl_ret)
    }
    pub fn CannonicalizeAll(
        &mut self,
        all: *mut crate::System::Xml::Schema::XmlSchemaAll,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaParticle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaParticle = __cordl_object
            .invoke("CannonicalizeAll", (all, root))?;
        Ok(__cordl_ret)
    }
    pub fn CleanupElement(
        &mut self,
        element: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupElement", (element))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSForSSimpleTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSForSSimpleTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
        schemaForSchema: *mut crate::System::Xml::Schema::XmlSchema,
        compilationSettings: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (nameTable, eventHandler, schemaForSchema, compilationSettings),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CompileAnyAttributeIntersection(
        &mut self,
        a: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        b: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute = __cordl_object
            .invoke("CompileAnyAttributeIntersection", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn CompileComplexContentRestriction(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        complexContent: *mut crate::System::Xml::Schema::XmlSchemaComplexContent,
        complexRestriction: *mut crate::System::Xml::Schema::XmlSchemaComplexContentRestriction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileComplexContentRestriction",
                (complexType, complexContent, complexRestriction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsParticleEmptiable(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsParticleEmptiable", (particle))?;
        Ok(__cordl_ret)
    }
    pub fn IsSequenceFromAll(
        &mut self,
        derivedSequence: *mut crate::System::Xml::Schema::XmlSchemaSequence,
        baseAll: *mut crate::System::Xml::Schema::XmlSchemaAll,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSequenceFromAll", (derivedSequence, baseAll))?;
        Ok(__cordl_ret)
    }
    pub fn CleanupSimpleType(
        &mut self,
        simpleType: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupSimpleType", (simpleType))?;
        Ok(__cordl_ret)
    }
    pub fn RecursivelyCheckRedefinedGroups(
        &mut self,
        redefinedGroup: *mut crate::System::Xml::Schema::XmlSchemaGroup,
        baseGroup: *mut crate::System::Xml::Schema::XmlSchemaGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecursivelyCheckRedefinedGroups", (redefinedGroup, baseGroup))?;
        Ok(__cordl_ret)
    }
    pub fn Compile(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Compile", ())?;
        Ok(__cordl_ret)
    }
    pub fn CleanupAttribute(
        &mut self,
        attribute: *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupAttribute", (attribute))?;
        Ok(__cordl_ret)
    }
    pub fn IsElementFromElement(
        &mut self,
        derivedElement: *mut crate::System::Xml::Schema::XmlSchemaElement,
        baseElement: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsElementFromElement", (derivedElement, baseElement))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidRestriction(
        &mut self,
        derivedParticle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
        baseParticle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidRestriction", (derivedParticle, baseParticle))?;
        Ok(__cordl_ret)
    }
    pub fn GetAnySchemaType(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaType = __cordl_object
            .invoke("GetAnySchemaType", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetMappingParticle(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
        collection: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetMappingParticle", (particle, collection))?;
        Ok(__cordl_ret)
    }
    pub fn IsAnyFromAny(
        &mut self,
        derivedAny: *mut crate::System::Xml::Schema::XmlSchemaAny,
        baseAny: *mut crate::System::Xml::Schema::XmlSchemaAny,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsAnyFromAny", (derivedAny, baseAny))?;
        Ok(__cordl_ret)
    }
    pub fn RecursivelyCheckRedefinedAttributeGroups(
        &mut self,
        attributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        baseAttributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecursivelyCheckRedefinedAttributeGroups",
                (attributeGroup, baseAttributeGroup),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CleanupGroup(
        &mut self,
        group: *mut crate::System::Xml::Schema::XmlSchemaGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupGroup", (group))?;
        Ok(__cordl_ret)
    }
    pub fn CheckParticleDerivation_XmlSchemaComplexType0(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckParticleDerivation", (complexType))?;
        Ok(__cordl_ret)
    }
    pub fn CheckParticleDerivation_XmlSchemaParticle_XmlSchemaParticle1(
        &mut self,
        derivedParticle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
        baseParticle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckParticleDerivation", (derivedParticle, baseParticle))?;
        Ok(__cordl_ret)
    }
    pub fn IsGroupBaseFromAny(
        &mut self,
        derivedGroupBase: *mut crate::System::Xml::Schema::XmlSchemaGroupBase,
        baseAny: *mut crate::System::Xml::Schema::XmlSchemaAny,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsGroupBaseFromAny", (derivedGroupBase, baseAny))?;
        Ok(__cordl_ret)
    }
    pub fn GetSimpleType(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaSimpleType = __cordl_object
            .invoke("GetSimpleType", (name))?;
        Ok(__cordl_ret)
    }
    pub fn CannonicalizeParticle(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaParticle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaParticle = __cordl_object
            .invoke("CannonicalizeParticle", (particle, root))?;
        Ok(__cordl_ret)
    }
    pub fn IsElementFromAny(
        &mut self,
        derivedElement: *mut crate::System::Xml::Schema::XmlSchemaElement,
        baseAny: *mut crate::System::Xml::Schema::XmlSchemaAny,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsElementFromAny", (derivedElement, baseAny))?;
        Ok(__cordl_ret)
    }
    pub fn IsProcessContentsRestricted(
        &mut self,
        baseType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        derivedAttributeWildcard: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        baseAttributeWildcard: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "IsProcessContentsRestricted",
                (baseType, derivedAttributeWildcard, baseAttributeWildcard),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CompileContentTypeParticle(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaParticle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaParticle = __cordl_object
            .invoke("CompileContentTypeParticle", (particle))?;
        Ok(__cordl_ret)
    }
    pub fn Execute(
        &mut self,
        schemaSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
        schemaCompiledInfo: *mut crate::System::Xml::Schema::SchemaInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Execute", (schemaSet, schemaCompiledInfo))?;
        Ok(__cordl_ret)
    }
    pub fn CompileParticleElements_XmlSchemaComplexType_XmlSchemaParticle0(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileParticleElements", (complexType, particle))?;
        Ok(__cordl_ret)
    }
    pub fn CompileParticleElements_XmlSchemaParticle1(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileParticleElements", (particle))?;
        Ok(__cordl_ret)
    }
    pub fn CleanupAttributes(
        &mut self,
        attributes: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupAttributes", (attributes))?;
        Ok(__cordl_ret)
    }
    pub fn CleanupComplexType(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupComplexType", (complexType))?;
        Ok(__cordl_ret)
    }
    pub fn CompileComplexType(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileComplexType", (complexType))?;
        Ok(__cordl_ret)
    }
    pub fn CompileSimpleContentRestriction(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        simpleRestriction: *mut crate::System::Xml::Schema::XmlSchemaSimpleContentRestriction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileSimpleContentRestriction",
                (complexType, simpleRestriction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CompileComplexContentExtension(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        complexContent: *mut crate::System::Xml::Schema::XmlSchemaComplexContent,
        complexExtension: *mut crate::System::Xml::Schema::XmlSchemaComplexContentExtension,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileComplexContentExtension",
                (complexType, complexContent, complexExtension),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CompileElement(
        &mut self,
        xe: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileElement", (xe))?;
        Ok(__cordl_ret)
    }
    pub fn ImportAllCompiledSchemas(
        &mut self,
        schemaSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImportAllCompiledSchemas", (schemaSet))?;
        Ok(__cordl_ret)
    }
    pub fn CleanupParticle(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupParticle", (particle))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessSubstitutionGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSubstitutionGroups", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompileGroup(
        &mut self,
        group: *mut crate::System::Xml::Schema::XmlSchemaGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileGroup", (group))?;
        Ok(__cordl_ret)
    }
    pub fn CannonicalizeSequence(
        &mut self,
        sequence: *mut crate::System::Xml::Schema::XmlSchemaSequence,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaParticle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaParticle = __cordl_object
            .invoke("CannonicalizeSequence", (sequence, root))?;
        Ok(__cordl_ret)
    }
    pub fn IsElementFromGroupBase(
        &mut self,
        derivedElement: *mut crate::System::Xml::Schema::XmlSchemaElement,
        baseGroupBase: *mut crate::System::Xml::Schema::XmlSchemaGroupBase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsElementFromGroupBase", (derivedElement, baseGroupBase))?;
        Ok(__cordl_ret)
    }
    pub fn IsGroupBaseFromGroupBase(
        &mut self,
        derivedGroupBase: *mut crate::System::Xml::Schema::XmlSchemaGroupBase,
        baseGroupBase: *mut crate::System::Xml::Schema::XmlSchemaGroupBase,
        skipEmptableOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "IsGroupBaseFromGroupBase",
                (derivedGroupBase, baseGroupBase, skipEmptableOnly),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CalculateEffectiveTotalRange(
        &mut self,
        particle: *mut crate::System::Xml::Schema::XmlSchemaParticle,
        minOccurs: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
        maxOccurs: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateEffectiveTotalRange", (particle, minOccurs, maxOccurs))?;
        Ok(__cordl_ret)
    }
    pub fn CompileSimpleContentExtension(
        &mut self,
        complexType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        simpleExtension: *mut crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileSimpleContentExtension", (complexType, simpleExtension))?;
        Ok(__cordl_ret)
    }
    pub fn CheckAtrributeGroupRestriction(
        &mut self,
        baseAttributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        derivedAttributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CheckAtrributeGroupRestriction",
                (baseAttributeGroup, derivedAttributeGroup),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CompileAttributeGroup(
        &mut self,
        attributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileAttributeGroup", (attributeGroup))?;
        Ok(__cordl_ret)
    }
    pub fn CopyPosition(
        &mut self,
        to: *mut crate::System::Xml::Schema::XmlSchemaAnnotated,
        from: *mut crate::System::Xml::Schema::XmlSchemaAnnotated,
        copyParent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyPosition", (to, from, copyParent))?;
        Ok(__cordl_ret)
    }
    pub fn CompileSubstitutionGroup(
        &mut self,
        substitutionGroup: *mut crate::System::Xml::Schema::XmlSchemaSubstitutionGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileSubstitutionGroup", (substitutionGroup))?;
        Ok(__cordl_ret)
    }
    pub fn IsFixedEqual(
        &mut self,
        baseDecl: *mut crate::System::Xml::Schema::SchemaDeclBase,
        derivedDecl: *mut crate::System::Xml::Schema::SchemaDeclBase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsFixedEqual", (baseDecl, derivedDecl))?;
        Ok(__cordl_ret)
    }
    pub fn CompileBaseMemberTypes(
        &mut self,
        simpleType: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = __cordl_object.invoke("CompileBaseMemberTypes", (simpleType))?;
        Ok(__cordl_ret)
    }
    pub fn GetComplexType(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaComplexType = __cordl_object
            .invoke("GetComplexType", (name))?;
        Ok(__cordl_ret)
    }
    pub fn CompileLocalAttributes(
        &mut self,
        baseType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        derivedType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        attributes: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        anyAttribute: *mut crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        derivedBy: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileLocalAttributes",
                (baseType, derivedType, attributes, anyAttribute, derivedBy),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nameTable: *mut crate::System::Xml::XmlNameTable,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
        schemaForSchema: *mut crate::System::Xml::Schema::XmlSchema,
        compilationSettings: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (nameTable, eventHandler, schemaForSchema, compilationSettings),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+Compiler")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Compiler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
