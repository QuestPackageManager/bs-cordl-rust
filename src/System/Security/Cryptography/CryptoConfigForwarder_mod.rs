#[cfg(feature = "System+Security+Cryptography+CryptoConfigForwarder")]
#[repr(C)]
#[derive(Debug)]
pub struct CryptoConfigForwarder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+Cryptography+CryptoConfigForwarder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::CryptoConfigForwarder =>
    "System.Security.Cryptography"."CryptoConfigForwarder"
);
#[cfg(feature = "System+Security+Cryptography+CryptoConfigForwarder")]
impl std::ops::Deref for crate::System::Security::Cryptography::CryptoConfigForwarder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+CryptoConfigForwarder")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::CryptoConfigForwarder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+CryptoConfigForwarder")]
impl crate::System::Security::Cryptography::CryptoConfigForwarder {}
#[cfg(feature = "System+Security+Cryptography+CryptoConfigForwarder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::CryptoConfigForwarder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
