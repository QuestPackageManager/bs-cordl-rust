#[cfg(feature = "Mono+Unity+CertHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CertHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Unity+CertHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::CertHelper => "Mono.Unity"
    ."CertHelper"
);
#[cfg(feature = "Mono+Unity+CertHelper")]
impl std::ops::Deref for crate::Mono::Unity::CertHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+CertHelper")]
impl std::ops::DerefMut for crate::Mono::Unity::CertHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+CertHelper")]
impl crate::Mono::Unity::CertHelper {
    pub fn AddCertificateToNativeChain(
        nativeCertificateChain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddCertificateToNativeChain",
                (nativeCertificateChain, certificate, errorState),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCertificatesToNativeChain(
        nativeCertificateChain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        certificates: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddCertificatesToNativeChain",
                (nativeCertificateChain, certificates, errorState),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+CertHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::CertHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
