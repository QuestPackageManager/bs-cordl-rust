#[cfg(feature = "System+Net+Security+CertificateHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Security+CertificateHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Security::CertificateHelper =>
    "System.Net.Security"."CertificateHelper"
);
#[cfg(feature = "System+Net+Security+CertificateHelper")]
impl std::ops::Deref for crate::System::Net::Security::CertificateHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+CertificateHelper")]
impl std::ops::DerefMut for crate::System::Net::Security::CertificateHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+CertificateHelper")]
impl crate::System::Net::Security::CertificateHelper {
    pub fn GetEligibleClientCertificate_2() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEligibleClientCertificate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEligibleClientCertificate_X509Certificate2Collection1(
        candidateCerts: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEligibleClientCertificate", (candidateCerts))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEligibleClientCertificate_X509CertificateCollection0(
        candidateCerts: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEligibleClientCertificate", (candidateCerts))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidClientCertificate(
        cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidClientCertificate", (cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidForClientAuthenticationEKU(
        eku: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509EnhancedKeyUsageExtension,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidForClientAuthenticationEKU", (eku))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidForDigitalSignatureUsage(
        ku: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidForDigitalSignatureUsage", (ku))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Security+CertificateHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Security::CertificateHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
