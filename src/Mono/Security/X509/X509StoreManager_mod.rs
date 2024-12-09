#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
#[repr(C)]
#[derive(Debug)]
pub struct X509StoreManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::X509StoreManager =>
    "Mono.Security.X509"."X509StoreManager"
);
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
impl std::ops::Deref for crate::Mono::Security::X509::X509StoreManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
impl std::ops::DerefMut for crate::Mono::Security::X509::X509StoreManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
impl crate::Mono::Security::X509::X509StoreManager {}
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::X509::X509StoreManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
