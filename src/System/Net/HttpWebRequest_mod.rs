#[cfg(feature = "System+Net+HttpWebRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpWebRequest {
    __cordl_parent: crate::System::Net::WebRequest,
    pub requestUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    pub actualUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    pub hostChanged: bool,
    pub allowAutoRedirect: bool,
    pub allowBuffering: bool,
    pub certificates: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    >,
    pub connectionGroup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub haveContentLength: bool,
    pub contentLength: i64,
    pub continueDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Net::HttpContinueDelegate,
    >,
    pub cookieContainer: quest_hook::libil2cpp::Gc<crate::System::Net::CookieContainer>,
    pub credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    pub haveResponse: bool,
    pub requestSent: bool,
    pub webHeaders: quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
    pub keepAlive: bool,
    pub maxAutoRedirect: i32,
    pub mediaType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub initialMethod: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub pipelined: bool,
    pub preAuthenticate: bool,
    pub usedPreAuth: bool,
    pub version: quest_hook::libil2cpp::Gc<crate::System::Version>,
    pub force_version: bool,
    pub actualVersion: quest_hook::libil2cpp::Gc<crate::System::Version>,
    pub proxy: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
    pub sendChunked: bool,
    pub servicePoint: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    pub timeout: i32,
    pub continueTimeout: i32,
    pub writeStream: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequestStream>,
    pub webResponse: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
    pub responseTask: quest_hook::libil2cpp::Gc<crate::System::Net::WebCompletionSource>,
    pub currentOperation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
    pub aborted: i32,
    pub gotRequestStream: bool,
    pub redirects: i32,
    pub expectContinue: bool,
    pub getResponseCalled: bool,
    pub locker: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub finished_reading: bool,
    pub auto_decomp: crate::System::Net::DecompressionMethods,
    pub readWriteTimeout: i32,
    pub tlsProvider: quest_hook::libil2cpp::Gc<
        crate::Mono::Net::Security::MobileTlsProvider,
    >,
    pub tlsSettings: quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Interface::MonoTlsSettings,
    >,
    pub certValidationCallback: quest_hook::libil2cpp::Gc<
        crate::System::Net::ServerCertValidationCallback,
    >,
    pub hostHasPort: bool,
    pub hostUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    pub auth_state: crate::System::Net::HttpWebRequest_AuthorizationState,
    pub proxy_auth_state: crate::System::Net::HttpWebRequest_AuthorizationState,
    pub ResendContentFactory: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
    >,
    pub _ThrowOnError_k__BackingField: bool,
    pub unsafe_auth_blah: bool,
}
#[cfg(feature = "System+Net+HttpWebRequest")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::HttpWebRequest {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "HttpWebRequest";
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
#[cfg(feature = "System+Net+HttpWebRequest")]
impl std::ops::Deref for crate::System::Net::HttpWebRequest {
    type Target = crate::System::Net::WebRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpWebRequest")]
impl std::ops::DerefMut for crate::System::Net::HttpWebRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpWebRequest")]
impl crate::System::Net::HttpWebRequest {
    #[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
    pub type AuthorizationState = crate::System::Net::HttpWebRequest_AuthorizationState;
    #[cfg(feature = "System+Net+HttpWebRequest+NtlmAuthState")]
    pub type NtlmAuthState = crate::System::Net::HttpWebRequest_NtlmAuthState;
    #[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
    pub type __GetRewriteHandler_b__271_0_d = crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d;
    pub fn Abort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Abort")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "Abort", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginGetRequestStream(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                2usize,
            >("BeginGetRequestStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "BeginGetRequestStream", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (callback, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginGetResponse(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                2usize,
            >("BeginGetResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "BeginGetResponse", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (callback, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckAuthorization(
        &mut self,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
        code: crate::System::Net::HttpStatusCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
                    crate::System::Net::HttpStatusCode,
                ),
                bool,
                2usize,
            >("CheckAuthorization")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "CheckAuthorization", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (response, code))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckFinalStatus(
        &mut self,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_4<
            bool,
            bool,
            quest_hook::libil2cpp::Gc<
                crate::System::Threading::Tasks::Task_1<
                    quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
                >,
            >,
            quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>),
                crate::System::ValueTuple_4<
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::BufferOffsetSize,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
                >,
                1usize,
            >("CheckFinalStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "CheckFinalStatus", 1usize
                )
            });
        let __cordl_ret: crate::System::ValueTuple_4<
            bool,
            bool,
            quest_hook::libil2cpp::Gc<
                crate::System::Threading::Tasks::Task_1<
                    quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
                >,
            >,
            quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
        > = unsafe { method.invoke_unchecked(self, (response))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckRequestStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("CheckRequestStarted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "CheckRequestStarted", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRequestAbortedException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
                0usize,
            >("CreateRequestAbortedException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "CreateRequestAbortedException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebException> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoContinueDelegate(
        &mut self,
        statusCode: i32,
        headers: quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DoContinueDelegate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "DoContinueDelegate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (statusCode, headers))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoPreAuthenticate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("DoPreAuthenticate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "DoPreAuthenticate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndGetRequestStream(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                1usize,
            >("EndGetRequestStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "EndGetRequestStream", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, (asyncResult))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndGetResponse(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
                1usize,
            >("EndGetResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "EndGetResponse", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse> = unsafe {
            method.invoke_unchecked(self, (asyncResult))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FlattenException(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Exception>),
                quest_hook::libil2cpp::Gc<crate::System::Exception>,
                1usize,
            >("FlattenException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "FlattenException", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = unsafe {
            method.invoke_unchecked((), (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHeaders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("GetHeaders")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetHeaders", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetObjectData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetObjectData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serializationInfo, streamingContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRequestHeaders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("GetRequestHeaders")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetRequestHeaders", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRequestStreamAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                    >,
                >,
                0usize,
            >("GetRequestStreamAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetRequestStreamAsync", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
                0usize,
            >("GetResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetResponse", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetResponseFromData(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponseStream>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_5<
                    quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebResponseStream>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        crate::System::ValueTuple_5<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::HttpWebResponse,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::BufferOffsetSize,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
                        >,
                    >,
                >,
                2usize,
            >("GetResponseFromData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetResponseFromData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_5<
                    quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, (stream, cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRewriteHandler(
        &mut self,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
        redirect: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Threading::Tasks::Task_1<
                    quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
                >,
            >,
            quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>, bool),
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::BufferOffsetSize,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
                >,
                2usize,
            >("GetRewriteHandler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetRewriteHandler", 2usize
                )
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Threading::Tasks::Task_1<
                    quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
                >,
            >,
            quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
        > = unsafe { method.invoke_unchecked(self, (response, redirect))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
                0usize,
            >("GetServicePoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetServicePoint", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWebException_Exception0(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Exception>),
                quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
                1usize,
            >("GetWebException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetWebException", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebException> = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWebException__cordl_bool1(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        aborted: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Exception>, bool),
                quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
                2usize,
            >("GetWebException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "GetWebException", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebException> = unsafe {
            method.invoke_unchecked((), (e, aborted))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleNtlmAuth(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponseStream>,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
        writeBuffer: quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
            bool,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebResponseStream>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
                    crate::System::Threading::CancellationToken,
                ),
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
                    bool,
                >,
                4usize,
            >("HandleNtlmAuth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "HandleNtlmAuth", 4usize
                )
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
            bool,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (stream, response, writeBuffer, cancellationToken),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MyGetRequestStreamAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Threading::CancellationToken),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                    >,
                >,
                1usize,
            >("MyGetRequestStreamAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "MyGetRequestStreamAsync", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
            >,
        > = unsafe { method.invoke_unchecked(self, (cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn MyGetResponseAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Threading::CancellationToken),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
                    >,
                >,
                1usize,
            >("MyGetResponseAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "MyGetResponseAsync", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
            >,
        > = unsafe { method.invoke_unchecked(self, (cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_3() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext2(
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Uri0(
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uri))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Uri_MobileTlsProvider_MonoTlsSettings1(
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        tlsProvider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uri, tlsProvider, settings))?;
        Ok(__cordl_object.into())
    }
    pub fn Redirect(
        &mut self,
        code: crate::System::Net::HttpStatusCode,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Net::HttpStatusCode,
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
                ),
                bool,
                2usize,
            >("Redirect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "Redirect", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (code, response))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetAuthorization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetAuthorization")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "ResetAuthorization", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RewriteRedirectToGet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RewriteRedirectToGet")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "RewriteRedirectToGet", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RunWithTimeoutWorker<T>(
        workerTask: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        >,
        timeout: i32,
        abort: quest_hook::libil2cpp::Gc<crate::System::Action>,
        aborted: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
        cts: quest_hook::libil2cpp::Gc<crate::System::Threading::CancellationTokenSource>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<T>,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::CancellationTokenSource,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
                5usize,
            >("RunWithTimeoutWorker")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "RunWithTimeoutWorker", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = unsafe {
            method.invoke_unchecked((), (workerTask, timeout, abort, aborted, cts))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RunWithTimeout_Func_2_1<T>(
        &mut self,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::System::Threading::CancellationToken,
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Func_2<
                        crate::System::Threading::CancellationToken,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<T>,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
                1usize,
            >("RunWithTimeout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "RunWithTimeout", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = unsafe { method.invoke_unchecked(self, (func))? };
        Ok(__cordl_ret.into())
    }
    pub fn RunWithTimeout_i32_Action_Func_1_CancellationToken0<T>(
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::System::Threading::CancellationToken,
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
            >,
        >,
        timeout: i32,
        abort: quest_hook::libil2cpp::Gc<crate::System::Action>,
        aborted: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            crate::System::Threading::CancellationToken,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task_1<T>,
                            >,
                        >,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
                5usize,
            >("RunWithTimeout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "RunWithTimeout", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (func, timeout, abort, aborted, cancellationToken),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendRequest(
        &mut self,
        redirecting: bool,
        writeBuffer: quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
                3usize,
            >("SendRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "SendRequest", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation> = unsafe {
            method.invoke_unchecked(self, (redirecting, writeBuffer, cancellationToken))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("System.Runtime.Serialization.ISerializable.GetObjectData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(),
                    "System.Runtime.Serialization.ISerializable.GetObjectData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serializationInfo, streamingContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetHostUri(
        &mut self,
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hostUri: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Uri>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    >,
                ),
                bool,
                2usize,
            >("TryGetHostUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "TryGetHostUri", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (hostName, hostUri))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _GetRewriteHandler_b__271_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
                    >,
                >,
                0usize,
            >("<GetRewriteHandler>b__271_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "<GetRewriteHandler>b__271_0", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _RunWithTimeout_b__242_0<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("<RunWithTimeout>b__242_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "<RunWithTimeout>b__242_0", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serializationInfo, streamingContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Uri0(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (uri))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Uri_MobileTlsProvider_MonoTlsSettings1(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        tlsProvider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::Gc<
                        crate::Mono::Net::Security::MobileTlsProvider,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Mono::Security::Interface::MonoTlsSettings,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (uri, tlsProvider, settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Aborted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_Aborted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_Aborted", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Address(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Uri>,
                0usize,
            >("get_Address")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_Address", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AllowWriteStreamBuffering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_AllowWriteStreamBuffering")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_AllowWriteStreamBuffering", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_AuthUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Uri>,
                0usize,
            >("get_AuthUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_AuthUri", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AutomaticDecompression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::DecompressionMethods> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Net::DecompressionMethods,
                0usize,
            >("get_AutomaticDecompression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_AutomaticDecompression", 0usize
                )
            });
        let __cordl_ret: crate::System::Net::DecompressionMethods = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ClientCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
                >,
                0usize,
            >("get_ClientCertificates")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ClientCertificates", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentLength(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("get_ContentLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ContentLength", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Credentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                0usize,
            >("get_Credentials")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_Credentials", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultMaximumErrorResponseLength() -> quest_hook::libil2cpp::Result<
        i32,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                i32,
                0usize,
            >("get_DefaultMaximumErrorResponseLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_DefaultMaximumErrorResponseLength", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultMaximumResponseHeadersLength() -> quest_hook::libil2cpp::Result<
        i32,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                i32,
                0usize,
            >("get_DefaultMaximumResponseHeadersLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_DefaultMaximumResponseHeadersLength", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ExpectContinue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_ExpectContinue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ExpectContinue", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Headers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
                0usize,
            >("get_Headers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_Headers", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::WebHeaderCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Host(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Host")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_Host", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalAllowBuffering(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_InternalAllowBuffering")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_InternalAllowBuffering", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_KeepAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_KeepAlive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_KeepAlive", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Method")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_Method", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_MethodWithBuffer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_MethodWithBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_MethodWithBuffer", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ProtocolVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Version>,
                0usize,
            >("get_ProtocolVersion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ProtocolVersion", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Proxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
                0usize,
            >("get_Proxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_Proxy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ProxyQuery(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_ProxyQuery")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ProxyQuery", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ReadWriteTimeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_ReadWriteTimeout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ReadWriteTimeout", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_RequestUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Uri>,
                0usize,
            >("get_RequestUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_RequestUri", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_SendChunked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_SendChunked")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_SendChunked", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ServerCertValidationCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServerCertValidationCallback>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::ServerCertValidationCallback,
                >,
                0usize,
            >("get_ServerCertValidationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ServerCertValidationCallback", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServerCertValidationCallback,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ServerCertificateValidationCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::RemoteCertificateValidationCallback,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::Security::RemoteCertificateValidationCallback,
                >,
                0usize,
            >("get_ServerCertificateValidationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ServerCertificateValidationCallback", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::RemoteCertificateValidationCallback,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
                0usize,
            >("get_ServicePoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ServicePoint", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ServicePointNoLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
                0usize,
            >("get_ServicePointNoLock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ServicePointNoLock", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ThrowOnError(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_ThrowOnError")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_ThrowOnError", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Timeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Timeout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_Timeout", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TlsProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileTlsProvider>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileTlsProvider>,
                0usize,
            >("get_TlsProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_TlsProvider", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TlsSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::MonoTlsSettings>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Mono::Security::Interface::MonoTlsSettings,
                >,
                0usize,
            >("get_TlsSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_TlsSettings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TransferEncoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_TransferEncoding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_TransferEncoding", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UnsafeAuthenticatedConnectionSharing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_UnsafeAuthenticatedConnectionSharing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_UnsafeAuthenticatedConnectionSharing", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UseDefaultCredentials(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_UseDefaultCredentials")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "get_UseDefaultCredentials", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_AllowAutoRedirect(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_AllowAutoRedirect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_AllowAutoRedirect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_AllowWriteStreamBuffering(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_AllowWriteStreamBuffering")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_AllowWriteStreamBuffering", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_AutomaticDecompression(
        &mut self,
        value: crate::System::Net::DecompressionMethods,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Net::DecompressionMethods),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_AutomaticDecompression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_AutomaticDecompression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ConnectionGroupName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ConnectionGroupName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_ConnectionGroupName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ContentLength(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ContentLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_ContentLength", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_CookieContainer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::CookieContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::CookieContainer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_CookieContainer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_CookieContainer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Credentials(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Credentials")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_Credentials", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ExpectContinue(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ExpectContinue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_ExpectContinue", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_FinishedReading(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_FinishedReading")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_FinishedReading", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Host(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Host")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_Host", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_InternalContentLength(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_InternalContentLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_InternalContentLength", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_KeepAlive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_KeepAlive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_KeepAlive", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_MaximumAutomaticRedirections(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_MaximumAutomaticRedirections")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_MaximumAutomaticRedirections", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Method(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Method")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_Method", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_PreAuthenticate(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_PreAuthenticate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_PreAuthenticate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ProtocolVersion(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Version>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ProtocolVersion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_ProtocolVersion", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Proxy(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Proxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_Proxy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_SendChunked(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_SendChunked")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_SendChunked", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ServerCertificateValidationCallback(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::RemoteCertificateValidationCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Net::Security::RemoteCertificateValidationCallback,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ServerCertificateValidationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_ServerCertificateValidationCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ThrowOnError(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ThrowOnError")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_ThrowOnError", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Timeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_Timeout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest as quest_hook::libil2cpp::Type >
                    ::class(), "set_Timeout", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+HttpWebRequest")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::HttpWebRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+HttpWebRequest")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Net::HttpWebRequest {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+HttpWebRequest")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Net::HttpWebRequest {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HttpWebRequest_AuthorizationState {
    pub request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
    pub isProxy: bool,
    pub isCompleted: bool,
    pub ntlm_auth_state: crate::System::Net::HttpWebRequest_NtlmAuthState,
}
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::HttpWebRequest_AuthorizationState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "HttpWebRequest/AuthorizationState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::HttpWebRequest_AuthorizationState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::HttpWebRequest_AuthorizationState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::HttpWebRequest_AuthorizationState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::HttpWebRequest_AuthorizationState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::HttpWebRequest_AuthorizationState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
impl crate::System::Net::HttpWebRequest_AuthorizationState {
    pub fn CheckAuthorization(
        &mut self,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
        code: crate::System::Net::HttpStatusCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest_AuthorizationState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
                    crate::System::Net::HttpStatusCode,
                ),
                bool,
                2usize,
            >("CheckAuthorization")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest_AuthorizationState as
                    quest_hook::libil2cpp::Type > ::class(), "CheckAuthorization", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (response, code))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest_AuthorizationState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest_AuthorizationState as
                    quest_hook::libil2cpp::Type > ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest_AuthorizationState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest_AuthorizationState as
                    quest_hook::libil2cpp::Type > ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
        isProxy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest_AuthorizationState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest_AuthorizationState as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (request, isProxy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest_AuthorizationState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest_AuthorizationState as
                    quest_hook::libil2cpp::Type > ::class(), "get_IsCompleted", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNtlmAuthenticated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest_AuthorizationState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsNtlmAuthenticated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest_AuthorizationState as
                    quest_hook::libil2cpp::Type > ::class(), "get_IsNtlmAuthenticated",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NtlmAuthState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::HttpWebRequest_NtlmAuthState,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest_AuthorizationState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Net::HttpWebRequest_NtlmAuthState,
                0usize,
            >("get_NtlmAuthState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest_AuthorizationState as
                    quest_hook::libil2cpp::Type > ::class(), "get_NtlmAuthState", 0usize
                )
            });
        let __cordl_ret: crate::System::Net::HttpWebRequest_NtlmAuthState = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+NtlmAuthState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpWebRequest_NtlmAuthState {
    #[default]
    Challenge = 1i32,
    None = 0i32,
    Response = 2i32,
}
#[cfg(feature = "System+Net+HttpWebRequest+NtlmAuthState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::HttpWebRequest_NtlmAuthState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "HttpWebRequest/NtlmAuthState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+NtlmAuthState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::HttpWebRequest_NtlmAuthState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+NtlmAuthState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::HttpWebRequest_NtlmAuthState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+NtlmAuthState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::HttpWebRequest_NtlmAuthState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+NtlmAuthState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::HttpWebRequest_NtlmAuthState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HttpWebRequest___GetRewriteHandler_b__271_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder_1<
        quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
    >,
    pub __4__this: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
    pub _ms_5__2: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    pub __u__1: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter,
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "HttpWebRequest/<<GetRewriteHandler>b__271_0>d";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
impl crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("MoveNext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d as
                    quest_hook::libil2cpp::Type > ::class(), "MoveNext", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::IAsyncStateMachine,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetStateMachine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d as
                    quest_hook::libil2cpp::Type > ::class(), "SetStateMachine", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (stateMachine))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
