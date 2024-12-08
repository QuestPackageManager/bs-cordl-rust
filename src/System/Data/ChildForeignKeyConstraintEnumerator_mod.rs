#[cfg(feature = "System+Data+ChildForeignKeyConstraintEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ChildForeignKeyConstraintEnumerator {
    __cordl_parent: crate::System::Data::ForeignKeyConstraintEnumerator,
    pub _table: *mut crate::System::Data::DataTable,
}
#[cfg(feature = "System+Data+ChildForeignKeyConstraintEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Data::ChildForeignKeyConstraintEnumerator => "System.Data"
    ."ChildForeignKeyConstraintEnumerator"
);
#[cfg(feature = "System+Data+ChildForeignKeyConstraintEnumerator")]
impl std::ops::Deref for crate::System::Data::ChildForeignKeyConstraintEnumerator {
    type Target = crate::System::Data::ForeignKeyConstraintEnumerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ChildForeignKeyConstraintEnumerator")]
impl std::ops::DerefMut for crate::System::Data::ChildForeignKeyConstraintEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ChildForeignKeyConstraintEnumerator")]
impl crate::System::Data::ChildForeignKeyConstraintEnumerator {
    pub fn _ctor(
        &mut self,
        dataSet: *mut crate::System::Data::DataSet,
        inTable: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataSet, inTable))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidCandidate(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidCandidate", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        dataSet: *mut crate::System::Data::DataSet,
        inTable: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataSet, inTable))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+ChildForeignKeyConstraintEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::ChildForeignKeyConstraintEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
