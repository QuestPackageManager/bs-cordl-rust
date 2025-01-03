#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ParentForeignKeyConstraintEnumerator {
    __cordl_parent: crate::System::Data::ForeignKeyConstraintEnumerator,
    pub _table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
}
#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Data::ParentForeignKeyConstraintEnumerator => "System.Data"
    ."ParentForeignKeyConstraintEnumerator"
);
#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
impl std::ops::Deref for crate::System::Data::ParentForeignKeyConstraintEnumerator {
    type Target = crate::System::Data::ForeignKeyConstraintEnumerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
impl std::ops::DerefMut for crate::System::Data::ParentForeignKeyConstraintEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
impl crate::System::Data::ParentForeignKeyConstraintEnumerator {
    pub fn IsValidCandidate(
        &mut self,
        constraint: quest_hook::libil2cpp::Gc<crate::System::Data::Constraint>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidCandidate", (constraint))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        inTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataSet, inTable))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        inTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataSet, inTable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::ParentForeignKeyConstraintEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
