#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::AsyncInitialization::AddressablesLoader
    => "BGLib.AsyncInitialization"."AddressablesLoader"
);
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl std::ops::Deref for crate::BGLib::AsyncInitialization::AddressablesLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl std::ops::DerefMut for crate::BGLib::AsyncInitialization::AddressablesLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl crate::BGLib::AsyncInitialization::AddressablesLoader {
    #[cfg(
        feature = "BGLib+AsyncInitialization+AddressablesLoader+_InstantiateFromAddressableToContainer_d__0_2"
    )]
    pub type _InstantiateFromAddressableToContainer_d__0_2<
        TInstantiate: quest_hook::libil2cpp::Type,
        TReturn: quest_hook::libil2cpp::Type,
    > = crate::BGLib::AsyncInitialization::AddressablesLoader__InstantiateFromAddressableToContainer_d__0_2<
        TInstantiate,
        TReturn,
    >;
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AsyncInitialization::AddressablesLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
