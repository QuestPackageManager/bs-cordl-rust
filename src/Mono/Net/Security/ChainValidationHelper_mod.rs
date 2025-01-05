#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ChainValidationHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub owner: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
    >,
    pub settings: quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Interface::MonoTlsSettings,
    >,
    pub provider: quest_hook::libil2cpp::Gc<
        crate::Mono::Net::Security::MobileTlsProvider,
    >,
    pub certValidationCallback: quest_hook::libil2cpp::Gc<
        crate::System::Net::ServerCertValidationCallback,
    >,
    pub certSelectionCallback: quest_hook::libil2cpp::Gc<
        crate::System::Net::Security::LocalCertSelectionCallback,
    >,
    pub tlsStream: quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MonoTlsStream>,
    pub request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::ChainValidationHelper =>
    "Mono.Net.Security"."ChainValidationHelper"
);
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl std::ops::Deref for crate::Mono::Net::Security::ChainValidationHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn Create(
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::MonoTlsSettings>,
        >,
        stream: quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MonoTlsStream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::ChainValidationHelper>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::ChainValidationHelper,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (provider, settings, stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultSelectionCallback(
        targetHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localCertificates: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        remoteCertificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        acceptableIssuers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DefaultSelectionCallback",
                (targetHost, localCertificates, remoteCertificate, acceptableIssuers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInternalValidator(
        owner: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::ChainValidationHelper>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::ChainValidationHelper,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInternalValidator", (owner, provider, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValidationCallback(
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServerCertValidationCallback>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServerCertValidationCallback,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValidationCallback", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallback(
        &mut self,
        leaf: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        errors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InvokeCallback", (leaf, chain, errors))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        owner: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
        cloneSettings: bool,
        stream: quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MonoTlsStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (owner, provider, settings, cloneSettings, stream))?;
        Ok(__cordl_object.into())
    }
    pub fn SelectClientCertificate(
        &mut self,
        targetHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localCertificates: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        remoteCertificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        acceptableIssuers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        clientCertificate: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Security::Cryptography::X509Certificates::X509Certificate,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn ValidateCertificate(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serverMode: bool,
        leaf: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::ValidationResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::ValidationResult,
        > = __cordl_object
            .invoke("ValidateCertificate", (host, serverMode, leaf, chain))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateChain_ByRefMut1(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        server: bool,
        leaf: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Security::Cryptography::X509Certificates::X509Chain,
            >,
        >,
        certs: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        errors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::ValidationResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::ValidationResult,
        > = __cordl_object
            .invoke("ValidateChain", (host, server, leaf, chain, certs, errors))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateChain_Gc0(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        server: bool,
        leaf: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        certs: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        errors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::ValidationResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::ValidationResult,
        > = __cordl_object
            .invoke("ValidateChain", (host, server, leaf, chain, certs, errors))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
        cloneSettings: bool,
        stream: quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MonoTlsStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (owner, provider, settings, cloneSettings, stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::MonoTlsSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        > = __cordl_object.invoke("get_Settings", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::ICertificateValidator>,
> for crate::Mono::Net::Security::ChainValidationHelper {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Interface::ICertificateValidator,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::ICertificateValidator>,
> for crate::Mono::Net::Security::ChainValidationHelper {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Interface::ICertificateValidator,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
