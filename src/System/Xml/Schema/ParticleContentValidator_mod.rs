#[cfg(feature = "System+Xml+Schema+ParticleContentValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleContentValidator {
    __cordl_parent: crate::System::Xml::Schema::ContentValidator,
    pub symbols: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SymbolsDictionary,
    >,
    pub positions: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
    pub stack: quest_hook::libil2cpp::Gc<crate::System::Collections::Stack>,
    pub contentNode: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SyntaxTreeNode,
    >,
    pub isPartial: bool,
    pub minMaxNodesCount: i32,
    pub enableUpaCheck: bool,
}
#[cfg(feature = "System+Xml+Schema+ParticleContentValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::ParticleContentValidator =>
    "System.Xml.Schema"."ParticleContentValidator"
);
#[cfg(feature = "System+Xml+Schema+ParticleContentValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::ParticleContentValidator {
    type Target = crate::System::Xml::Schema::ContentValidator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ParticleContentValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ParticleContentValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ParticleContentValidator")]
impl crate::System::Xml::Schema::ParticleContentValidator {
    pub fn AddChoice(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChoice", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddLeafNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SyntaxTreeNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLeafNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddLeafRange(
        &mut self,
        min: crate::System::Decimal,
        max: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLeafRange", (min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        particle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddName", (name, particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNamespaceList(
        &mut self,
        namespaceList: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::NamespaceList,
        >,
        particle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNamespaceList", (namespaceList, particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPlus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPlus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddQMark(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddQMark", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSequence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSequence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddStar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildTransitionTable(
        &mut self,
        firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        followpos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::BitSet>,
        >,
        endMarkerPos: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<i32>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<i32>,
            >,
        > = __cordl_object
            .invoke("BuildTransitionTable", (firstpos, followpos, endMarkerPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateTotalFollowposForRangeNodes(
        &mut self,
        firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        followpos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::BitSet>,
        >,
        posWithRangeTerminals: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::BitSet,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::BitSet>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::BitSet>,
        > = __cordl_object
            .invoke(
                "CalculateTotalFollowposForRangeNodes",
                (firstpos, followpos, posWithRangeTerminals),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCMUPAWithLeafRangeNodes(
        &mut self,
        curpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckCMUPAWithLeafRangeNodes", (curpos))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUniqueParticleAttribution_BitSet1(
        &mut self,
        curpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckUniqueParticleAttribution", (curpos))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUniqueParticleAttribution_Il2CppArray0(
        &mut self,
        firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        followpos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::BitSet>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckUniqueParticleAttribution", (firstpos, followpos))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloseGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Closure(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::InteriorNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Closure", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteValidation(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CompleteValidation", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exists(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Exists", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finish(
        &mut self,
        useDFA: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ContentValidator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ContentValidator,
        > = __cordl_object.invoke("Finish", (useDFA))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicableMinMaxFollowPos(
        &mut self,
        curpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        posWithRangeTerminals: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::BitSet,
        >,
        minmaxFollowPos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::BitSet>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet> = __cordl_object
            .invoke(
                "GetApplicableMinMaxFollowPos",
                (curpos, posWithRangeTerminals, minmaxFollowPos),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitValidation(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitValidation", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_XmlSchemaContentType0(
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentType))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        enableUpaCheck: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentType, enableUpaCheck))?;
        Ok(__cordl_object.into())
    }
    pub fn OpenGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateElement(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ValidateElement", (name, context, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlSchemaContentType0(
        &mut self,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        enableUpaCheck: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contentType, enableUpaCheck))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+ParticleContentValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::ParticleContentValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
