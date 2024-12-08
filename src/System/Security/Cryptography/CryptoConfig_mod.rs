#[cfg(feature = "System+Security+Cryptography+CryptoConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct CryptoConfig {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Security+Cryptography+CryptoConfig")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::CryptoConfig =>
    "System.Security.Cryptography"."CryptoConfig"
);
#[cfg(feature = "System+Security+Cryptography+CryptoConfig")]
impl std::ops::Deref for crate::System::Security::Cryptography::CryptoConfig {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+CryptoConfig")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::CryptoConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+CryptoConfig")]
impl crate::System::Security::Cryptography::CryptoConfig {}
#[cfg(feature = "System+Security+Cryptography+CryptoConfig")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::CryptoConfig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
