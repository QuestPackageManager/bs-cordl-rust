#[cfg(feature = "System+Net+WebResponseStream")]
#[repr(C)]
#[derive(Debug)]
pub struct WebResponseStream {
    __cordl_parent: crate::System::Net::WebConnectionStream,
    pub innerStream: *mut crate::System::Net::WebReadStream,
    pub nextReadCalled: bool,
    pub bufferedEntireContent: bool,
    pub pendingRead: *mut crate::System::Net::WebCompletionSource,
    pub locker: *mut crate::System::Object,
    pub nestedRead: i32,
    pub read_eof: bool,
    pub _RequestStream_k__BackingField: *mut crate::System::Net::WebRequestStream,
    pub _Headers_k__BackingField: *mut crate::System::Net::WebHeaderCollection,
    pub _StatusCode_k__BackingField: crate::System::Net::HttpStatusCode,
    pub _StatusDescription_k__BackingField: *mut crate::System::String,
    pub _Version_k__BackingField: *mut crate::System::Version,
    pub _KeepAlive_k__BackingField: bool,
    pub _ChunkedRead_k__BackingField: bool,
}
#[cfg(feature = "System+Net+WebResponseStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebResponseStream => "System.Net"
    ."WebResponseStream"
);
#[cfg(feature = "System+Net+WebResponseStream")]
impl std::ops::Deref for crate::System::Net::WebResponseStream {
    type Target = crate::System::Net::WebConnectionStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebResponseStream")]
impl std::ops::DerefMut for crate::System::Net::WebResponseStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebResponseStream")]
impl crate::System::Net::WebResponseStream {
    #[cfg(feature = "System+Net+WebResponseStream+_InitReadAsync_d__52")]
    pub type _InitReadAsync_d__52 = crate::System::Net::WebResponseStream__InitReadAsync_d__52;
    #[cfg(feature = "System+Net+WebResponseStream+_ReadAllAsyncInner_d__47")]
    pub type _ReadAllAsyncInner_d__47 = crate::System::Net::WebResponseStream__ReadAllAsyncInner_d__47;
    #[cfg(feature = "System+Net+WebResponseStream+_ReadAllAsync_d__48")]
    pub type _ReadAllAsync_d__48 = crate::System::Net::WebResponseStream__ReadAllAsync_d__48;
    #[cfg(feature = "System+Net+WebResponseStream+_ReadAsync_d__40")]
    pub type _ReadAsync_d__40 = crate::System::Net::WebResponseStream__ReadAsync_d__40;
    #[cfg(feature = "System+Net+WebResponseStream+__c__DisplayClass41_0")]
    pub type __c__DisplayClass41_0 = crate::System::Net::WebResponseStream___c__DisplayClass41_0;
    pub fn Close_internal(
        &mut self,
        disposed: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close_internal", (disposed))?;
        Ok(__cordl_ret)
    }
    pub fn GetReadException(
        &mut self,
        status: crate::System::Net::WebExceptionStatus,
        error: *mut crate::System::Exception,
        _cordl_where: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebException> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebException = __cordl_object
            .invoke("GetReadException", (status, error, _cordl_where))?;
        Ok(__cordl_ret)
    }
    pub fn GetResponse(
        &mut self,
        buffer: *mut crate::System::Net::BufferOffsetSize,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
        state: quest_hook::libil2cpp::ByRefMut<crate::System::Net::ReadState>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetResponse", (buffer, pos, state))?;
        Ok(__cordl_ret)
    }
    pub fn InitReadAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("InitReadAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        buffer: *mut crate::System::Net::BufferOffsetSize,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (buffer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        request: *mut crate::System::Net::WebRequestStream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (request))?;
        Ok(__cordl_object)
    }
    pub fn ProcessRead(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<i32> = __cordl_object
            .invoke("ProcessRead", (buffer, offset, _cordl_size, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAllAsync(
        &mut self,
        resending: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("ReadAllAsync", (resending, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAllAsyncInner(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ReadAllAsyncInner", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsync(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<i32> = __cordl_object
            .invoke("ReadAsync", (buffer, offset, count, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn TryReadFromBufferedContent(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        count: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryReadFromBufferedContent", (buffer, offset, count, result))?;
        Ok(__cordl_ret)
    }
    pub fn WriteAsync(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteAsync", (buffer, offset, count, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        request: *mut crate::System::Net::WebRequestStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (request))?;
        Ok(__cordl_ret)
    }
    pub fn get_CanRead(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanRead", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CanWrite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanWrite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChunkedRead(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ChunkedRead", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ExpectContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ExpectContent", ())?;
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
    pub fn get_KeepAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_KeepAlive", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RequestStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebRequestStream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebRequestStream = __cordl_object
            .invoke("get_RequestStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StatusCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::HttpStatusCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::HttpStatusCode = __cordl_object
            .invoke("get_StatusCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StatusDescription(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_StatusDescription", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Version> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Version = __cordl_object
            .invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ChunkedRead(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ChunkedRead", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Headers(
        &mut self,
        value: *mut crate::System::Net::WebHeaderCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Headers", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_StatusCode(
        &mut self,
        value: crate::System::Net::HttpStatusCode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StatusCode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_StatusDescription(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StatusDescription", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Version(
        &mut self,
        value: *mut crate::System::Version,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Version", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+WebResponseStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebResponseStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
