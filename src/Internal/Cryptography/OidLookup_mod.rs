#[cfg(feature = "Internal+Cryptography+OidLookup")]
#[repr(C)]
#[derive(Debug)]
pub struct OidLookup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Internal+Cryptography+OidLookup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Internal::Cryptography::OidLookup =>
    "Internal.Cryptography"."OidLookup"
);
#[cfg(feature = "Internal+Cryptography+OidLookup")]
impl std::ops::Deref for crate::Internal::Cryptography::OidLookup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Cryptography+OidLookup")]
impl std::ops::DerefMut for crate::Internal::Cryptography::OidLookup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Cryptography+OidLookup")]
impl crate::Internal::Cryptography::OidLookup {
    #[cfg(feature = "Internal+Cryptography+OidLookup+__c")]
    pub type __c = crate::Internal::Cryptography::OidLookup___c;
}
#[cfg(feature = "Internal+Cryptography+OidLookup")]
impl quest_hook::libil2cpp::ObjectType for crate::Internal::Cryptography::OidLookup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
