#[cfg(feature = "System+Net+WebRequestStream")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestStream {
    __cordl_parent: crate::System::Net::WebConnectionStream,
    pub writeBuffer: *mut crate::System::IO::MemoryStream,
    pub requestWritten: bool,
    pub allowBuffering: bool,
    pub sendChunked: bool,
    pub pendingWrite: *mut crate::System::Net::WebCompletionSource,
    pub totalWritten: i64,
    pub headers: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub headersSent: bool,
    pub completeRequestWritten: i32,
    pub chunkTrailerWritten: i32,
    pub _InnerStream_k__BackingField: *mut crate::System::IO::Stream,
    pub _KeepAlive_k__BackingField: bool,
}
#[cfg(feature = "System+Net+WebRequestStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebRequestStream => "System.Net"
    ."WebRequestStream"
);
#[cfg(feature = "System+Net+WebRequestStream")]
impl std::ops::Deref for crate::System::Net::WebRequestStream {
    type Target = crate::System::Net::WebConnectionStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebRequestStream")]
impl std::ops::DerefMut for crate::System::Net::WebRequestStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebRequestStream")]
impl crate::System::Net::WebRequestStream {
    #[cfg(feature = "System+Net+WebRequestStream+_FinishWriting_d__31")]
    pub type _FinishWriting_d__31 = crate::System::Net::WebRequestStream__FinishWriting_d__31;
    #[cfg(feature = "System+Net+WebRequestStream+_Initialize_d__36")]
    pub type _Initialize_d__36 = crate::System::Net::WebRequestStream__Initialize_d__36;
    #[cfg(feature = "System+Net+WebRequestStream+_ProcessWrite_d__34")]
    pub type _ProcessWrite_d__34 = crate::System::Net::WebRequestStream__ProcessWrite_d__34;
    #[cfg(feature = "System+Net+WebRequestStream+_SetHeadersAsync_d__37")]
    pub type _SetHeadersAsync_d__37 = crate::System::Net::WebRequestStream__SetHeadersAsync_d__37;
    #[cfg(feature = "System+Net+WebRequestStream+_WriteAsyncInner_d__33")]
    pub type _WriteAsyncInner_d__33 = crate::System::Net::WebRequestStream__WriteAsyncInner_d__33;
    #[cfg(feature = "System+Net+WebRequestStream+_WriteChunkTrailer_d__40")]
    pub type _WriteChunkTrailer_d__40 = crate::System::Net::WebRequestStream__WriteChunkTrailer_d__40;
    #[cfg(feature = "System+Net+WebRequestStream+_WriteChunkTrailer_inner_d__39")]
    pub type _WriteChunkTrailer_inner_d__39 = crate::System::Net::WebRequestStream__WriteChunkTrailer_inner_d__39;
    #[cfg(feature = "System+Net+WebRequestStream+_WriteRequestAsync_d__38")]
    pub type _WriteRequestAsync_d__38 = crate::System::Net::WebRequestStream__WriteRequestAsync_d__38;
    pub fn CheckWriteOverflow(
        &mut self,
        contentLength: i64,
        totalWritten: i64,
        _cordl_size: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckWriteOverflow", (contentLength, totalWritten, _cordl_size))?;
        Ok(__cordl_ret)
    }
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
    pub fn FinishWriting(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("FinishWriting", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn GetWriteBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::BufferOffsetSize> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::BufferOffsetSize = __cordl_object
            .invoke("GetWriteBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("Initialize", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn KillBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KillBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        connection: *mut crate::System::Net::WebConnection,
        operation: *mut crate::System::Net::WebOperation,
        stream: *mut crate::System::IO::Stream,
        tunnel: *mut crate::System::Net::WebConnectionTunnel,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connection, operation, stream, tunnel))?;
        Ok(__cordl_object)
    }
    pub fn ProcessWrite(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("ProcessWrite", (buffer, offset, _cordl_size, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsync(
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
            .invoke("ReadAsync", (buffer, offset, _cordl_size, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn SetHeadersAsync(
        &mut self,
        setInternalLength: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("SetHeadersAsync", (setInternalLength, cancellationToken))?;
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
    pub fn WriteAsyncInner(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        completion: *mut crate::System::Net::WebCompletionSource,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "WriteAsyncInner",
                (buffer, offset, _cordl_size, completion, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteChunkTrailer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteChunkTrailer", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteChunkTrailer_inner(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteChunkTrailer_inner", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn WriteRequestAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteRequestAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        connection: *mut crate::System::Net::WebConnection,
        operation: *mut crate::System::Net::WebOperation,
        stream: *mut crate::System::IO::Stream,
        tunnel: *mut crate::System::Net::WebConnectionTunnel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connection, operation, stream, tunnel))?;
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
    pub fn get_HasWriteBuffer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasWriteBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InnerStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("get_InnerStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeepAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_KeepAlive", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WriteBufferLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WriteBufferLength", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+WebRequestStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebRequestStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
