#[cfg(feature = "System+Net+Http+HttpClientHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpClientHandler {
    __cordl_parent: crate::System::Net::Http::HttpMessageHandler,
    pub _delegatingHandler: *mut crate::System::Net::Http::IMonoHttpClientHandler,
    pub _clientCertificateOptions: crate::System::Net::Http::ClientCertificateOption,
}
#[cfg(feature = "System+Net+Http+HttpClientHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::HttpClientHandler =>
    "System.Net.Http"."HttpClientHandler"
);
#[cfg(feature = "System+Net+Http+HttpClientHandler")]
impl std::ops::Deref for crate::System::Net::Http::HttpClientHandler {
    type Target = crate::System::Net::Http::HttpMessageHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpClientHandler")]
impl std::ops::DerefMut for crate::System::Net::Http::HttpClientHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpClientHandler")]
impl crate::System::Net::Http::HttpClientHandler {
    #[cfg(feature = "System+Net+Http+HttpClientHandler+__c")]
    pub type __c = crate::System::Net::Http::HttpClientHandler___c;
    pub fn SendAsync(
        &mut self,
        request: *mut crate::System::Net::Http::HttpRequestMessage,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Net::Http::HttpResponseMessage,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Net::Http::HttpResponseMessage,
        > = __cordl_object.invoke("SendAsync", (request, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowForModifiedManagedSslOptionsIfStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowForModifiedManagedSslOptionsIfStarted", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ClientCertificateOptions(
        &mut self,
        value: crate::System::Net::Http::ClientCertificateOption,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ClientCertificateOptions", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IMonoHttpClientHandler1(
        &mut self,
        handler: *mut crate::System::Net::Http::IMonoHttpClientHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handler))?;
        Ok(__cordl_ret)
    }
    pub fn get_ClientCertificateOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::Http::ClientCertificateOption,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Http::ClientCertificateOption = __cordl_object
            .invoke("get_ClientCertificateOptions", ())?;
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
    pub fn SetWebRequestTimeout(
        &mut self,
        timeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWebRequestTimeout", (timeout))?;
        Ok(__cordl_ret)
    }
    pub fn _set_ClientCertificateOptions_b__23_0(
        &mut self,
        sender: *mut crate::System::Object,
        targetHost: *mut crate::System::String,
        localCertificates: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        remoteCertificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        acceptableIssuers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate = __cordl_object
            .invoke(
                "<set_ClientCertificateOptions>b__23_0",
                (
                    sender,
                    targetHost,
                    localCertificates,
                    remoteCertificate,
                    acceptableIssuers,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IMonoHttpClientHandler1(
        handler: *mut crate::System::Net::Http::IMonoHttpClientHandler,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handler))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+Http+HttpClientHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::HttpClientHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
