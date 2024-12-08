#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoWebRequestHandler {
    __cordl_parent: crate::System::Object,
    pub allowAutoRedirect: bool,
    pub automaticDecompression: crate::System::Net::DecompressionMethods,
    pub cookieContainer: *mut crate::System::Net::CookieContainer,
    pub credentials: *mut crate::System::Net::ICredentials,
    pub maxAutomaticRedirections: i32,
    pub maxRequestContentBufferSize: i64,
    pub preAuthenticate: bool,
    pub proxy: *mut crate::System::Net::IWebProxy,
    pub useCookies: bool,
    pub useProxy: bool,
    pub sslOptions: *mut crate::System::Net::Security::SslClientAuthenticationOptions,
    pub allowPipelining: bool,
    pub cachePolicy: *mut crate::System::Net::Cache::RequestCachePolicy,
    pub authenticationLevel: crate::System::Net::Security::AuthenticationLevel,
    pub continueTimeout: crate::System::TimeSpan,
    pub impersonationLevel: crate::System::Security::Principal::TokenImpersonationLevel,
    pub maxResponseHeadersLength: i32,
    pub readWriteTimeout: i32,
    pub serverCertificateValidationCallback: *mut crate::System::Net::Security::RemoteCertificateValidationCallback,
    pub unsafeAuthenticatedConnectionSharing: bool,
    pub sentRequest: bool,
    pub connectionGroupName: *mut crate::System::String,
    pub timeout: crate::System::Nullable_1<crate::System::TimeSpan>,
    pub disposed: bool,
}
#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::MonoWebRequestHandler =>
    "System.Net.Http"."MonoWebRequestHandler"
);
#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
impl std::ops::Deref for crate::System::Net::Http::MonoWebRequestHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
impl std::ops::DerefMut for crate::System::Net::Http::MonoWebRequestHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
impl crate::System::Net::Http::MonoWebRequestHandler {
    #[cfg(feature = "System+Net+Http+MonoWebRequestHandler+__c")]
    pub type __c = crate::System::Net::Http::MonoWebRequestHandler___c;
    #[cfg(feature = "System+Net+Http+MonoWebRequestHandler+_SendAsync_d__99")]
    pub type _SendAsync_d__99 = crate::System::Net::Http::MonoWebRequestHandler__SendAsync_d__99;
    pub fn _CreateWebRequest_b__96_0(
        &mut self,
        t: *mut crate::System::String,
        lc: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        rc: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        ai: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate = __cordl_object
            .invoke("<CreateWebRequest>b__96_0", (t, lc, rc, ai))?;
        Ok(__cordl_ret)
    }
    pub fn GetConnectionKeepAlive(
        &mut self,
        headers: *mut crate::System::Net::Http::Headers::HttpRequestHeaders,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetConnectionKeepAlive", (headers))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
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
    pub fn get_CookieContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::CookieContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::CookieContainer = __cordl_object
            .invoke("get_CookieContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Net_Http_IMonoHttpClientHandler_SetWebRequestTimeout(
        &mut self,
        timeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Net.Http.IMonoHttpClientHandler.SetWebRequestTimeout",
                (timeout),
            )?;
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
    pub fn get_SslOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Security::SslClientAuthenticationOptions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Security::SslClientAuthenticationOptions = __cordl_object
            .invoke("get_SslOptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxRequestContentBufferSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_MaxRequestContentBufferSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateWebRequest(
        &mut self,
        request: *mut crate::System::Net::Http::HttpRequestMessage,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::HttpWebRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::HttpWebRequest = __cordl_object
            .invoke("CreateWebRequest", (request))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureModifiability(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureModifiability", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateResponseMessage(
        &mut self,
        wr: *mut crate::System::Net::HttpWebResponse,
        requestMessage: *mut crate::System::Net::Http::HttpRequestMessage,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Http::HttpResponseMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Http::HttpResponseMessage = __cordl_object
            .invoke("CreateResponseMessage", (wr, requestMessage, cancellationToken))?;
        Ok(__cordl_ret)
    }
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
    pub fn set_SslOptions(
        &mut self,
        value: *mut crate::System::Net::Security::SslClientAuthenticationOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SslOptions", (value))?;
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
#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::MonoWebRequestHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
