#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::AddressablesExtensions =>
    "BGLib.UnityExtension"."AddressablesExtensions"
);
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
impl std::ops::Deref for crate::BGLib::UnityExtension::AddressablesExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::AddressablesExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
impl crate::BGLib::UnityExtension::AddressablesExtensions {
    #[cfg(
        feature = "BGLib+UnityExtension+AddressablesExtensions+__c__DisplayClass3_0_1"
    )]
    pub type __c__DisplayClass3_0_1<T: quest_hook::libil2cpp::Type> = crate::BGLib::UnityExtension::AddressablesExtensions___c__DisplayClass3_0_1<
        T,
    >;
}
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::AddressablesExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
