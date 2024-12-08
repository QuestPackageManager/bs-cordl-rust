#[cfg(feature = "System+Net+FtpWebRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpWebRequest {
    __cordl_parent: crate::System::Net::WebRequest,
    pub _syncObject: *mut crate::System::Object,
    pub _authInfo: *mut crate::System::Net::ICredentials,
    pub _uri: *mut crate::System::Uri,
    pub _methodInfo: *mut crate::System::Net::FtpMethodInfo,
    pub _renameTo: *mut crate::System::String,
    pub _getRequestStreamStarted: bool,
    pub _getResponseStarted: bool,
    pub _startTime: crate::System::DateTime,
    pub _timeout: i32,
    pub _remainingTimeout: i32,
    pub _contentLength: i64,
    pub _contentOffset: i64,
    pub _clientCertificates: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    pub _passive: bool,
    pub _binary: bool,
    pub _connectionGroupName: *mut crate::System::String,
    pub _async: bool,
    pub _aborted: bool,
    pub _timedOut: bool,
    pub _exception: *mut crate::System::Exception,
    pub _timerQueue: *mut crate::System::Net::TimerThread_Queue,
    pub _timerCallback: *mut crate::System::Net::TimerThread_Callback,
    pub _enableSsl: bool,
    pub _connection: *mut crate::System::Net::FtpControlStream,
    pub _stream: *mut crate::System::IO::Stream,
    pub _requestStage: crate::System::Net::FtpWebRequest_RequestStage,
    pub _onceFailed: bool,
    pub _ftpRequestHeaders: *mut crate::System::Net::WebHeaderCollection,
    pub _ftpWebResponse: *mut crate::System::Net::FtpWebResponse,
    pub _readWriteTimeout: i32,
    pub _writeAsyncResult: *mut crate::System::Net::ContextAwareResult,
    pub _readAsyncResult: *mut crate::System::Net::LazyAsyncResult,
    pub _requestCompleteAsyncResult: *mut crate::System::Net::LazyAsyncResult,
}
#[cfg(feature = "System+Net+FtpWebRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpWebRequest => "System.Net"
    ."FtpWebRequest"
);
#[cfg(feature = "System+Net+FtpWebRequest")]
impl std::ops::Deref for crate::System::Net::FtpWebRequest {
    type Target = crate::System::Net::WebRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpWebRequest")]
impl std::ops::DerefMut for crate::System::Net::FtpWebRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpWebRequest")]
impl crate::System::Net::FtpWebRequest {
    #[cfg(feature = "System+Net+FtpWebRequest+__c")]
    pub type __c = crate::System::Net::FtpWebRequest___c;
    #[cfg(feature = "System+Net+FtpWebRequest+RequestStage")]
    pub type RequestStage = crate::System::Net::FtpWebRequest_RequestStage;
    #[cfg(feature = "System+Net+FtpWebRequest+_CreateConnectionAsync_d__86")]
    pub type _CreateConnectionAsync_d__86 = crate::System::Net::FtpWebRequest__CreateConnectionAsync_d__86;
    pub fn Abort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Abort", ())?;
        Ok(__cordl_ret)
    }
    pub fn AsyncRequestCallback(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AsyncRequestCallback", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn AttemptedRecovery(
        &mut self,
        e: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AttemptedRecovery", (e))?;
        Ok(__cordl_ret)
    }
    pub fn BeginGetRequestStream(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginGetRequestStream", (callback, state))?;
        Ok(__cordl_ret)
    }
    pub fn BeginGetResponse(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginGetResponse", (callback, state))?;
        Ok(__cordl_ret)
    }
    pub fn CheckError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckError", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::FtpControlStream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::FtpControlStream = __cordl_object
            .invoke("CreateConnection", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateConnectionAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateConnectionAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn DataStreamClosed(
        &mut self,
        closeState: crate::System::Net::CloseExState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DataStreamClosed", (closeState))?;
        Ok(__cordl_ret)
    }
    pub fn EndGetRequestStream(
        &mut self,
        asyncResult: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("EndGetRequestStream", (asyncResult))?;
        Ok(__cordl_ret)
    }
    pub fn EndGetResponse(
        &mut self,
        asyncResult: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebResponse> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebResponse = __cordl_object
            .invoke("EndGetResponse", (asyncResult))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureFtpWebResponse(
        &mut self,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureFtpWebResponse", (exception))?;
        Ok(__cordl_ret)
    }
    pub fn FinishRequestStage(
        &mut self,
        stage: crate::System::Net::FtpWebRequest_RequestStage,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::FtpWebRequest_RequestStage> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::FtpWebRequest_RequestStage = __cordl_object
            .invoke("FinishRequestStage", (stage))?;
        Ok(__cordl_ret)
    }
    pub fn GetResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebResponse> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebResponse = __cordl_object
            .invoke("GetResponse", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        uri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uri))?;
        Ok(__cordl_object)
    }
    pub fn RequestCallback(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestCallback", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn SetException(
        &mut self,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetException", (exception))?;
        Ok(__cordl_ret)
    }
    pub fn SubmitRequest(
        &mut self,
        isAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubmitRequest", (isAsync))?;
        Ok(__cordl_ret)
    }
    pub fn SyncRequestCallback(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncRequestCallback", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn TimedSubmitRequestHelper(
        &mut self,
        isAsync: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("TimedSubmitRequestHelper", (isAsync))?;
        Ok(__cordl_ret)
    }
    pub fn TimerCallback(
        &mut self,
        timer: *mut crate::System::Net::TimerThread_Timer,
        timeNoticed: i32,
        context: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TimerCallback", (timer, timeNoticed, context))?;
        Ok(__cordl_ret)
    }
    pub fn TranslateConnectException(
        &mut self,
        e: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("TranslateConnectException", (e))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        uri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uri))?;
        Ok(__cordl_ret)
    }
    pub fn get_Aborted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Aborted", ())?;
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
    pub fn get_ContentLength(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ContentLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentOffset(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ContentOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Credentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::ICredentials> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ICredentials = __cordl_object
            .invoke("get_Credentials", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EnableSsl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EnableSsl", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Headers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebHeaderCollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebHeaderCollection = __cordl_object
            .invoke("get_Headers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InUse(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_InUse", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Method", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MethodInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::FtpMethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::FtpMethodInfo = __cordl_object
            .invoke("get_MethodInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Proxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IWebProxy> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IWebProxy = __cordl_object
            .invoke("get_Proxy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReadWriteTimeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ReadWriteTimeout", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RemainingTimeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RemainingTimeout", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RenameTo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_RenameTo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RequestUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("get_RequestUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Timeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Timeout", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TimerQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::TimerThread_Queue> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::TimerThread_Queue = __cordl_object
            .invoke("get_TimerQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseBinary(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseBinary", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseDefaultCredentials(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseDefaultCredentials", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UsePassive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UsePassive", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CachePolicy(
        &mut self,
        value: *mut crate::System::Net::Cache::RequestCachePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CachePolicy", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ConnectionGroupName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ConnectionGroupName", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_Credentials(
        &mut self,
        value: *mut crate::System::Net::ICredentials,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Credentials", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Method(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Method", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_Proxy(
        &mut self,
        value: *mut crate::System::Net::IWebProxy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Proxy", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+FtpWebRequest")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FtpWebRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+FtpWebRequest+RequestStage")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtpWebRequest_RequestStage {
    CheckForError = 0i32,
    ReadReady = 3i32,
    ReleaseConnection = 4i32,
    RequestStarted = 1i32,
    WriteReady = 2i32,
}
#[cfg(feature = "System+Net+FtpWebRequest+RequestStage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpWebRequest_RequestStage =>
    "System.Net"."FtpWebRequest/RequestStage"
);