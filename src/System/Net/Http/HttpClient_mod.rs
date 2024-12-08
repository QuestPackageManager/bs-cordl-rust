#[cfg(feature = "System+Net+Http+HttpClient")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpClient {
    __cordl_parent: crate::System::Net::Http::HttpMessageInvoker,
    pub base_address: *mut crate::System::Uri,
    pub cts: *mut crate::System::Threading::CancellationTokenSource,
    pub disposed: bool,
    pub headers: *mut crate::System::Net::Http::Headers::HttpRequestHeaders,
    pub buffer_size: i64,
    pub timeout: crate::System::TimeSpan,
}
#[cfg(feature = "System+Net+Http+HttpClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::HttpClient =>
    "System.Net.Http"."HttpClient"
);
#[cfg(feature = "System+Net+Http+HttpClient")]
impl std::ops::Deref for crate::System::Net::Http::HttpClient {
    type Target = crate::System::Net::Http::HttpMessageInvoker;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpClient")]
impl std::ops::DerefMut for crate::System::Net::Http::HttpClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpClient")]
impl crate::System::Net::Http::HttpClient {
    #[cfg(feature = "System+Net+Http+HttpClient+_GetStringAsync_d__53")]
    pub type _GetStringAsync_d__53 = crate::System::Net::Http::HttpClient__GetStringAsync_d__53;
    #[cfg(feature = "System+Net+Http+HttpClient+_SendAsyncWorker_d__47")]
    pub type _SendAsyncWorker_d__47 = crate::System::Net::Http::HttpClient__SendAsyncWorker_d__47;
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
    pub fn _ctor_HttpMessageHandler__cordl_bool1(
        &mut self,
        handler: *mut crate::System::Net::Http::HttpMessageHandler,
        disposeHandler: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handler, disposeHandler))?;
        Ok(__cordl_ret)
    }
    pub fn SendAsync_HttpCompletionOption0(
        &mut self,
        request: *mut crate::System::Net::Http::HttpRequestMessage,
        completionOption: crate::System::Net::Http::HttpCompletionOption,
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
        > = __cordl_object.invoke("SendAsync", (request, completionOption))?;
        Ok(__cordl_ret)
    }
    pub fn SendAsync_CancellationToken1(
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
    pub fn SendAsync_HttpCompletionOption_CancellationToken2(
        &mut self,
        request: *mut crate::System::Net::Http::HttpRequestMessage,
        completionOption: crate::System::Net::Http::HttpCompletionOption,
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
        > = __cordl_object
            .invoke("SendAsync", (request, completionOption, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn set_Timeout(
        &mut self,
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Timeout", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringAsync(
        &mut self,
        requestUri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetStringAsync", (requestUri))?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxResponseContentBufferSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_MaxResponseContentBufferSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAsync(
        &mut self,
        requestUri: *mut crate::System::Uri,
        completionOption: crate::System::Net::Http::HttpCompletionOption,
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
        > = __cordl_object.invoke("GetAsync", (requestUri, completionOption))?;
        Ok(__cordl_ret)
    }
    pub fn SendAsyncWorker(
        &mut self,
        request: *mut crate::System::Net::Http::HttpRequestMessage,
        completionOption: crate::System::Net::Http::HttpCompletionOption,
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
        > = __cordl_object
            .invoke("SendAsyncWorker", (request, completionOption, cancellationToken))?;
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
    pub fn __n__0(
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
        > = __cordl_object.invoke("<>n__0", (request, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_HttpMessageHandler__cordl_bool1(
        handler: *mut crate::System::Net::Http::HttpMessageHandler,
        disposeHandler: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handler, disposeHandler))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+Http+HttpClient")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::HttpClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
