#[cfg(feature = "System+Net+Security+SslClientAuthenticationOptions")]
#[repr(C)]
#[derive(Debug)]
pub struct SslClientAuthenticationOptions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _encryptionPolicy: crate::System::Net::Security::EncryptionPolicy,
    pub _checkCertificateRevocation: crate::System::Security::Cryptography::X509Certificates::X509RevocationMode,
    pub _enabledSslProtocols: crate::System::Security::Authentication::SslProtocols,
    pub _allowRenegotiation: bool,
    pub _LocalCertificateSelectionCallback_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::Security::LocalCertificateSelectionCallback,
    >,
    pub _RemoteCertificateValidationCallback_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::Security::RemoteCertificateValidationCallback,
    >,
    pub _TargetHost_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _ClientCertificates_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    >,
}
#[cfg(feature = "System+Net+Security+SslClientAuthenticationOptions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Security::SslClientAuthenticationOptions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Security";
    const CLASS_NAME: &'static str = "SslClientAuthenticationOptions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Net+Security+SslClientAuthenticationOptions")]
impl std::ops::Deref for crate::System::Net::Security::SslClientAuthenticationOptions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+SslClientAuthenticationOptions")]
impl std::ops::DerefMut
for crate::System::Net::Security::SslClientAuthenticationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+SslClientAuthenticationOptions")]
impl crate::System::Net::Security::SslClientAuthenticationOptions {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_ClientCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        > = __cordl_object.invoke("get_ClientCertificates", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalCertificateSelectionCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::LocalCertificateSelectionCallback,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::LocalCertificateSelectionCallback,
        > = __cordl_object.invoke("get_LocalCertificateSelectionCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RemoteCertificateValidationCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::RemoteCertificateValidationCallback,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::RemoteCertificateValidationCallback,
        > = __cordl_object.invoke("get_RemoteCertificateValidationCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetHost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_TargetHost", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_ClientCertificates(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ClientCertificates", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_EncryptionPolicy(
        &mut self,
        value: crate::System::Net::Security::EncryptionPolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EncryptionPolicy", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LocalCertificateSelectionCallback(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::LocalCertificateSelectionCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LocalCertificateSelectionCallback", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TargetHost(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TargetHost", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Security+SslClientAuthenticationOptions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Security::SslClientAuthenticationOptions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
