#[cfg(feature = "System+Xml+Schema+XmlSchemaComplexType")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaComplexType {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaType,
    pub block: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub contentModel: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaContentModel,
    >,
    pub particle: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaParticle,
    >,
    pub attributes: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectCollection,
    >,
    pub anyAttribute: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    >,
    pub contentTypeParticle: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaParticle,
    >,
    pub blockResolved: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub localElements: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub attributeUses: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaObjectTable,
    >,
    pub attributeWildcard: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaAnyAttribute,
    >,
    pub pvFlags: u8,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaComplexType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaComplexType =>
    "System.Xml.Schema"."XmlSchemaComplexType"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaComplexType")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaComplexType {
    type Target = crate::System::Xml::Schema::XmlSchemaType;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaComplexType")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaComplexType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaComplexType")]
impl crate::System::Xml::Schema::XmlSchemaComplexType {
    pub fn ClearCompiledState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCompiledState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CloneAttributes(
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectCollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloneAttributes", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloneGroupBaseParticles(
        groupBaseParticles: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectCollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloneGroupBaseParticles", (groupBaseParticles, parentSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloneParticle(
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloneParticle", (particle, parentSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone_XmlSchema1(
        &mut self,
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObject,
        > = __cordl_object.invoke("Clone", (parentSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsIdAttribute(
        &mut self,
        findAll: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ContainsIdAttribute", (findAll))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAnyType(
        processContents: crate::System::Xml::Schema::XmlSchemaContentProcessing,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateAnyType", (processContents))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResolvedElementForm(
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSchemaForm> {
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaForm = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResolvedElementForm", (parentSchema, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAttributeQNameRef(
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAttributeQNameRef", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasParticleRef(
        particle: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        >,
        parentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasParticleRef", (particle, parentSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetAttributeWildcard(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAttributeWildcard", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAttributes(
        &mut self,
        newAttributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAttributes", (newAttributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBlockResolved(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBlockResolved", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetContentTypeParticle(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContentTypeParticle", (value))?;
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
    pub fn get_AnyAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        > = __cordl_object.invoke("get_AnyAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AnyType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_AnyType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AnyTypeContentValidator() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ContentValidator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ContentValidator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AnyTypeContentValidator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AttributeUses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        > = __cordl_object.invoke("get_AttributeUses", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AttributeWildcard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        > = __cordl_object.invoke("get_AttributeWildcard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectCollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        > = __cordl_object.invoke("get_Attributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Block(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaDerivationMethod = __cordl_object
            .invoke("get_Block", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BlockResolved(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaDerivationMethod = __cordl_object
            .invoke("get_BlockResolved", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaContentModel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaContentModel,
        > = __cordl_object.invoke("get_ContentModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaContentType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaContentType = __cordl_object
            .invoke("get_ContentType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentTypeParticle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("get_ContentTypeParticle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAbstract(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAbstract", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsMixed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsMixed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectTable,
        > = __cordl_object.invoke("get_LocalElements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Particle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("get_Particle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UntypedAnyType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UntypedAnyType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AnyAttribute(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AnyAttribute", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Block(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Block", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ContentModel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaContentModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContentModel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HasWildCard(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HasWildCard", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsAbstract(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsAbstract", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsMixed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsMixed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Particle(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Particle", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaComplexType")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaComplexType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
