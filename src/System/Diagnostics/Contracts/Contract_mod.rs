#[cfg(feature = "System+Diagnostics+Contracts+Contract")]
#[repr(C)]
#[derive(Debug)]
pub struct Contract {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Diagnostics+Contracts+Contract")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Contracts::Contract =>
    "System.Diagnostics.Contracts"."Contract"
);
#[cfg(feature = "System+Diagnostics+Contracts+Contract")]
impl std::ops::Deref for crate::System::Diagnostics::Contracts::Contract {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        collection: quest_hook::libil2cpp::Gc<T>,
        predicate: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ForAll", (collection, predicate))?;
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
