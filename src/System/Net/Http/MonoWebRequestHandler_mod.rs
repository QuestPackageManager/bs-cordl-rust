#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoWebRequestHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    pub connectionGroupName: *mut quest_hook::libil2cpp::Il2CppString,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CreateResponseMessage(
        &mut self,
        wr: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
        requestMessage: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::HttpRequestMessage,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::HttpResponseMessage,
        > = __cordl_object
            .invoke("CreateResponseMessage", (wr, requestMessage, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateWebRequest(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest> = __cordl_object
            .invoke("CreateWebRequest", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn EnsureModifiability(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureModifiability", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConnectionKeepAlive(
        &mut self,
        headers: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::Headers::HttpRequestHeaders,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetConnectionKeepAlive", (headers))?;
        Ok(__cordl_ret.into())
    }
    pub fn MethodHasBody(
        method: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMethod>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MethodHasBody", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SendAsync(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::Http::HttpResponseMessage,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::Http::HttpResponseMessage,
            >,
        > = __cordl_object.invoke("SendAsync", (request, cancellationToken))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _CreateWebRequest_b__96_0(
        &mut self,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lc: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        rc: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        ai: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        > = __cordl_object.invoke("<CreateWebRequest>b__96_0", (t, lc, rc, ai))?;
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
    pub fn get_CookieContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::CookieContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::CookieContainer,
        > = __cordl_object.invoke("get_CookieContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxRequestContentBufferSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_MaxRequestContentBufferSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SslOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::SslClientAuthenticationOptions,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::SslClientAuthenticationOptions,
        > = __cordl_object.invoke("get_SslOptions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SslOptions(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::SslClientAuthenticationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SslOptions", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
impl AsRef<crate::System::IDisposable>
for crate::System::Net::Http::MonoWebRequestHandler {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
impl AsMut<crate::System::IDisposable>
for crate::System::Net::Http::MonoWebRequestHandler {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
impl AsRef<crate::System::Net::Http::IMonoHttpClientHandler>
for crate::System::Net::Http::MonoWebRequestHandler {
    fn as_ref(&self) -> &crate::System::Net::Http::IMonoHttpClientHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Http+MonoWebRequestHandler")]
impl AsMut<crate::System::Net::Http::IMonoHttpClientHandler>
for crate::System::Net::Http::MonoWebRequestHandler {
    fn as_mut(&mut self) -> &mut crate::System::Net::Http::IMonoHttpClientHandler {
        unsafe { std::mem::transmute(self) }
    }
}
