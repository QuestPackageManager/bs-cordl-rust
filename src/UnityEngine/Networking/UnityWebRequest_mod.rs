#[cfg(feature = "UnityEngine+Networking+UnityWebRequest+Result")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnityWebRequest_Result {
    ConnectionError = 2i32,
    DataProcessingError = 4i32,
    InProgress = 0i32,
    ProtocolError = 3i32,
    Success = 1i32,
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest+Result")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Networking::UnityWebRequest_Result
    => "UnityEngine.Networking"."UnityWebRequest/Result"
);
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequest {
    __cordl_parent: crate::System::Object,
    pub m_Ptr: crate::System::IntPtr,
    pub m_DownloadHandler: *mut crate::UnityEngine::Networking::DownloadHandler,
    pub m_UploadHandler: *mut crate::UnityEngine::Networking::UploadHandler,
    pub m_CertificateHandler: *mut crate::UnityEngine::Networking::CertificateHandler,
    pub m_Uri: *mut crate::System::Uri,
    pub _disposeCertificateHandlerOnDispose_k__BackingField: bool,
    pub _disposeDownloadHandlerOnDispose_k__BackingField: bool,
    pub _disposeUploadHandlerOnDispose_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Networking::UnityWebRequest =>
    "UnityEngine.Networking"."UnityWebRequest"
);
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest")]
impl std::ops::Deref for crate::UnityEngine::Networking::UnityWebRequest {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest")]
impl std::ops::DerefMut for crate::UnityEngine::Networking::UnityWebRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest")]
impl crate::UnityEngine::Networking::UnityWebRequest {
    pub const kHttpVerbCREATE: &'static str = "CREATE";
    pub const kHttpVerbDELETE: &'static str = "DELETE";
    pub const kHttpVerbGET: &'static str = "GET";
    pub const kHttpVerbHEAD: &'static str = "HEAD";
    pub const kHttpVerbPOST: &'static str = "POST";
    pub const kHttpVerbPUT: &'static str = "PUT";
    #[cfg(feature = "UnityEngine+Networking+UnityWebRequest+Result")]
    pub type Result = crate::UnityEngine::Networking::UnityWebRequest_Result;
    #[cfg(feature = "UnityEngine+Networking+UnityWebRequest+UnityWebRequestMethod")]
    pub type UnityWebRequestMethod = crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestMethod;
    #[cfg(feature = "UnityEngine+Networking+UnityWebRequest+UnityWebRequestError")]
    pub type UnityWebRequestError = crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError;
    pub fn SetUrl(
        &mut self,
        url: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError = __cordl_object
            .invoke("SetUrl", (url))?;
        Ok(__cordl_ret)
    }
    pub fn DisposeHandlers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisposeHandlers", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_method(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_method", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_disposeCertificateHandlerOnDispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_disposeCertificateHandlerOnDispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDone(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDone", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTimeoutMsec(
        &mut self,
        timeout: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError = __cordl_object
            .invoke("SetTimeoutMsec", (timeout))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_uploadHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Networking::UploadHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Networking::UploadHandler = __cordl_object
            .invoke("get_uploadHandler", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetCustomMethod(
        &mut self,
        customMethodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError = __cordl_object
            .invoke("SetCustomMethod", (customMethodName))?;
        Ok(__cordl_ret)
    }
    pub fn InternalDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCustomMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCustomMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalSetDefaults(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalSetDefaults", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_downloadedBytes(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_downloadedBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetRedirectLimitFromScripting(
        &mut self,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRedirectLimitFromScripting", (limit))?;
        Ok(__cordl_ret)
    }
    pub fn set_uri(
        &mut self,
        value: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uri", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_redirectLimit(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_redirectLimit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_disposeUploadHandlerOnDispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_disposeUploadHandlerOnDispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendWebRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation = __cordl_object
            .invoke("SendWebRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetMethod(
        &mut self,
        methodType: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestMethod,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError = __cordl_object
            .invoke("SetMethod", (methodType))?;
        Ok(__cordl_ret)
    }
    pub fn GetError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError = __cordl_object
            .invoke("GetError", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetResponseHeader(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetResponseHeader", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_disposeDownloadHandlerOnDispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_disposeDownloadHandlerOnDispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_disposeDownloadHandlerOnDispose(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposeDownloadHandlerOnDispose", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_url(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_url", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetDownloadHandler(
        &mut self,
        dh: *mut crate::UnityEngine::Networking::DownloadHandler,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError = __cordl_object
            .invoke("SetDownloadHandler", (dh))?;
        Ok(__cordl_ret)
    }
    pub fn get_downloadHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Networking::DownloadHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Networking::DownloadHandler = __cordl_object
            .invoke("get_downloadHandler", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_timeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_timeout", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        url: *mut crate::System::String,
        method: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, method))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_DownloadHandler_UploadHandler1(
        &mut self,
        url: *mut crate::System::String,
        method: *mut crate::System::String,
        downloadHandler: *mut crate::UnityEngine::Networking::DownloadHandler,
        uploadHandler: *mut crate::UnityEngine::Networking::UploadHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, method, downloadHandler, uploadHandler))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Uri_DownloadHandler_UploadHandler2(
        &mut self,
        uri: *mut crate::System::Uri,
        method: *mut crate::System::String,
        downloadHandler: *mut crate::UnityEngine::Networking::DownloadHandler,
        uploadHandler: *mut crate::UnityEngine::Networking::UploadHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uri, method, downloadHandler, uploadHandler))?;
        Ok(__cordl_ret)
    }
    pub fn SetUploadHandler(
        &mut self,
        uh: *mut crate::UnityEngine::Networking::UploadHandler,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError = __cordl_object
            .invoke("SetUploadHandler", (uh))?;
        Ok(__cordl_ret)
    }
    pub fn InternalSetMethod(
        &mut self,
        methodType: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalSetMethod", (methodType))?;
        Ok(__cordl_ret)
    }
    pub fn get_method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_method", ())?;
        Ok(__cordl_ret)
    }
    pub fn BeginWebRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation = __cordl_object
            .invoke("BeginWebRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalSetCustomMethod(
        &mut self,
        customMethodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalSetCustomMethod", (customMethodName))?;
        Ok(__cordl_ret)
    }
    pub fn set_disposeCertificateHandlerOnDispose(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposeCertificateHandlerOnDispose", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_error(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_error", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_url(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_url", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestMethod = __cordl_object
            .invoke("GetMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_responseCode(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_responseCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_disposeUploadHandlerOnDispose(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposeUploadHandlerOnDispose", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isModifiable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isModifiable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_Result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_Result = __cordl_object
            .invoke("get_result", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_certificateHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Networking::CertificateHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Networking::CertificateHandler = __cordl_object
            .invoke("get_certificateHandler", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUrl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetUrl", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_downloadHandler(
        &mut self,
        value: *mut crate::UnityEngine::Networking::DownloadHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_downloadHandler", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_certificateHandler(
        &mut self,
        value: *mut crate::UnityEngine::Networking::CertificateHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_certificateHandler", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalSetUrl(
        &mut self,
        url: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalSetUrl", (url))?;
        Ok(__cordl_ret)
    }
    pub fn SetCertificateHandler(
        &mut self,
        ch: *mut crate::UnityEngine::Networking::CertificateHandler,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError = __cordl_object
            .invoke("SetCertificateHandler", (ch))?;
        Ok(__cordl_ret)
    }
    pub fn set_uploadHandler(
        &mut self,
        value: *mut crate::UnityEngine::Networking::UploadHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uploadHandler", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        url: *mut crate::System::String,
        method: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, method))?;
        Ok(__cordl_object)
    }
    pub fn New_String_DownloadHandler_UploadHandler1(
        url: *mut crate::System::String,
        method: *mut crate::System::String,
        downloadHandler: *mut crate::UnityEngine::Networking::DownloadHandler,
        uploadHandler: *mut crate::UnityEngine::Networking::UploadHandler,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, method, downloadHandler, uploadHandler))?;
        Ok(__cordl_object)
    }
    pub fn New_Uri_DownloadHandler_UploadHandler2(
        uri: *mut crate::System::Uri,
        method: *mut crate::System::String,
        downloadHandler: *mut crate::UnityEngine::Networking::DownloadHandler,
        uploadHandler: *mut crate::UnityEngine::Networking::UploadHandler,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uri, method, downloadHandler, uploadHandler))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::UnityWebRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest+UnityWebRequestError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnityWebRequest_UnityWebRequestError {
    Aborted = 17i32,
    AccessDenied = 9i32,
    AlreadySent = 35i32,
    CannotConnectToHost = 8i32,
    CannotModifyRequest = 31i32,
    CannotOverrideSystemHeaders = 34i32,
    CannotResolveHost = 7i32,
    CannotResolveProxy = 6i32,
    DataProcessingError = 39i32,
    FailedToReceiveData = 22i32,
    FailedToSendData = 21i32,
    GenericHttpError = 10i32,
    HTTPPostError = 15i32,
    HeaderNameContainsInvalidCharacters = 32i32,
    HeaderValueContainsInvalidCharacters = 33i32,
    InsecureConnectionNotAllowed = 40i32,
    InvalidMethod = 36i32,
    InvalidRedirect = 30i32,
    LoginFailed = 27i32,
    MalformattedUrl = 5i32,
    NoInternetConnection = 38i32,
    NotImplemented = 37i32,
    OK = 0i32,
    OKCached = 1i32,
    OutOfMemory = 13i32,
    ReadError = 12i32,
    ReceivedNoData = 19i32,
    RedirectLimitInvalid = 29i32,
    SDKError = 3i32,
    SSLCACertError = 25i32,
    SSLCannotConnect = 16i32,
    SSLCertificateError = 23i32,
    SSLCipherNotAvailable = 24i32,
    SSLNotSupported = 20i32,
    SSLShutdownFailed = 28i32,
    Timeout = 14i32,
    TooManyRedirects = 18i32,
    Unknown = 2i32,
    UnrecognizedContentEncoding = 26i32,
    UnsupportedProtocol = 4i32,
    WriteError = 11i32,
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest+UnityWebRequestError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::UnityWebRequest_UnityWebRequestError =>
    "UnityEngine.Networking"."UnityWebRequest/UnityWebRequestError"
);
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest+UnityWebRequestMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnityWebRequest_UnityWebRequestMethod {
    Custom = 4i32,
    Get = 0i32,
    Head = 3i32,
    Post = 1i32,
    Put = 2i32,
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequest+UnityWebRequestMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::UnityWebRequest_UnityWebRequestMethod =>
    "UnityEngine.Networking"."UnityWebRequest/UnityWebRequestMethod"
);
