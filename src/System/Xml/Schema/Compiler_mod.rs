#[cfg(feature = "System+Xml+Schema+Compiler")]
#[repr(C)]
#[derive(Debug)]
pub struct Compiler {
    __cordl_parent: crate::System::Xml::Schema::BaseProcessor,
    pub restrictionErrorMsg: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub attributes: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub attributeGroups: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub elements: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub schemaTypes: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub groups: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub notations: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub examplars: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub identityConstraints: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub complexTypeStack: quest_hook::libil2cpp::Gc<crate::System::Collections::Stack>,
    pub schemasToCompile: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub importedSchemas: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub schemaForSchema: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchema,
    >,
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
    pub fn BuildParticleContentModel(
        &mut self,
        contentValidator: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ParticleContentValidator,
        >,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("BuildParticleContentModel", (contentValidator, particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateEffectiveTotalRange(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        minOccurs: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
        maxOccurs: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateEffectiveTotalRange", (particle, minOccurs, maxOccurs))?;
        Ok(__cordl_ret.into())
    }
    pub fn CannonicalizeAll(
        &mut self,
        all: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAll>,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("CannonicalizeAll", (all, root))?;
        Ok(__cordl_ret.into())
    }
    pub fn CannonicalizeChoice(
        &mut self,
        choice: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaChoice>,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("CannonicalizeChoice", (choice, root))?;
        Ok(__cordl_ret.into())
    }
    pub fn CannonicalizeElement(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("CannonicalizeElement", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn CannonicalizeGroupRef(
        &mut self,
        groupRef: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaGroupRef,
        >,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("CannonicalizeGroupRef", (groupRef, root))?;
        Ok(__cordl_ret.into())
    }
    pub fn CannonicalizeParticle(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("CannonicalizeParticle", (particle, root))?;
        Ok(__cordl_ret.into())
    }
    pub fn CannonicalizePointlessRoot(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("CannonicalizePointlessRoot", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CannonicalizeSequence(
        &mut self,
        sequence: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSequence,
        >,
        root: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("CannonicalizeSequence", (sequence, root))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckAtrributeGroupRestriction(
        &mut self,
        baseAttributeGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        >,
        derivedAttributeGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CheckAtrributeGroupRestriction",
                (baseAttributeGroup, derivedAttributeGroup),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckParticleDerivation_XmlSchemaComplexType0(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckParticleDerivation", (complexType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckParticleDerivation_XmlSchemaParticle_XmlSchemaParticle1(
        &mut self,
        derivedParticle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        baseParticle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckParticleDerivation", (derivedParticle, baseParticle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUnionType(
        &mut self,
        unionMember: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
        memberTypeDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        >,
        parentType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckUnionType", (unionMember, memberTypeDefinitions, parentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupAttribute(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupAttribute", (attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupAttributeGroup(
        &mut self,
        attributeGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupAttributeGroup", (attributeGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupAttributes(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupAttributes", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupComplexType(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupComplexType", (complexType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupElement(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupElement", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupGroup(
        &mut self,
        group: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupGroup", (group))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupParticle(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupParticle", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupSimpleType(
        &mut self,
        simpleType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupSimpleType", (simpleType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compile(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Compile", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileAnyAttributeIntersection(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
        b: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        > = __cordl_object.invoke("CompileAnyAttributeIntersection", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileAnyAttributeUnion(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
        b: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        > = __cordl_object.invoke("CompileAnyAttributeUnion", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileAttribute(
        &mut self,
        xa: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileAttribute", (xa))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileAttributeGroup(
        &mut self,
        attributeGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileAttributeGroup", (attributeGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileBaseMemberTypes(
        &mut self,
        simpleType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
            >,
        > = __cordl_object.invoke("CompileBaseMemberTypes", (simpleType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileComplexContent(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ContentValidator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ContentValidator,
        > = __cordl_object.invoke("CompileComplexContent", (complexType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileComplexContentExtension(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        complexContent: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexContent,
        >,
        complexExtension: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexContentExtension,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileComplexContentExtension",
                (complexType, complexContent, complexExtension),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileComplexContentRestriction(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        complexContent: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexContent,
        >,
        complexRestriction: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexContentRestriction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileComplexContentRestriction",
                (complexType, complexContent, complexRestriction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileComplexType(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileComplexType", (complexType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileComplexTypeElements(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileComplexTypeElements", (complexType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileContentTypeParticle(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("CompileContentTypeParticle", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileElement(
        &mut self,
        xe: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileElement", (xe))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileGroup(
        &mut self,
        group: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileGroup", (group))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileIdentityConstraint(
        &mut self,
        xi: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileIdentityConstraint", (xi))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileLocalAttributes(
        &mut self,
        baseType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        derivedType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        anyAttribute: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn CompileParticleElements_XmlSchemaComplexType_XmlSchemaParticle0(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileParticleElements", (complexType, particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileParticleElements_XmlSchemaParticle1(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileParticleElements", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileSimpleContentExtension(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        simpleExtension: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleContentExtension,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileSimpleContentExtension", (complexType, simpleExtension))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileSimpleContentRestriction(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        simpleRestriction: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleContentRestriction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileSimpleContentRestriction",
                (complexType, simpleRestriction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileSimpleType(
        &mut self,
        simpleType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileSimpleType", (simpleType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileSubstitutionGroup(
        &mut self,
        substitutionGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSubstitutionGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileSubstitutionGroup", (substitutionGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyPosition(
        &mut self,
        to: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnnotated>,
        from: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnnotated>,
        copyParent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyPosition", (to, from, copyParent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
        schemaCompiledInfo: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SchemaInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Execute", (schemaSet, schemaCompiledInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAnySchemaType(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaType,
        > = __cordl_object.invoke("GetAnySchemaType", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComplexType(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        > = __cordl_object.invoke("GetComplexType", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMappingParticle(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetMappingParticle", (particle, collection))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSchemaContentType(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        complexContent: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexContent,
        >,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaContentType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaContentType = __cordl_object
            .invoke("GetSchemaContentType", (complexType, complexContent, particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSimpleType(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = __cordl_object.invoke("GetSimpleType", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportAllCompiledSchemas(
        &mut self,
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImportAllCompiledSchemas", (schemaSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAnyFromAny(
        &mut self,
        derivedAny: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAny>,
        baseAny: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAny>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsAnyFromAny", (derivedAny, baseAny))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsChoiceFromChoiceSubstGroup(
        &mut self,
        derivedChoice: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaChoice,
        >,
        baseChoice: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaChoice,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsChoiceFromChoiceSubstGroup", (derivedChoice, baseChoice))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsElementFromAny(
        &mut self,
        derivedElement: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        >,
        baseAny: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAny>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsElementFromAny", (derivedElement, baseAny))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsElementFromElement(
        &mut self,
        derivedElement: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        >,
        baseElement: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsElementFromElement", (derivedElement, baseElement))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsElementFromGroupBase(
        &mut self,
        derivedElement: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        >,
        baseGroupBase: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaGroupBase,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsElementFromGroupBase", (derivedElement, baseGroupBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFixedEqual(
        &mut self,
        baseDecl: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaDeclBase>,
        derivedDecl: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SchemaDeclBase,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsFixedEqual", (baseDecl, derivedDecl))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGroupBaseFromAny(
        &mut self,
        derivedGroupBase: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaGroupBase,
        >,
        baseAny: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAny>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsGroupBaseFromAny", (derivedGroupBase, baseAny))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGroupBaseFromGroupBase(
        &mut self,
        derivedGroupBase: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaGroupBase,
        >,
        baseGroupBase: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaGroupBase,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn IsParticleEmptiable(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsParticleEmptiable", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsProcessContentsRestricted(
        &mut self,
        baseType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        derivedAttributeWildcard: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        >,
        baseAttributeWildcard: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "IsProcessContentsRestricted",
                (baseType, derivedAttributeWildcard, baseAttributeWildcard),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSequenceFromAll(
        &mut self,
        derivedSequence: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSequence,
        >,
        baseAll: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAll>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSequenceFromAll", (derivedSequence, baseAll))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSequenceFromChoice(
        &mut self,
        derivedSequence: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSequence,
        >,
        baseChoice: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaChoice,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSequenceFromChoice", (derivedSequence, baseChoice))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn IsValidOccurrenceRangeRestriction_XmlSchemaParticle_XmlSchemaParticle0(
        &mut self,
        derivedParticle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        baseParticle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "IsValidOccurrenceRangeRestriction",
                (derivedParticle, baseParticle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidRestriction(
        &mut self,
        derivedParticle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        baseParticle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidRestriction", (derivedParticle, baseParticle))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        eventHandler: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
        schemaForSchema: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        >,
        compilationSettings: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaCompilationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (nameTable, eventHandler, schemaForSchema, compilationSettings),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Output(
        &mut self,
        schemaInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Output", (schemaInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Prepare(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        cleanup: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Prepare", (schema, cleanup))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSubstitutionGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSubstitutionGroups", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PushComplexType(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushComplexType", (complexType))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecursivelyCheckRedefinedAttributeGroups(
        &mut self,
        attributeGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        >,
        baseAttributeGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecursivelyCheckRedefinedAttributeGroups",
                (attributeGroup, baseAttributeGroup),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RecursivelyCheckRedefinedGroups(
        &mut self,
        redefinedGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaGroup,
        >,
        baseGroup: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecursivelyCheckRedefinedGroups", (redefinedGroup, baseGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultFixed(
        &mut self,
        xa: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
        decl: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaAttDef>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultFixed", (xa, decl))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSForSSimpleTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSForSSimpleTypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        eventHandler: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
        schemaForSchema: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        >,
        compilationSettings: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaCompilationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (nameTable, eventHandler, schemaForSchema, compilationSettings),
            )?;
        Ok(__cordl_ret.into())
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
