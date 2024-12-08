#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
#[repr(C)]
#[derive(Debug)]
pub struct CryptoConvert {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::CryptoConvert =>
    "Mono.Security.Cryptography"."CryptoConvert"
);
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::CryptoConvert {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::CryptoConvert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
impl crate::Mono::Security::Cryptography::CryptoConvert {}
#[cfg(feature = "Mono+Security+Cryptography+CryptoConvert")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::CryptoConvert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
