#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ChainValidationHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub owner: *mut crate::System::WeakReference_1<
        *mut crate::System::Net::Security::SslStream,
    >,
    pub settings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
    pub provider: *mut crate::Mono::Net::Security::MobileTlsProvider,
    pub certValidationCallback: *mut crate::System::Net::ServerCertValidationCallback,
    pub certSelectionCallback: *mut crate::System::Net::Security::LocalCertSelectionCallback,
    pub tlsStream: *mut crate::Mono::Net::Security::MonoTlsStream,
    pub request: *mut crate::System::Net::HttpWebRequest,
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::ChainValidationHelper =>
    "Mono.Net.Security"."ChainValidationHelper"
);
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl std::ops::Deref for crate::Mono::Net::Security::ChainValidationHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl std::ops::DerefMut for crate::Mono::Net::Security::ChainValidationHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl crate::Mono::Net::Security::ChainValidationHelper {
    #[cfg(feature = "Mono+Net+Security+ChainValidationHelper+__c__DisplayClass11_0")]
    pub type __c__DisplayClass11_0 = crate::Mono::Net::Security::ChainValidationHelper___c__DisplayClass11_0;
    pub fn InvokeCallback(
        &mut self,
        leaf: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        chain: *mut crate::System::Security::Cryptography::X509Certificates::X509Chain,
        errors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InvokeCallback", (leaf, chain, errors))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        owner: *mut crate::System::Net::Security::SslStream,
        provider: *mut crate::Mono::Net::Security::MobileTlsProvider,
        settings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
        cloneSettings: bool,
        stream: *mut crate::Mono::Net::Security::MonoTlsStream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (owner, provider, settings, cloneSettings, stream))?;
        Ok(__cordl_object)
    }
    pub fn SelectClientCertificate(
        &mut self,
        targetHost: *mut quest_hook::libil2cpp::Il2CppString,
        localCertificates: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        remoteCertificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        acceptableIssuers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        clientCertificate: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SelectClientCertificate",
                (
                    targetHost,
                    localCertificates,
                    remoteCertificate,
                    acceptableIssuers,
                    clientCertificate,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ValidateCertificate(
        &mut self,
        host: *mut quest_hook::libil2cpp::Il2CppString,
        serverMode: bool,
        leaf: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        chain: *mut crate::System::Security::Cryptography::X509Certificates::X509Chain,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::ValidationResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::ValidationResult = __cordl_object
            .invoke("ValidateCertificate", (host, serverMode, leaf, chain))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateChain_ByRefMut1(
        &mut self,
        host: *mut quest_hook::libil2cpp::Il2CppString,
        server: bool,
        leaf: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        chain: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        certs: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        errors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::ValidationResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::ValidationResult = __cordl_object
            .invoke("ValidateChain", (host, server, leaf, chain, certs, errors))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateChain_X509Chain0(
        &mut self,
        host: *mut quest_hook::libil2cpp::Il2CppString,
        server: bool,
        leaf: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        chain: *mut crate::System::Security::Cryptography::X509Certificates::X509Chain,
        certs: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        errors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::ValidationResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::ValidationResult = __cordl_object
            .invoke("ValidateChain", (host, server, leaf, chain, certs, errors))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        owner: *mut crate::System::Net::Security::SslStream,
        provider: *mut crate::Mono::Net::Security::MobileTlsProvider,
        settings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
        cloneSettings: bool,
        stream: *mut crate::Mono::Net::Security::MonoTlsStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (owner, provider, settings, cloneSettings, stream))?;
        Ok(__cordl_ret)
    }
    pub fn get_Settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::MonoTlsSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::MonoTlsSettings = __cordl_object
            .invoke("get_Settings", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::ChainValidationHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
