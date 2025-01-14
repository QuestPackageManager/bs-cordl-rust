#[cfg(feature = "System+Diagnostics+Contracts+Contract")]
#[repr(C)]
#[derive(Debug)]
pub struct Contract {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Diagnostics+Contracts+Contract")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Diagnostics::Contracts::Contract {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Diagnostics.Contracts";
    const CLASS_NAME: &'static str = "Contract";
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
#[cfg(feature = "System+Diagnostics+Contracts+Contract")]
impl std::ops::Deref for crate::System::Diagnostics::Contracts::Contract {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Contracts+Contract")]
impl std::ops::DerefMut for crate::System::Diagnostics::Contracts::Contract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Contracts+Contract")]
impl crate::System::Diagnostics::Contracts::Contract {
    pub fn ForAll<T>(
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<T>,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
                ),
                bool,
                2usize,
            >("ForAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ForAll", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (collection, predicate))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+Contracts+Contract")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::Contracts::Contract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
