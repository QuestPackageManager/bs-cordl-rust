#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstraintStruct {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub constraint: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::CompiledIdentityConstraint,
    >,
    pub axisSelector: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SelectorActiveAxis,
    >,
    pub axisFields: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub qualifiedTable: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub keyrefTable: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub tableDim: i32,
}
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::ConstraintStruct =>
    "System.Xml.Schema"."ConstraintStruct"
);
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
impl std::ops::Deref for crate::System::Xml::Schema::ConstraintStruct {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ConstraintStruct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
impl crate::System::Xml::Schema::ConstraintStruct {
    pub fn New(
        constraint: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::CompiledIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constraint))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        constraint: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::CompiledIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (constraint))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TableDim(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TableDim", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+ConstraintStruct")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::ConstraintStruct {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
