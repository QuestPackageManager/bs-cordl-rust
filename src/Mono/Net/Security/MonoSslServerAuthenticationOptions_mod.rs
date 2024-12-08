#[cfg(feature = "Mono+Net+Security+MonoSslServerAuthenticationOptions")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoSslServerAuthenticationOptions {
    __cordl_parent: crate::Mono::Net::Security::MonoSslAuthenticationOptions,
    pub _Options_k__BackingField: *mut crate::System::Net::Security::SslServerAuthenticationOptions,
}
#[cfg(feature = "Mono+Net+Security+MonoSslServerAuthenticationOptions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Net::Security::MonoSslServerAuthenticationOptions => "Mono.Net.Security"
    ."MonoSslServerAuthenticationOptions"
);
#[cfg(feature = "Mono+Net+Security+MonoSslServerAuthenticationOptions")]
impl std::ops::Deref for crate::Mono::Net::Security::MonoSslServerAuthenticationOptions {
    type Target = crate::Mono::Net::Security::MonoSslAuthenticationOptions;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MonoSslServerAuthenticationOptions")]
impl std::ops::DerefMut
for crate::Mono::Net::Security::MonoSslServerAuthenticationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MonoSslServerAuthenticationOptions")]
impl crate::Mono::Net::Security::MonoSslServerAuthenticationOptions {
    pub fn set_EncryptionPolicy(
        &mut self,
        value: crate::System::Net::Security::EncryptionPolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EncryptionPolicy", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ClientCertificateRequired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ClientCertificateRequired", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ServerCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate = __cordl_object
            .invoke("get_ServerCertificate", ())?;
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
    pub fn get_TargetHost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_TargetHost", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_EnabledSslProtocols(
        &mut self,
        value: crate::System::Security::Authentication::SslProtocols,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnabledSslProtocols", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CertificateRevocationCheckMode(
        &mut self,
        value: crate::System::Security::Cryptography::X509Certificates::X509RevocationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CertificateRevocationCheckMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ClientCertificates(
        &mut self,
        value: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ClientCertificates", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ClientCertificateRequired(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ClientCertificateRequired", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ServerCertificate(
        &mut self,
        value: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ServerCertificate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Options(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Security::SslServerAuthenticationOptions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Security::SslServerAuthenticationOptions = __cordl_object
            .invoke("get_Options", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ServerMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ServerMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EnabledSslProtocols(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Authentication::SslProtocols,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Authentication::SslProtocols = __cordl_object
            .invoke("get_EnabledSslProtocols", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ClientCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection = __cordl_object
            .invoke("get_ClientCertificates", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_TargetHost(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TargetHost", (value))?;
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
#[cfg(feature = "Mono+Net+Security+MonoSslServerAuthenticationOptions")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::MonoSslServerAuthenticationOptions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
