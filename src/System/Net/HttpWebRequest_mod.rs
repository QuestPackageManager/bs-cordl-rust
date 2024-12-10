#[cfg(feature = "System+Net+HttpWebRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpWebRequest {
    __cordl_parent: crate::System::Net::WebRequest,
    pub requestUri: *mut crate::System::Uri,
    pub actualUri: *mut crate::System::Uri,
    pub hostChanged: bool,
    pub allowAutoRedirect: bool,
    pub allowBuffering: bool,
    pub certificates: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    pub connectionGroup: *mut quest_hook::libil2cpp::Il2CppString,
    pub haveContentLength: bool,
    pub contentLength: i64,
    pub continueDelegate: *mut crate::System::Net::HttpContinueDelegate,
    pub cookieContainer: *mut crate::System::Net::CookieContainer,
    pub credentials: *mut crate::System::Net::ICredentials,
    pub haveResponse: bool,
    pub requestSent: bool,
    pub webHeaders: *mut crate::System::Net::WebHeaderCollection,
    pub keepAlive: bool,
    pub maxAutoRedirect: i32,
    pub mediaType: *mut quest_hook::libil2cpp::Il2CppString,
    pub method: *mut quest_hook::libil2cpp::Il2CppString,
    pub initialMethod: *mut quest_hook::libil2cpp::Il2CppString,
    pub pipelined: bool,
    pub preAuthenticate: bool,
    pub usedPreAuth: bool,
    pub version: *mut crate::System::Version,
    pub force_version: bool,
    pub actualVersion: *mut crate::System::Version,
    pub proxy: *mut crate::System::Net::IWebProxy,
    pub sendChunked: bool,
    pub servicePoint: *mut crate::System::Net::ServicePoint,
    pub timeout: i32,
    pub continueTimeout: i32,
    pub writeStream: *mut crate::System::Net::WebRequestStream,
    pub webResponse: *mut crate::System::Net::HttpWebResponse,
    pub responseTask: *mut crate::System::Net::WebCompletionSource,
    pub currentOperation: *mut crate::System::Net::WebOperation,
    pub aborted: i32,
    pub gotRequestStream: bool,
    pub redirects: i32,
    pub expectContinue: bool,
    pub getResponseCalled: bool,
    pub locker: *mut quest_hook::libil2cpp::Il2CppObject,
    pub finished_reading: bool,
    pub auto_decomp: crate::System::Net::DecompressionMethods,
    pub readWriteTimeout: i32,
    pub tlsProvider: *mut crate::Mono::Net::Security::MobileTlsProvider,
    pub tlsSettings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
    pub certValidationCallback: *mut crate::System::Net::ServerCertValidationCallback,
    pub hostHasPort: bool,
    pub hostUri: *mut crate::System::Uri,
    pub auth_state: crate::System::Net::HttpWebRequest_AuthorizationState,
    pub proxy_auth_state: crate::System::Net::HttpWebRequest_AuthorizationState,
    pub ResendContentFactory: *mut crate::System::Func_2<
        *mut crate::System::IO::Stream,
        *mut crate::System::Threading::Tasks::Task,
    >,
    pub _ThrowOnError_k__BackingField: bool,
    pub unsafe_auth_blah: bool,
}
#[cfg(feature = "System+Net+HttpWebRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpWebRequest => "System.Net"
    ."HttpWebRequest"
);
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
    #[cfg(feature = "System+Net+HttpWebRequest+_GetResponseFromData_d__244")]
    pub type _GetResponseFromData_d__244 = crate::System::Net::HttpWebRequest__GetResponseFromData_d__244;
    #[cfg(feature = "System+Net+HttpWebRequest+_MyGetResponseAsync_d__243")]
    pub type _MyGetResponseAsync_d__243 = crate::System::Net::HttpWebRequest__MyGetResponseAsync_d__243;
    #[cfg(feature = "System+Net+HttpWebRequest+_RunWithTimeoutWorker_d__241_1")]
    pub type _RunWithTimeoutWorker_d__241_1<T: quest_hook::libil2cpp::Type> = crate::System::Net::HttpWebRequest__RunWithTimeoutWorker_d__241_1<
        T,
    >;
    #[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
    pub type __GetRewriteHandler_b__271_0_d = crate::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d;
    #[cfg(feature = "System+Net+HttpWebRequest+__c__241_1")]
    pub type __c__241_1<T: quest_hook::libil2cpp::Type> = crate::System::Net::HttpWebRequest___c__241_1<
        T,
    >;
    pub fn Abort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Abort", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginGetRequestStream(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginGetRequestStream", (callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginGetResponse(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginGetResponse", (callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckAuthorization(
        &mut self,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
        code: crate::System::Net::HttpStatusCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckAuthorization", (response, code))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckFinalStatus(
        &mut self,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_4<
            bool,
            bool,
            *mut crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::BufferOffsetSize,
            >,
            *mut crate::System::Net::WebException,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_4<
            bool,
            bool,
            *mut crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::BufferOffsetSize,
            >,
            *mut crate::System::Net::WebException,
        > = __cordl_object.invoke("CheckFinalStatus", (response))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckRequestStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckRequestStarted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoContinueDelegate(
        &mut self,
        statusCode: i32,
        headers: quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoContinueDelegate", (statusCode, headers))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoPreAuthenticate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoPreAuthenticate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EndGetRequestStream(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("EndGetRequestStream", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndGetResponse(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse> = __cordl_object
            .invoke("EndGetResponse", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHeaders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetHeaders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (serializationInfo, streamingContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRequestHeaders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetRequestHeaders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRequestStreamAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
        > = __cordl_object.invoke("GetRequestStreamAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse> = __cordl_object
            .invoke("GetResponse", ())?;
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
                    *mut crate::System::Net::HttpWebResponse,
                    bool,
                    bool,
                    *mut crate::System::Net::BufferOffsetSize,
                    *mut crate::System::Net::WebOperation,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_5<
                    *mut crate::System::Net::HttpWebResponse,
                    bool,
                    bool,
                    *mut crate::System::Net::BufferOffsetSize,
                    *mut crate::System::Net::WebOperation,
                >,
            >,
        > = __cordl_object.invoke("GetResponseFromData", (stream, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRewriteHandler(
        &mut self,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
        redirect: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            *mut crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::BufferOffsetSize,
            >,
            *mut crate::System::Net::WebException,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<
            *mut crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::BufferOffsetSize,
            >,
            *mut crate::System::Net::WebException,
        > = __cordl_object.invoke("GetRewriteHandler", (response, redirect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint> = __cordl_object
            .invoke("GetServicePoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWebException(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebException> = __cordl_object
            .invoke("GetWebException", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNtlmAuth(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponseStream>,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebResponse>,
        writeBuffer: quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<*mut crate::System::Net::WebOperation, bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<
            *mut crate::System::Net::WebOperation,
            bool,
        > = __cordl_object
            .invoke(
                "HandleNtlmAuth",
                (stream, response, writeBuffer, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MyGetRequestStreamAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
        > = __cordl_object.invoke("MyGetRequestStreamAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn MyGetResponseAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::HttpWebResponse,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::HttpWebResponse,
            >,
        > = __cordl_object.invoke("MyGetResponseAsync", (cancellationToken))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Redirect", (code, response))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetAuthorization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetAuthorization", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RewriteRedirectToGet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RewriteRedirectToGet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RunWithTimeout<T>(
        &mut self,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::System::Threading::CancellationToken,
                *mut crate::System::Threading::Tasks::Task_1<T>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = __cordl_object.invoke("RunWithTimeout", (func))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation> = __cordl_object
            .invoke("SendRequest", (redirecting, writeBuffer, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (serializationInfo, streamingContext),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetHostUri(
        &mut self,
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hostUri: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetHostUri", (hostName, hostUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn _GetRewriteHandler_b__271_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::BufferOffsetSize,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::BufferOffsetSize,
            >,
        > = __cordl_object.invoke("<GetRewriteHandler>b__271_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _RunWithTimeout_b__242_0<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("<RunWithTimeout>b__242_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Uri0(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uri))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uri, tlsProvider, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Aborted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Aborted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Address(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = __cordl_object
            .invoke("get_Address", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllowWriteStreamBuffering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_AllowWriteStreamBuffering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AuthUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = __cordl_object
            .invoke("get_AuthUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AutomaticDecompression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::DecompressionMethods> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::DecompressionMethods = __cordl_object
            .invoke("get_AutomaticDecompression", ())?;
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
    pub fn get_ContentLength(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ContentLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Credentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials> = __cordl_object
            .invoke("get_Credentials", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExpectContinue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ExpectContinue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Headers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::WebHeaderCollection,
        > = __cordl_object.invoke("get_Headers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Host(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Host", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalAllowBuffering(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_InternalAllowBuffering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeepAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_KeepAlive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Method", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MethodWithBuffer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_MethodWithBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProtocolVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = __cordl_object
            .invoke("get_ProtocolVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Proxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy> = __cordl_object
            .invoke("get_Proxy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProxyQuery(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ProxyQuery", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReadWriteTimeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ReadWriteTimeout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RequestUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = __cordl_object
            .invoke("get_RequestUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SendChunked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SendChunked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ServerCertValidationCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServerCertValidationCallback>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServerCertValidationCallback,
        > = __cordl_object.invoke("get_ServerCertValidationCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ServerCertificateValidationCallback(
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
        > = __cordl_object.invoke("get_ServerCertificateValidationCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint> = __cordl_object
            .invoke("get_ServicePoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ServicePointNoLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint> = __cordl_object
            .invoke("get_ServicePointNoLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ThrowOnError(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ThrowOnError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Timeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Timeout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TlsProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileTlsProvider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        > = __cordl_object.invoke("get_TlsProvider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TlsSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::MonoTlsSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        > = __cordl_object.invoke("get_TlsSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TransferEncoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_TransferEncoding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnsafeAuthenticatedConnectionSharing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_UnsafeAuthenticatedConnectionSharing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseDefaultCredentials(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseDefaultCredentials", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AllowAutoRedirect(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllowAutoRedirect", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AllowWriteStreamBuffering(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllowWriteStreamBuffering", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AutomaticDecompression(
        &mut self,
        value: crate::System::Net::DecompressionMethods,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AutomaticDecompression", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ConnectionGroupName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ConnectionGroupName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ContentLength(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContentLength", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CookieContainer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::CookieContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CookieContainer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Credentials(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Credentials", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ExpectContinue(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExpectContinue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FinishedReading(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FinishedReading", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Host(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Host", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InternalContentLength(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InternalContentLength", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_KeepAlive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeepAlive", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MaximumAutomaticRedirections(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaximumAutomaticRedirections", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Method(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Method", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PreAuthenticate(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PreAuthenticate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ProtocolVersion(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ProtocolVersion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Proxy(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Proxy", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SendChunked(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SendChunked", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ServerCertificateValidationCallback(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::RemoteCertificateValidationCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ServerCertificateValidationCallback", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ThrowOnError(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ThrowOnError", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Timeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Timeout", (value))?;
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
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HttpWebRequest_AuthorizationState {
    pub request: *mut crate::System::Net::HttpWebRequest,
    pub isProxy: bool,
    pub isCompleted: bool,
    pub ntlm_auth_state: crate::System::Net::HttpWebRequest_NtlmAuthState,
}
#[cfg(feature = "System+Net+HttpWebRequest+AuthorizationState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpWebRequest_AuthorizationState
    => "System.Net"."HttpWebRequest/AuthorizationState"
);
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckAuthorization",
            (response, code),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
        isProxy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (request, isProxy),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCompleted",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNtlmAuthenticated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNtlmAuthenticated",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NtlmAuthState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::HttpWebRequest_NtlmAuthState,
    > {
        let __cordl_ret: crate::System::Net::HttpWebRequest_NtlmAuthState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NtlmAuthState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+HttpWebRequest+NtlmAuthState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpWebRequest_NtlmAuthState {
    Challenge = 1i32,
    None = 0i32,
    Response = 2i32,
}
#[cfg(feature = "System+Net+HttpWebRequest+NtlmAuthState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpWebRequest_NtlmAuthState =>
    "System.Net"."HttpWebRequest/NtlmAuthState"
);
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HttpWebRequest___GetRewriteHandler_b__271_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder_1<
        *mut crate::System::Net::BufferOffsetSize,
    >,
    pub __4__this: *mut crate::System::Net::HttpWebRequest,
    pub _ms_5__2: *mut crate::System::IO::MemoryStream,
    pub __u__1: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter,
}
#[cfg(feature = "System+Net+HttpWebRequest+__GetRewriteHandler_b__271_0_d")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::HttpWebRequest___GetRewriteHandler_b__271_0_d => "System.Net"
    ."HttpWebRequest/<<GetRewriteHandler>b__271_0>d"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
