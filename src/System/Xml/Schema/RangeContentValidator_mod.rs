#[cfg(feature = "System+Xml+Schema+RangeContentValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct RangeContentValidator {
    __cordl_parent: crate::System::Xml::Schema::ContentValidator,
    pub firstpos: *mut crate::System::Xml::Schema::BitSet,
    pub followpos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::Schema::BitSet,
    >,
    pub positionsWithRangeTerminals: *mut crate::System::Xml::Schema::BitSet,
    pub symbols: *mut crate::System::Xml::Schema::SymbolsDictionary,
    pub positions: *mut crate::System::Xml::Schema::Positions,
    pub minMaxNodesCount: i32,
    pub endMarkerPos: i32,
}
#[cfg(feature = "System+Xml+Schema+RangeContentValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::RangeContentValidator =>
    "System.Xml.Schema"."RangeContentValidator"
);
#[cfg(feature = "System+Xml+Schema+RangeContentValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::RangeContentValidator {
    type Target = crate::System::Xml::Schema::ContentValidator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RangeContentValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::RangeContentValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RangeContentValidator")]
impl crate::System::Xml::Schema::RangeContentValidator {
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
    pub fn ExpectedElements(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
        isRequiredOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object.invoke("ExpectedElements", (context, isRequiredOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectedParticles(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
        isRequiredOnly: bool,
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object
            .invoke("ExpectedParticles", (context, isRequiredOnly, schemaSet))?;
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
    pub fn New(
        firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        followpos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::BitSet>,
        >,
        symbols: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SymbolsDictionary,
        >,
        positions: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
        endMarkerPos: i32,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        isEmptiable: bool,
        positionsWithRangeTerminals: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::BitSet,
        >,
        minmaxNodesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    firstpos,
                    followpos,
                    symbols,
                    positions,
                    endMarkerPos,
                    contentType,
                    isEmptiable,
                    positionsWithRangeTerminals,
                    minmaxNodesCount,
                ),
            )?;
        Ok(__cordl_object.into())
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
    pub fn _ctor(
        &mut self,
        firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        followpos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::BitSet>,
        >,
        symbols: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SymbolsDictionary,
        >,
        positions: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
        endMarkerPos: i32,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        isEmptiable: bool,
        positionsWithRangeTerminals: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::BitSet,
        >,
        minmaxNodesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    firstpos,
                    followpos,
                    symbols,
                    positions,
                    endMarkerPos,
                    contentType,
                    isEmptiable,
                    positionsWithRangeTerminals,
                    minmaxNodesCount,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+RangeContentValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::RangeContentValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
