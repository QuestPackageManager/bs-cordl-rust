#[cfg(feature = "System+Data+ForeignKeyConstraintEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ForeignKeyConstraintEnumerator {
    __cordl_parent: crate::System::Data::ConstraintEnumerator,
}
#[cfg(feature = "System+Data+ForeignKeyConstraintEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::ForeignKeyConstraintEnumerator =>
    "System.Data"."ForeignKeyConstraintEnumerator"
);
#[cfg(feature = "System+Data+ForeignKeyConstraintEnumerator")]
impl std::ops::Deref for crate::System::Data::ForeignKeyConstraintEnumerator {
    type Target = crate::System::Data::ConstraintEnumerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ForeignKeyConstraintEnumerator")]
impl std::ops::DerefMut for crate::System::Data::ForeignKeyConstraintEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ForeignKeyConstraintEnumerator")]
impl crate::System::Data::ForeignKeyConstraintEnumerator {
    pub fn GetForeignKeyConstraint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::ForeignKeyConstraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::ForeignKeyConstraint = __cordl_object
            .invoke("GetForeignKeyConstraint", ())?;
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
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataSet))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        dataSet: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataSet))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+ForeignKeyConstraintEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::ForeignKeyConstraintEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
