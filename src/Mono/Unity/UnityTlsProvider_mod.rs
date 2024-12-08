#[cfg(feature = "Mono+Unity+UnityTlsProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTlsProvider {
    __cordl_parent: crate::Mono::Net::Security::MobileTlsProvider,
}
#[cfg(feature = "Mono+Unity+UnityTlsProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTlsProvider => "Mono.Unity"
    ."UnityTlsProvider"
);
#[cfg(feature = "Mono+Unity+UnityTlsProvider")]
impl std::ops::Deref for crate::Mono::Unity::UnityTlsProvider {
    type Target = crate::Mono::Net::Security::MobileTlsProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsProvider")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTlsProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsProvider")]
impl crate::Mono::Unity::UnityTlsProvider {
    pub fn get_SupportsSslStream(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SupportsSslStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SupportsCleanShutdown(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SupportsCleanShutdown", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateSslStream(
        &mut self,
        sslStream: *mut crate::System::Net::Security::SslStream,
        innerStream: *mut crate::System::IO::Stream,
        leaveInnerStreamOpen: bool,
        settings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Net::Security::MobileAuthenticatedStream,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Net::Security::MobileAuthenticatedStream = __cordl_object
            .invoke(
                "CreateSslStream",
                (sslStream, innerStream, leaveInnerStreamOpen, settings),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ValidateCertificate(
        &mut self,
        validator: *mut crate::Mono::Net::Security::ChainValidationHelper,
        targetHost: *mut crate::System::String,
        serverMode: bool,
        certificates: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        wantsChain: bool,
        chain: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Security::Cryptography::X509Certificates::X509Chain,
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
        Ok(__cordl_ret)
    }
    pub fn get_SupportsConnectionInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SupportsConnectionInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ID(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object.invoke("get_ID", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SupportedProtocols(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Authentication::SslProtocols,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Authentication::SslProtocols = __cordl_object
            .invoke("get_SupportedProtocols", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SupportsMonoExtensions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SupportsMonoExtensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::UnityTlsProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
