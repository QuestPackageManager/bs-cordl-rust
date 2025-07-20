#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ParentForeignKeyConstraintEnumerator {
    __cordl_parent: crate::System::Data::ForeignKeyConstraintEnumerator,
    pub _table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
}
#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Data::ParentForeignKeyConstraintEnumerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "ParentForeignKeyConstraintEnumerator";
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
#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
impl std::ops::Deref for crate::System::Data::ParentForeignKeyConstraintEnumerator {
    type Target = crate::System::Data::ForeignKeyConstraintEnumerator;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
impl std::ops::DerefMut for crate::System::Data::ParentForeignKeyConstraintEnumerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ParentForeignKeyConstraintEnumerator")]
impl crate::System::Data::ParentForeignKeyConstraintEnumerator {
    pub fn IsValidCandidate(
        &mut self,
        constraint: quest_hook::libil2cpp::Gc<crate::System::Data::Constraint>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Data::Constraint>),
                        bool,
                        1usize,
                    >("IsValidCandidate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsValidCandidate", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (constraint))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dataSet, inTable))?
        };
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
