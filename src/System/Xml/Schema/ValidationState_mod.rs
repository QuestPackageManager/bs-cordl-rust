#[cfg(feature = "System+Xml+Schema+ValidationState")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub IsNill: bool,
    pub IsDefault: bool,
    pub NeedValidateChildren: bool,
    pub CheckRequiredAttribute: bool,
    pub ValidationSkipped: bool,
    pub ProcessContents: crate::System::Xml::Schema::XmlSchemaContentProcessing,
    pub Validity: crate::System::Xml::Schema::XmlSchemaValidity,
    pub ElementDecl: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SchemaElementDecl,
    >,
    pub ElementDeclBeforeXsi: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SchemaElementDecl,
    >,
    pub LocalName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Namespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Constr: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ConstraintStruct>,
        >,
    >,
    pub CurrentState: crate::System::Xml::Schema::StateUnion,
    pub HasMatched: bool,
    pub CurPos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        >,
    >,
    pub AllElementsSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub RunningPositions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::System::Xml::Schema::RangePositionInfo,
        >,
    >,
    pub TooComplex: bool,
}
#[cfg(feature = "System+Xml+Schema+ValidationState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::ValidationState =>
    "System.Xml.Schema"."ValidationState"
);
#[cfg(feature = "System+Xml+Schema+ValidationState")]
impl std::ops::Deref for crate::System::Xml::Schema::ValidationState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ValidationState")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ValidationState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ValidationState")]
impl crate::System::Xml::Schema::ValidationState {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "System+Xml+Schema+ValidationState")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::ValidationState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
