#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MobileTlsProvider {
    __cordl_parent: crate::Mono::Security::Interface::MonoTlsProvider,
}
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::MobileTlsProvider =>
    "Mono.Net.Security"."MobileTlsProvider"
);
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
impl std::ops::Deref for crate::Mono::Net::Security::MobileTlsProvider {
    type Target = crate::Mono::Security::Interface::MonoTlsProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
impl std::ops::DerefMut for crate::Mono::Net::Security::MobileTlsProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
impl crate::Mono::Net::Security::MobileTlsProvider {
    pub fn CreateSslStream(
        &mut self,
        sslStream: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        leaveInnerStreamOpen: bool,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileAuthenticatedStream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileAuthenticatedStream,
        > = __cordl_object
            .invoke(
                "CreateSslStream",
                (sslStream, innerStream, leaveInnerStreamOpen, settings),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ValidateCertificate(
        &mut self,
        validator: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::ChainValidationHelper,
        >,
        targetHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serverMode: bool,
        certificates: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        wantsChain: bool,
        chain: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Security::Cryptography::X509Certificates::X509Chain,
            >,
        >,
        errors: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Security::SslPolicyErrors,
        >,
        status11: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ValidateCertificate",
                (
                    validator,
                    targetHost,
                    serverMode,
                    certificates,
                    wantsChain,
                    chain,
                    errors,
                    status11,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::MobileTlsProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
