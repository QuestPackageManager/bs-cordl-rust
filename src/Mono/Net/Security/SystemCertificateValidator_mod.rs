#[cfg(feature = "Mono+Net+Security+SystemCertificateValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemCertificateValidator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Net+Security+SystemCertificateValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::SystemCertificateValidator
    => "Mono.Net.Security"."SystemCertificateValidator"
);
#[cfg(feature = "Mono+Net+Security+SystemCertificateValidator")]
impl std::ops::Deref for crate::Mono::Net::Security::SystemCertificateValidator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+SystemCertificateValidator")]
impl std::ops::DerefMut for crate::Mono::Net::Security::SystemCertificateValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+SystemCertificateValidator")]
impl crate::Mono::Net::Security::SystemCertificateValidator {
    pub fn NeedsChain(
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NeedsChain", (settings))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Net+Security+SystemCertificateValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::SystemCertificateValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
