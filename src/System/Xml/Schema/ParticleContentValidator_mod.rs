#[cfg(feature = "System+Xml+Schema+ParticleContentValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleContentValidator {
    __cordl_parent: crate::System::Xml::Schema::ContentValidator,
    pub symbols: *mut crate::System::Xml::Schema::SymbolsDictionary,
    pub positions: *mut crate::System::Xml::Schema::Positions,
    pub stack: *mut crate::System::Collections::Stack,
    pub contentNode: *mut crate::System::Xml::Schema::SyntaxTreeNode,
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
    pub fn _ctor_XmlSchemaContentType0(
        &mut self,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contentType))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn AddChoice(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChoice", ())?;
        Ok(__cordl_ret)
    }
    pub fn Finish(
        &mut self,
        useDFA: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::ContentValidator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::ContentValidator = __cordl_object
            .invoke("Finish", (useDFA))?;
        Ok(__cordl_ret)
    }
    pub fn CheckCMUPAWithLeafRangeNodes(
        &mut self,
        curpos: *mut crate::System::Xml::Schema::BitSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckCMUPAWithLeafRangeNodes", (curpos))?;
        Ok(__cordl_ret)
    }
    pub fn InitValidation(
        &mut self,
        context: *mut crate::System::Xml::Schema::ValidationState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitValidation", (context))?;
        Ok(__cordl_ret)
    }
    pub fn AddNamespaceList(
        &mut self,
        namespaceList: *mut crate::System::Xml::Schema::NamespaceList,
        particle: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNamespaceList", (namespaceList, particle))?;
        Ok(__cordl_ret)
    }
    pub fn OpenGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicableMinMaxFollowPos(
        &mut self,
        curpos: *mut crate::System::Xml::Schema::BitSet,
        posWithRangeTerminals: *mut crate::System::Xml::Schema::BitSet,
        minmaxFollowPos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::BitSet,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::BitSet> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::BitSet = __cordl_object
            .invoke(
                "GetApplicableMinMaxFollowPos",
                (curpos, posWithRangeTerminals, minmaxFollowPos),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddStar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStar", ())?;
        Ok(__cordl_ret)
    }
    pub fn Closure(
        &mut self,
        node: *mut crate::System::Xml::Schema::InteriorNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Closure", (node))?;
        Ok(__cordl_ret)
    }
    pub fn AddPlus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPlus", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateElement(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        context: *mut crate::System::Xml::Schema::ValidationState,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ValidateElement", (name, context, errorCode))?;
        Ok(__cordl_ret)
    }
    pub fn Exists(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Exists", (name))?;
        Ok(__cordl_ret)
    }
    pub fn AddLeafNode(
        &mut self,
        node: *mut crate::System::Xml::Schema::SyntaxTreeNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLeafNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn AddQMark(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddQMark", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckUniqueParticleAttribution_Il2CppArray0(
        &mut self,
        firstpos: *mut crate::System::Xml::Schema::BitSet,
        followpos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::BitSet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckUniqueParticleAttribution", (firstpos, followpos))?;
        Ok(__cordl_ret)
    }
    pub fn CheckUniqueParticleAttribution_BitSet1(
        &mut self,
        curpos: *mut crate::System::Xml::Schema::BitSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckUniqueParticleAttribution", (curpos))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateTotalFollowposForRangeNodes(
        &mut self,
        firstpos: *mut crate::System::Xml::Schema::BitSet,
        followpos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::BitSet,
        >,
        posWithRangeTerminals: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::Schema::BitSet,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::BitSet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::BitSet,
        > = __cordl_object
            .invoke(
                "CalculateTotalFollowposForRangeNodes",
                (firstpos, followpos, posWithRangeTerminals),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddName(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        particle: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddName", (name, particle))?;
        Ok(__cordl_ret)
    }
    pub fn AddSequence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSequence", ())?;
        Ok(__cordl_ret)
    }
    pub fn CloseGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn BuildTransitionTable(
        &mut self,
        firstpos: *mut crate::System::Xml::Schema::BitSet,
        followpos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::BitSet,
        >,
        endMarkerPos: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object
            .invoke("BuildTransitionTable", (firstpos, followpos, endMarkerPos))?;
        Ok(__cordl_ret)
    }
    pub fn CompleteValidation(
        &mut self,
        context: *mut crate::System::Xml::Schema::ValidationState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CompleteValidation", (context))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_XmlSchemaContentType0(
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentType))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        enableUpaCheck: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentType, enableUpaCheck))?;
        Ok(__cordl_object)
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
