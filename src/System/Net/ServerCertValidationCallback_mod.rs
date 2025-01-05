#[cfg(feature = "System+Net+ServerCertValidationCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct ServerCertValidationCallback {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_ValidationCallback: quest_hook::libil2cpp::Gc<
        crate::System::Net::Security::RemoteCertificateValidationCallback,
    >,
    pub m_Context: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
}
#[cfg(feature = "System+Net+ServerCertValidationCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ServerCertValidationCallback =>
    "System.Net"."ServerCertValidationCallback"
);
#[cfg(feature = "System+Net+ServerCertValidationCallback")]
impl std::ops::Deref for crate::System::Net::ServerCertValidationCallback {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServerCertValidationCallback")]
impl std::ops::DerefMut for crate::System::Net::ServerCertValidationCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServerCertValidationCallback")]
impl crate::System::Net::ServerCertValidationCallback {
    #[cfg(feature = "System+Net+ServerCertValidationCallback+CallbackContext")]
    pub type CallbackContext = crate::System::Net::ServerCertValidationCallback_CallbackContext;
    pub fn Callback(
        &mut self,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Callback", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        request: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        sslPolicyErrors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (request, certificate, chain, sslPolicyErrors))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        validationCallback: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::RemoteCertificateValidationCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (validationCallback))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        validationCallback: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::RemoteCertificateValidationCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (validationCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValidationCallback(
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
        > = __cordl_object.invoke("get_ValidationCallback", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ServerCertValidationCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::ServerCertValidationCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+ServerCertValidationCallback+CallbackContext")]
#[repr(C)]
#[derive(Debug)]
pub struct ServerCertValidationCallback_CallbackContext {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub request: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub certificate: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    >,
    pub chain: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Chain,
    >,
    pub sslPolicyErrors: crate::System::Net::Security::SslPolicyErrors,
    pub result: bool,
}
#[cfg(feature = "System+Net+ServerCertValidationCallback+CallbackContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::ServerCertValidationCallback_CallbackContext => "System.Net"
    ."ServerCertValidationCallback/CallbackContext"
);
#[cfg(feature = "System+Net+ServerCertValidationCallback+CallbackContext")]
impl std::ops::Deref
for crate::System::Net::ServerCertValidationCallback_CallbackContext {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServerCertValidationCallback+CallbackContext")]
impl std::ops::DerefMut
for crate::System::Net::ServerCertValidationCallback_CallbackContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServerCertValidationCallback+CallbackContext")]
impl crate::System::Net::ServerCertValidationCallback_CallbackContext {
    pub fn New(
        request: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        sslPolicyErrors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (request, certificate, chain, sslPolicyErrors))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        request: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        sslPolicyErrors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (request, certificate, chain, sslPolicyErrors))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ServerCertValidationCallback+CallbackContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::ServerCertValidationCallback_CallbackContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
