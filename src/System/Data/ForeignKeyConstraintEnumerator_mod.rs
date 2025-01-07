#[cfg(feature = "System+Data+ForeignKeyConstraintEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ForeignKeyConstraintEnumerator {
    __cordl_parent: crate::System::Data::ConstraintEnumerator,
}
#[cfg(feature = "System+Data+ForeignKeyConstraintEnumerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Data::ForeignKeyConstraintEnumerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "ForeignKeyConstraintEnumerator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::ForeignKeyConstraint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::ForeignKeyConstraint,
        > = __cordl_object.invoke("GetForeignKeyConstraint", ())?;
        Ok(__cordl_ret.into())
    }
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataSet))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataSet))?;
        Ok(__cordl_ret.into())
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
